pub mod user;

pub use user::*;

mod prelude {
    pub use serde::{Deserialize, Serialize};
}