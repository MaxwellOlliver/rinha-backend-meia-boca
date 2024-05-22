use crate::auth::Claims;
use actix_service::Service;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use futures::future::{ok, Ready};
use futures::FutureExt;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::rc::Rc;

pub struct AuthMiddleware<S> {
    service: Rc<S>,
    secret_key: String,
}

impl<S> AuthMiddleware<S> {
    pub fn new(service: S, secret_key: String) -> Self {
        AuthMiddleware {
            service: Rc::new(service),
            secret_key,
        }
    }
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = futures::future::Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..]; // Remove "Bearer " prefix
                    if let Ok(token_data) = decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(self.secret_key.as_ref()),
                        &Validation::default(),
                    ) {
                        req.extensions_mut().insert(token_data.claims);
                        let fut = self.service.call(req);
                        return futures::future::Either::Left(fut);
                    }
                }
            }
        }

        futures::future::Either::Right(ok(
            req.into_response(actix_web::HttpResponse::Unauthorized().finish().into_body())
        ))
    }
}
