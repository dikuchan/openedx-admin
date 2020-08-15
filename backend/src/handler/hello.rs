use actix_web::HttpResponse;

pub fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello!".to_string())
}
