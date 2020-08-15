use crate::model::user::LoginInfoDTO;
use chrono::Utc;
use jsonwebtoken::{
    EncodingKey,
    Header,
};

/*
  Use secret key for token encoding.
 */
pub static KEY: [u8; 16] = *include_bytes!("../secret.key");
static WEEK: i64 = 60 * 60 * 24 * 7;

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    // Issued at.
    pub iat: i64,
    // Expires at.
    pub exp: i64,
    pub user: String,
    pub login_session: String,
}

impl UserToken {
    pub fn generate(login: LoginInfoDTO) -> String {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // Into seconds.
        let payload = UserToken {
            iat: now,
            exp: now + WEEK,
            user: login.email,
            login_session: login.login_session,
        };

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(&KEY),
        ).unwrap()
    }
}
