pub mod user_model;
pub mod device_model;
pub mod system_model;

pub use user_model::*;
pub use device_model::*;
pub use system_model::*;
pub use crate::error::DatabaseError;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

mod prelude {
    pub use std::vec::Vec;
    pub use serde::{Deserialize, Serialize};
    pub use sqlx::{Pool, Postgres};
    pub use super::DatabaseResult;
    pub use sqlx::postgres::PgQueryResult;
    pub use crate::error::DatabaseError;
    pub use utoipa::ToSchema;
}

pub type DatabaseResult<A> = Result<A, DatabaseError>;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ErrorModel{
    pub name: String,
    pub message: String,
}

impl ErrorModel {
    pub fn new(name: &str, message: &str) -> Self {
        ErrorModel {
            name: name.to_string(),
            message: message.to_string()
        }
    }
}

pub enum ResponseError {
    UserLoggedIn,
    UserNotLoggedIn,
    UserLoginInvalid,
    UserAlreadyExists,
    UserPasswordCantBeEmpty,
    SystemAlreadyExists,
    SystemDoesntExist,
    SystemNotOwner,
    SystemAlreadyOwner,
    SystemAlreadyMember,
    DeviceAlreadyExists,
    SystemNotMember,
    DeviceNotOwner,
    DeviceDoesntExist,
    DeviceStructureDoesntExist,
    InternalError(String),
}

impl ResponseError {
    pub fn get_error(&self) -> ErrorModel {
        match self {
            Self::UserLoggedIn => ErrorModel::new("USER_LOGGED_IN", "Can't do this operation, already logged in"),
            Self::UserNotLoggedIn => ErrorModel::new("USER_NOT_LOGGED_IN", "You are not logged in"),
            Self::UserLoginInvalid => ErrorModel::new("USER_LOGIN_INVALID", "Invalid username or password"),
            Self::UserAlreadyExists => ErrorModel::new("USER_ALREADY_EXISTS", "User with this name already exists"),
            Self::UserPasswordCantBeEmpty => ErrorModel::new("USER_PASSWORD_CANT_BE_EMPTY", "Username and password cannot be empty"),
            Self::SystemAlreadyExists => ErrorModel::new("SYSTEM_ALREADY_EXISTS", "System with this name already exists"),
            Self::SystemAlreadyMember => ErrorModel::new("SYSTEM_ALREADY_MEMBER", "User is already member of the system"),
            Self::SystemAlreadyOwner => ErrorModel::new("SYSTEM_ALREADY_OWNER", "Cannot add access to owner, who already has full access"),
            Self::SystemNotMember => ErrorModel::new("SYSTEM_NOT_MEMBER", "User is not a member of the system"),
            Self::SystemDoesntExist => ErrorModel::new("SYSTEM_DOESNT_EXIST", "System doesn't exist"),
            Self::SystemNotOwner => ErrorModel::new("SYSTEM_NOT_OWNER", "You are not the owner of the system"),
            Self::DeviceAlreadyExists => ErrorModel::new("DEVICE_ALREADY_EXISTS", "Device with this name already exists"),
            Self::DeviceNotOwner => ErrorModel::new("DEVICE_NOT_OWNER", "You are not the owner of the device"),
            Self::DeviceDoesntExist => ErrorModel::new("DEVICE_DOESNT_EXIST", "Device doesn't exist"),
            Self::DeviceStructureDoesntExist => ErrorModel::new("DEVICE_STRUCTURE_DOESNT_EXIST", "Device structure doesn't exist"),
            Self::InternalError(message) => ErrorModel::new("INTERNAL_ERROR", message),
        }
    }
}