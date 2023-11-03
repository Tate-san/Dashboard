use super::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceNewSchema{
    pub name: String,
    pub topic: String,
}