use std::io;

use actix_web::{ResponseError, http::StatusCode, HttpResponse};

pub type ServerResponse = Result<HttpResponse, ServerError>;

#[derive(Debug)]
pub enum ServerErrorType{
    IoError,
    SqlxError,
    ArgonError,
}

#[derive(Debug)]
pub struct ServerError {
    pub message: Option<String>,
    pub err_type: ServerErrorType,
}

impl ServerError {
    pub fn message(&self) -> String {
        match &self.message {
            Some(c) => c.clone(),
            None => String::from(""),
        }
    }
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<io::Error> for ServerError {
    fn from(error: io::Error) -> Self {
        Self {
            message: Some(error.to_string()),
            err_type: ServerErrorType::SqlxError
        }
    }
}

impl From<sqlx::Error> for ServerError {
    fn from(error: sqlx::Error) -> Self {
        Self {
            message: Some(error.to_string()),
            err_type: ServerErrorType::IoError
        }
    }
}

impl From<argon2::Error> for ServerError {
    fn from(error: argon2::Error) -> Self {
        Self {
            message: Some(error.to_string()),
            err_type: ServerErrorType::IoError
        }
    }
}

impl ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match self.err_type {
            ServerErrorType::IoError => StatusCode::INTERNAL_SERVER_ERROR,
            ServerErrorType::SqlxError => StatusCode::INTERNAL_SERVER_ERROR,
            ServerErrorType::ArgonError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse { 
        HttpResponse::build(self.status_code())
            .json(serde_json::json!({
                "status": "error",
                "message": format!("{:?}", self.message())
            }))
    }
}