use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

pub trait Event: Any + Send + Sync + Clone {}

impl<T> Event for T where T: Any + Send + Sync + Clone {}

type AsyncCallback<T> = Box<dyn Fn(T) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + 'static>;

pub struct AsyncEventBus {
    subscribers: Arc<Mutex<Vec<mpsc::Sender<Box<dyn Any + Send>>>>>,
}

impl AsyncEventBus {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn publish<T: Event>(&self, event: T) {
        let subscribers = self.subscribers.lock().unwrap();
        for subscriber in subscribers.iter() {
            let _ = subscriber.send(Box::new(event.clone())).await;
        }
    }

    pub async fn subscribe<T: Event>(&self, subscriber: AsyncCallback<T>) {
        let (tx, mut rx) = mpsc::channel::<Box<dyn Any + Send>>(32);
        self.subscribers.lock().unwrap().push(tx);

        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                if let Some(event) = event.downcast_ref::<T>() {
                    let cloned_event = event.clone();
                    subscriber(cloned_event).await;
                }
            }
        });
    }
}

pub struct SyncEventBus {
    subscribers: Arc<Mutex<Vec<AsyncCallback<Box<dyn Any + Send + Sync>>>>>,
}

impl SyncEventBus {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn publish<T: Event>(&self, event: T) {
        let subscribers = self.subscribers.lock().unwrap();
        let mut futures = Vec::new();
        for subscriber in subscribers.iter() {
            futures.push(subscriber(Box::new(event.clone())));
        }

        futures::future::join_all(futures).await;
    }

    pub fn subscribe<T: Event>(&self, subscriber: AsyncCallback<T>) {
        let mut subs = self.subscribers.lock().unwrap();
        // 适配 subscriber 的输入类型
        let handler = Box::new(move |event: Box<dyn Any + Send + Sync>| {
            let event = event.downcast_ref::<T>().unwrap().clone();
            subscriber(event)
        }) as AsyncCallback<Box<dyn Any + Send + Sync>>;
        subs.push(handler);
    }
}

#[cfg(test)]
mod tests {
    use tokio::time::sleep;

    use super::*;

    #[derive(Debug, Clone)]
    pub struct UserRegisteredEvent {
        pub user_id: String,
        pub username: String,
        pub email: String,
    }

    #[derive(Debug, Clone)]
    pub struct OrderPlacedEvent {
        pub order_id: String,
        pub amount: f32,
    }

    #[tokio::test]
    async fn it_works() {
        // 创建一个事件总线实例
        let event_bus = Arc::new(AsyncEventBus::new());

        // 订阅 UserRegisteredEvent
        event_bus
            .subscribe(Box::new(move |event: UserRegisteredEvent| {
                Box::pin(async move {
                    // 异步逻辑
                })
            }))
            .await;

        // 发布一个 UserRegisteredEvent
        let user_event = UserRegisteredEvent {
            user_id: "123".to_string(),
            username: "john_doe".to_string(),
            email: "john@example.com".to_string(),
        };

        event_bus.publish(user_event).await;

        // 发布一个 OrderPlacedEvent
        let order_event = OrderPlacedEvent {
            order_id: "456".to_string(),
            amount: 99.99,
        };

        event_bus.publish(order_event).await;

        sleep(std::time::Duration::from_secs(1)).await;
    }
}
