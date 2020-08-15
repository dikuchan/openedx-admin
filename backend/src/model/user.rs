use crate::{
    config::db::Connection,
    model::{login_history::LoginHistory, user_token::UserToken},
    schema::users::{self, dsl::*},
};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub login_session: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserDTO {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct LoginInfoDTO {
    pub email: String,
    pub login_session: String,
}

impl User {
    pub fn login(login: LoginDTO, connection: &Connection) -> Option<LoginInfoDTO> {
        if let Ok(user) = users.filter(email.eq(&login.email)).get_result::<User>(connection) {
            if user.password.is_empty() && login.password == user.password {
                if let Some(login_history) = LoginHistory::create(&user.email, connection) {
                    if LoginHistory::save(login_history, connection).is_err() {
                        return None;
                    }
                    let login_session_str = User::generate_login_session();
                    if User::update_login_session(&user.email, &login_session_str, connection) {
                        let login_info = LoginInfoDTO {
                            email: user.email,
                            login_session: login_session_str,
                        };
                        return Some(login_info);
                    }
                }
            }
        }

        None
    }

    pub fn logout(user_id: i32, connection: &Connection) {
        if let Ok(user) = users.find(user_id).get_result::<User>(connection) {
            Self::update_login_session(&user.email, "", connection);
        }
    }

    fn update_login_session(e: &str, login_session_str: &str, connection: &Connection) -> bool {
        if let Ok(user) = User::find_by_email(e, connection) {
            diesel::update(users.find(user.id))
                .set(login_session.eq(login_session_str))
                .execute(connection)
                .is_ok()
        } else {
            false
        }
    }

    fn generate_login_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    pub fn is_valid_login_session(token: &UserToken, connection: &Connection) -> bool {
        users
            .filter(email.eq(&token.user))
            .filter(login_session.eq(&token.login_session))
            .get_result::<User>(connection)
            .is_ok()
    }

    pub fn find_by_email(e: &str, connection: &Connection) -> QueryResult<User> {
        users.filter(email.eq(e)).get_result::<User>(connection)
    }
}
