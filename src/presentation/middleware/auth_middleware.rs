use std::pin::Pin;
use std::rc::Rc;

use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use actix_web::error::ErrorUnauthorized;
use actix_web::HttpMessage;
use actix_web::web::Data;
use futures::future::{ok, Ready};
use futures::Future;
use futures::task::{Context, Poll};

use crate::config::auth::decode_jwt;


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
        // Extract the Authorization header and configuration
        let auth_header = match req.headers().get("Authorization") {
            Some(header) => header,
            None => return Box::pin(async { Err(ErrorUnauthorized("Unauthorized")) }),
        };

        let config = match req
            .app_data::<Data<crate::config::settings::Config>>()
            .cloned()
        {
            Some(cfg) => cfg,
            None => return Box::pin(async { Err(ErrorUnauthorized("Unauthorized")) }),
        };

        // Convert Authorization header to str and check if it starts with "Bearer "
        let auth_header = match auth_header.to_str() {
            Ok(header) if header.starts_with("Bearer ") => &header[7..],
            _ => return Box::pin(async { Err(ErrorUnauthorized("Unauthorized")) }),
        };

        // Decode the JWT token
        let claims = match decode_jwt(auth_header, &config.auth.jwt_secret) {
            Ok(claims) => claims,
            Err(_) => return Box::pin(async { Err(ErrorUnauthorized("Unauthorized")) }),
        };

        // Insert claims into request extensions and call the service
        req.extensions_mut().insert(claims);
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
