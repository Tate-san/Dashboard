use super::prelude::*;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct DeviceNewSchema {
    pub name: String,
    pub topic: String,
    pub structure: Vec<DeviceStructureNewSchema>
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct DeviceStructureNewSchema {
    pub real_name: String,
    pub alias_name: String,
    pub data_type: String,
}