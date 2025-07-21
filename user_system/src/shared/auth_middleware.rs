use crate::shared::claims::Claims;
use actix_web::HttpMessage;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use casbin::CoreApi;
use casbin::Enforcer;
use futures::future::{ok, LocalBoxFuture, Ready};
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::Mutex;

pub struct AuthMiddleware {
    enforcer: Arc<Mutex<Enforcer>>,
}

impl AuthMiddleware {
    pub fn new(enforcer: Arc<Mutex<Enforcer>>) -> AuthMiddleware {
        AuthMiddleware { enforcer }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareImpl<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareImpl {
            service: Rc::new(service),
            enforcer: self.enforcer.clone(),
        })
    }
}

pub struct AuthMiddlewareImpl<S> {
    service: Rc<S>,
    enforcer: Arc<Mutex<Enforcer>>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareImpl<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = Rc::clone(&self.service);
        let enforcer = self.enforcer.clone();
        Box::pin(async move {
            // 1. 解析 token
            let token_opt = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.strip_prefix("Bearer "));

            let token = match token_opt {
                Some(t) => t,
                None => return Err(actix_web::error::ErrorUnauthorized("Missing token")),
            };

            let claims = match Claims::validate(token) {
                Ok(c) => c,
                Err(_) => return Err(actix_web::error::ErrorUnauthorized("Invalid token")),
            };

            // 2. 保存 Claims 到请求中
            req.extensions_mut().insert(claims.clone());

            // 开始验证权限
            // 如果是个人信息,直接放行
            if req.path().contains("/api/user_system/user_profile") {
                return srv.call(req).await;
            }

            // 3. 构造 Casbin 的三元组 (sub, obj, act)
            // 角色名称
            let subs = claims.rules;

            for sub in subs {
                let obj = req.path().to_string();
                let act = req.method().as_str().to_string().to_uppercase();
                let allowed = enforcer
                    .lock()
                    .await
                    .enforce((sub.clone(), obj.clone(), act.clone()))
                    .unwrap();

                if allowed {
                    return srv.call(req).await;
                }
            }

            Err(actix_web::error::ErrorForbidden(format!(
                "User {} not permitted",
                claims.username
            )))
        })
    }
}
