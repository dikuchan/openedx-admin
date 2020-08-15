use crate::{
    config::db::Pool,
    util::message,
    model::{
        user::LoginDTO,
        response::ResponseBody,
    },
    service::account,
};
use actix_web::{web, HttpRequest, HttpResponse, Result};

/*
  POST /auth/login
 */
pub async fn login(login_dto: web::Json<LoginDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match account::login(login_dto.0, &pool) {
        Ok(response) => Ok(HttpResponse::Ok().json(
            ResponseBody::new(message::LOGIN_SUCCESS, response)
        )),
        Err(error) => Ok(error.response())
    }
}

/*
  POST /auth/logout
 */
pub async fn logout(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse> {
    if let Some(header) = req.headers().get(message::AUTHORIZATION) {
        account::logout(header, &pool);
        Ok(
            HttpResponse::Ok().json(ResponseBody::new(message::LOGOUT_SUCCESS, message::EMPTY))
        )
    } else {
        Ok(
            HttpResponse::Ok().json(ResponseBody::new(message::TOKEN_MISSING, message::EMPTY))
        )
    }
}
