use super::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemNewSchema{
    pub name: String,
    pub description: String,
}