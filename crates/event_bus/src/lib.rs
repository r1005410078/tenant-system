use std::{
    any::{Any, TypeId},
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;

pub trait Event: Any + Send + Sync + Clone {}
impl<T> Event for T where T: Any + Send + Sync + Clone {}

type AsyncCallback<T> = Box<dyn Fn(T) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

pub struct AsyncEventBus {
    subscribers: Arc<Mutex<HashMap<TypeId, Vec<AsyncCallbackBox>>>>,
}

type AsyncCallbackBox =
    Box<dyn Fn(Box<dyn Any + Send>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

impl AsyncEventBus {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn publish<T: Event>(&self, event: T) {
        let type_id = TypeId::of::<T>();
        let subscribers = self.subscribers.lock().unwrap();
        if let Some(callbacks) = subscribers.get(&type_id) {
            for callback in callbacks {
                let event_box = Box::new(event.clone()) as Box<dyn Any + Send>;
                callback(event_box).await;
            }
        }
    }

    pub fn subscribe<T: Event + 'static>(&self, callback: AsyncCallback<T>) {
        let type_id = TypeId::of::<T>();
        let mut subscribers = self.subscribers.lock().unwrap();
        let entry = subscribers.entry(type_id).or_insert_with(Vec::new);
        // 包装成统一的 Box<dyn Fn(Box<dyn Any + Send>)>
        let wrapper: AsyncCallbackBox = Box::new(move |event: Box<dyn Any + Send>| {
            let event = *event.downcast::<T>().unwrap();
            callback(event)
        });
        entry.push(wrapper);
    }
}

// 事件监听 trait
#[async_trait::async_trait]
pub trait EventListener<E: Event>: Send + Sync {
    async fn handle(&self, event: E);

    fn subscribe(self: Arc<Self>, bus: Arc<AsyncEventBus>)
    where
        Self: Sized + 'static,
        E: 'static,
    {
        let listener = self.clone();
        bus.subscribe::<E>(Box::new(move |event: E| {
            let listener = listener.clone();
            Box::pin(async move {
                listener.handle(event).await;
            })
        }));
    }
}

// ================== 测试 ==================
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tokio::time::{sleep, Duration};

    #[derive(Debug, Clone)]
    struct UserRegisteredEvent {
        pub username: String,
    }

    struct UserListener {
        pub count: Arc<AtomicUsize>,
    }

    #[async_trait::async_trait]
    impl EventListener<UserRegisteredEvent> for UserListener {
        async fn handle(&self, event: UserRegisteredEvent) {
            self.count.fetch_add(1, Ordering::SeqCst);

            println!("User registered: {}", event.username);
            sleep(Duration::from_millis(100)).await;
        }
    }

    #[tokio::test]
    async fn test_event_bus() {
        let bus = Arc::new(AsyncEventBus::new());
        let count = Arc::new(AtomicUsize::new(0));
        let listener = Arc::new(UserListener {
            count: count.clone(),
        });
        listener.subscribe(bus.clone());

        bus.publish(UserRegisteredEvent {
            username: "alice".to_string(),
        })
        .await;

        assert_eq!(count.load(Ordering::SeqCst), 1);
    }
}
