use sqlx::postgres::PgQueryResult;

use super::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserModel{
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub role_id: Option<i32>,
}

impl UserModel{

    pub fn new(username: String, password: String) -> Self {
        UserModel { 
            user_id: 0, 
            username, 
            password, 
            role_id: None ,
        }
    }

    pub async fn find_by_name(conn: &sqlx::Pool<Postgres>, username: &str) -> DatabaseResult<UserModel> {
        sqlx::query_as!(UserModel, r#"SELECT * from users WHERE username = $1"#, username)
            .fetch_one(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn find_by_id(conn: &sqlx::Pool<Postgres>, id: i32) -> DatabaseResult<UserModel> {
        sqlx::query_as!(UserModel, r#"SELECT * from users WHERE user_id = $1"#, id)
            .fetch_one(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn insert(&self, conn: &sqlx::Pool<Postgres>) -> DatabaseResult<PgQueryResult> {
        sqlx::query(r#"INSERT INTO users(username,password,role_id) VALUES ($1, $2, $3)"#)
            .bind(&self.username)
            .bind(&self.password)
            .bind(1)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }
}


