use super::prelude::*;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct KpiNewSchema {
    pub parameter: String,
    pub limitation: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct KpiUpdateSchema {
    pub parameter: String,
    pub limitation: String,
    pub value: String,
}

