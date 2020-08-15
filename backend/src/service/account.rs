use crate::{
    config::db::Pool,
    error::ServiceError,
    model::{
        user::{User, LoginDTO},
        user_token::UserToken,
    },
    util::{message, token},
};
use actix_web::{
    http::{header::HeaderValue, StatusCode},
    web,
};

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

pub fn login(login: LoginDTO, pool: &web::Data<Pool>) -> Result<TokenBodyResponse, ServiceError> {
    match User::login(login, &pool.get().unwrap()) {
        Some(user) => {
            let token = json!({
                "token": UserToken::generate(user),
                "token_type": "bearer",
            });
            match serde_json::from_value(token) {
                Ok(response) => Ok(response),
                Err(_) => Err(
                    ServiceError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        message::LOGIN_FAILED.to_string(),
                    )
                )
            }
        }
        None => Err(
            ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                message::LOGIN_FAILED.to_string(),
            )
        )
    }
}

pub fn logout(header: &HeaderValue, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    if let Ok(auth_string) = header.to_str() {
        if auth_string.starts_with("bearer") {
            let token = auth_string[6..auth_string.len()].trim();
            if let Ok(token_data) = token::decode(token) {
                if let Ok(email) = token::verify(&token_data, pool) {
                    if let Ok(user) = User::find_by_email(&email, &pool.get().unwrap()) {
                        User::logout(user.id, &pool.get().unwrap());
                        return Ok(());
                    }
                }
            }
        }
    }

    Err(
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            message::LOGIN_FAILED.to_string(),
        )
    )
}
