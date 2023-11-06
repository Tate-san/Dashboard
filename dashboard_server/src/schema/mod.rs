pub mod user_schema;
pub mod system_schema;
pub mod device_schema;

pub mod prelude {
    pub use serde::{Deserialize, Serialize};
    pub use utoipa::ToSchema;
}
