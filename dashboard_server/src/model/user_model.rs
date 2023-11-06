use super::prelude::*;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct RoleListModel{
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub enum RoleModel{
    User = 1,
    Admin
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UserModel{
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub role: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UserListModel{
    pub user_id: i32,
    pub username: String,
}

impl UserModel{

    pub fn new(username: String, password: String) -> Self {
        UserModel { 
            user_id: 0, 
            username, 
            password, 
            role: None,
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

    pub async fn list(conn: &sqlx::Pool<Postgres>) -> DatabaseResult<Vec<UserListModel>> {
        sqlx::query_as!(UserListModel, r#"SELECT user_id,username from users"#)
            .fetch_all(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn insert(&self, conn: &sqlx::Pool<Postgres>) -> DatabaseResult<PgQueryResult> {
        sqlx::query(r#"INSERT INTO users(username,password,role) VALUES ($1, $2, $3)"#)
            .bind(&self.username)
            .bind(&self.password)
            .bind(RoleModel::User as i32)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }
}


