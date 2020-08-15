use crate::{
    config::db::Connection,
    model::user::User,
    schema::login_history::{self, dsl::*},
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Identifiable, Associations, Queryable)]
#[belongs_to(User)]
#[table_name = "login_history"]
pub struct LoginHistory {
    pub id: i32,
    pub user_id: i32,
    pub login_timestamp: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "login_history"]
pub struct LoginHistoryDTO {
    pub user_id: i32,
    pub login_timestamp: DateTime<Utc>,
}

impl LoginHistory {
    pub fn create(email: &str, connection: &Connection) -> Option<LoginHistoryDTO> {
        if let Ok(user) = User::find_by_email(email, connection) {
            let result = LoginHistoryDTO {
                user_id: user.id,
                login_timestamp: Utc::now(),
            };
            Some(result)
        } else {
            None
        }
    }

    pub fn save(record: LoginHistoryDTO, connection: &Connection) -> QueryResult<usize> {
        diesel::insert_into(login_history)
            .values(&record)
            .execute(connection)
    }
}
