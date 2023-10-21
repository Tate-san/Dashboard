use super::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserModel{
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub role_id: Option<i32>,
}