use crate::handler::*;
use actix_web::web;

// Routes that don't require authorization to access.
pub const IGNORED_ROUTES: [&str; 2] = ["/hello", "/auth/login"];

pub fn services(config: &mut web::ServiceConfig) {
    config
        .service(
            web::resource("/hello").route(web::get().to(hello::hello))
        )
        .service(
            web::scope("/auth")
                .service(
                    web::resource("/login").route(web::post().to(account::login))
                )
                .service(
                    web::resource("/logout").route(web::post().to(account::logout))
                )
        );
}
