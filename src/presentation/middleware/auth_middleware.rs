use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use futures::future::{ok, Ready};
use futures::task::{Context, Poll};
use futures::Future;
use std::pin::Pin;
use std::rc::Rc;
use crate::config::auth::decode_jwt;
use actix_web::web::Data;
use actix_web::error::ErrorUnauthorized;
use actix_web::HttpMessage;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct AuthMiddlewareMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        let config = req.app_data::<Data<crate::config::settings::Config>>().cloned();

        if let (Some(auth_header), Some(config)) = (auth_header, config) {
            if let Ok(auth_header) = auth_header.to_str() {
                if auth_header.starts_with("Bearer ") {
                    let token = &auth_header[7..];
                    if let Ok(claims) = decode_jwt(token, &config.auth.jwt_secret) {
                        req.extensions_mut().insert(claims);
                        let fut = self.service.call(req);
                        return Box::pin(async move {
                            let res = fut.await?;
                            Ok(res)
                        });
                    }
                }
            }
        }

        Box::pin(async { Err(ErrorUnauthorized("Unauthorized")) })
    }
}
