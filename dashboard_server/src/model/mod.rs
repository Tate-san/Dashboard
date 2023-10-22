pub mod user;

pub use user::*;
pub use crate::error::DatabaseError;

mod prelude {
    pub use serde::{Deserialize, Serialize};
    pub use sqlx::{Pool, Postgres};
    pub use super::DatabaseResult;
    pub use crate::error::DatabaseError;
}

pub type DatabaseResult<A> = Result<A, DatabaseError>;