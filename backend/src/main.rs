#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use actix_cors::Cors;
use actix_service::Service;
use actix_web::{
    http::header::*,
    middleware::Logger, App, HttpServer,
};
use futures::FutureExt;
use std::{io, env};

mod config;
mod error;
mod handler;
mod middleware;
mod model;
mod schema;
mod service;
mod util;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().expect("Failed to read environment file.");
    env_logger::init();

    let app_host = env::var("APP_HOST").unwrap_or("0.0.0.0".to_string());
    let app_port = env::var("APP_PORT").unwrap_or("8080".to_string());
    let app_url = format!("{}:{}", &app_host, &app_port);
    let database_url = env::var("DATABASE_URL").expect("Cannot fetch database URL.");

    let pool = config::db::migrate(&database_url);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::new()
                .send_wildcard()
                .allowed_header(CONTENT_TYPE)
                .allowed_headers(vec![ACCEPT, AUTHORIZATION])
                .allowed_methods(vec!["GET", "POST", "PUT", "OPTIONS"])
                .max_age(3600)
                .finish()
            )
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(middleware::authentication::Authentication)
            .wrap_fn(|request, service| {
                service.call(request).map(|res| res)
            })
            .configure(config::app::services)
    })
        .bind(&app_url)?
        .run()
        .await
}
