use crate::model::response::ResponseBody;
use actix_web::{HttpResponse, http::StatusCode};

pub struct ServiceError {
    pub code: StatusCode,
    pub body: ResponseBody<String>,
}

impl ServiceError {
    pub fn new(code: StatusCode, message: String) -> Self {
        Self {
            code,
            body: ResponseBody {
                message,
                data: String::new(),
            },
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::build(self.code).json(&self.body)
    }
}
