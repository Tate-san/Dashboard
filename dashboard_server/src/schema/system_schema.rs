use super::prelude::*;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct SystemNewSchema{
    pub name: String,
    pub description: String,
}