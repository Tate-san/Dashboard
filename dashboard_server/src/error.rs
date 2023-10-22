use std::io;

use actix_web::{ResponseError, http::StatusCode, HttpResponse};

pub type ServerResponse = Result<HttpResponse, ServerError>;

pub enum DatabaseError {
    NotFound,
    DuplicateKey,
    InternalError(sqlx::Error)
}

impl From<sqlx::Error> for DatabaseError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => DatabaseError::NotFound,
            _ => {
                let message = error.to_string();
                if message.contains("duplicate key") {
                    return Self::DuplicateKey;
                }

                Self::InternalError(error)
            } 
        }
        
    }
}

#[derive(Debug)]
pub enum ServerErrorType {
    IoError,
    SqlxError,
    ArgonError,
}

#[derive(Debug)]
pub struct ServerError {
    pub message: Option<String>,
    pub error: ServerErrorType,
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
            error: ServerErrorType::SqlxError
        }
    }
}

impl From<sqlx::Error> for ServerError {
    fn from(error: sqlx::Error) -> Self {
        Self {
            message: Some(error.to_string()),
            error: ServerErrorType::IoError
        }
    }
}

impl From<argon2::Error> for ServerError {
    fn from(error: argon2::Error) -> Self {
        Self {
            message: Some(error.to_string()),
            error: ServerErrorType::IoError
        }
    }
}

impl From<DatabaseError> for ServerError {
    fn from(error: DatabaseError) -> Self {
        let message = match error {
            DatabaseError::DuplicateKey => "Duplicate key".to_string(),
            DatabaseError::NotFound => "Not found".to_string(),
            DatabaseError::InternalError(err) => err.to_string(),
        };

        Self {
            message: Some(message),
            error: ServerErrorType::SqlxError
        }
    }
}

impl ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match self.error {
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