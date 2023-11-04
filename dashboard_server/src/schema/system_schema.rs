use super::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemNewSchema{
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemAddUserSchema{
    pub user_id: i32,
    pub system_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemDeleteUserSchema{
    pub user_id: i32,
    pub system_id: i32,
}