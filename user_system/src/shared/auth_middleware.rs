use crate::shared::claims::Claims;
use actix_web::HttpMessage;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct AuthMiddleware;

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
        })
    }
}

pub struct AuthMiddlewareImpl<S> {
    service: Rc<S>,
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

        Box::pin(async move {
            let token_opt = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.strip_prefix("Bearer "));

            if let Some(token) = token_opt {
                match Claims::validate(token) {
                    Ok(data) => {
                        // 把解析出的 Claims 存入 request extensions 中
                        req.extensions_mut().insert(data);
                        // let claims = req.extensions().get::<Claims>();
                        srv.call(req).await
                    }
                    Err(_) => Err(actix_web::error::ErrorForbidden("Invalid token")),
                }
            } else {
                Err(actix_web::error::ErrorForbidden(
                    "Missing or invalid Authorization header",
                ))
            }
        })
    }
}
