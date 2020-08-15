use crate::{
    config::{app, db::Pool},
    model::response::ResponseBody,
    util::{message, token},
};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use actix_service::{Service, Transform};
use actix_web::{
    http::{Method, header::{HeaderName, HeaderValue}},
    dev::{ServiceRequest, ServiceResponse}, Error, HttpResponse,
};
use futures::{
    future::{ok, Ready},
    Future,
};

/*
  Middleware implementation is taken from the Actix documentation.
 */

pub struct Authentication;

impl<S, B> Transform<S> for Authentication
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticationMiddleware<S>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    /*
      Basically, this function contains the authentication logic.
     */
    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let mut authorized = false;
        let headers = req.headers_mut();

        headers.append(
            HeaderName::from_static("content-length"),
            HeaderValue::from_static("true"),
        );

        if Method::OPTIONS == *req.method() {
            authorized = true
        }
        for &route in app::IGNORED_ROUTES.iter() {
            if req.path().starts_with(route) { authorized = true; }
        }
        if !authorized {
            // TODO: Rewrite.
            if let Some(pool) = req.app_data::<Pool>() {
                if let Some(header) = req.headers_mut().get(message::AUTHORIZATION) {
                    if let Ok(auth_string) = header.to_str() {
                        if auth_string.starts_with("Bearer") || auth_string.starts_with("bearer") {
                            let token = auth_string[6..auth_string.len()].trim();
                            if let Ok(token_data) = token::decode(token) {
                                if token::verify(&token_data, &pool).is_ok() {
                                    authorized = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        if authorized {
            let fut = self.service.call(req);
            Box::pin(async move {
                let result = fut.await?;
                Ok(result)
            })
        } else {
            Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(ResponseBody::new(
                            message::TOKEN_INVALID,
                            message::EMPTY,
                        ))
                        .into_body(),
                ))
            })
        }
    }
}
