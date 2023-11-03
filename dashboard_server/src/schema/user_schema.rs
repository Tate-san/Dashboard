use super::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRegisterSchema{
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginSchema{
    pub username: String,
    pub password: String,
}