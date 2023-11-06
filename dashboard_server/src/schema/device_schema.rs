use super::prelude::*;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct DeviceNewSchema{
    pub name: String,
    pub topic: String,
}