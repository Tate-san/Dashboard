use super::prelude::*;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UserRegisterSchema{
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UserLoginSchema{
    pub username: String,
    pub password: String,
}