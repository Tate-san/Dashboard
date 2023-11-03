pub mod user_model;
pub mod device_model;
pub mod system_model;

pub use user_model::*;
pub use device_model::*;
pub use system_model::*;
pub use crate::error::DatabaseError;

mod prelude {
    pub use std::vec::Vec;
    pub use serde::{Deserialize, Serialize};
    pub use sqlx::{Pool, Postgres};
    pub use super::DatabaseResult;
    pub use sqlx::postgres::PgQueryResult;
    pub use crate::error::DatabaseError;
}

pub type DatabaseResult<A> = Result<A, DatabaseError>;