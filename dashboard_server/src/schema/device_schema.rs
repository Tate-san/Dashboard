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

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DeviceUpdateSchema {
    pub name: String,
    pub topic: String,
    pub structure: Vec<DeviceStructureUpdateSchema>
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DeviceStructureUpdateSchema {
    pub devicestructure_id: i32,
    pub real_name: String,
    pub alias_name: String,
    pub data_type: String,
}