use super::prelude::*;


#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct SystemAccessModel{
    pub systemaccess_id: i32,
    pub user_id: i32,
    pub system_id: i32,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct SystemModel{
    pub system_id: i32,
    pub owner_id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct SystemListModel{
    pub system_id: i32,
    pub owner_id: i32,
    pub name: String,
    pub description: String,
}

impl SystemModel {
    pub fn new(name: String, description: String, owner_id: i32) -> Self {
        SystemModel{
            system_id: 0,
            owner_id: owner_id,
            name: name,
            description: description
        }
    }

    pub async fn get_all_systems(conn: &sqlx::Pool<Postgres>) -> DatabaseResult<Vec<SystemListModel>> {
        sqlx::query_as!(SystemListModel, r#"SELECT system_id, owner_id, name, description FROM systems"#)
            .fetch_all(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn find_by_name_and_user_id(conn: &sqlx::Pool<Postgres>, name: String, user_id: i32) -> DatabaseResult<SystemModel> {
        sqlx::query_as!(SystemModel, r#"SELECT * FROM systems WHERE name = $1 AND owner_id = $2"#, name, user_id)
            .fetch_one(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn find_by_id(conn: &sqlx::Pool<Postgres>, system_id: i32) -> DatabaseResult<SystemModel> {
        sqlx::query_as!(SystemModel, r#"SELECT * FROM systems WHERE system_id = $1"#, system_id)
            .fetch_one(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn insert(&self, conn: &sqlx::Pool<Postgres>) -> DatabaseResult<PgQueryResult> {
        sqlx::query(r#"INSERT INTO systems(name, description, owner_id) VALUES ($1, $2, $3)"#)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.owner_id)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn delete(conn: &sqlx::Pool<Postgres>, system_id: i32) -> DatabaseResult<PgQueryResult> {
        sqlx::query(r#"DELETE FROM systems WHERE system_id = $1"#)
            .bind(system_id)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }
}

impl SystemAccessModel {
    pub fn new(system_id: i32, user_id: i32) -> Self {
        SystemAccessModel{
            systemaccess_id: 0,
            system_id,
            user_id
        }
    }

    pub async fn insert(&self, conn: &sqlx::Pool<Postgres>) -> DatabaseResult<PgQueryResult> {
        sqlx::query(r#"INSERT INTO systemaccess(user_id, system_id) VALUES ($1, $2)"#)
            .bind(&self.user_id)
            .bind(&self.system_id)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn delete(&self, conn: &sqlx::Pool<Postgres>) -> DatabaseResult<PgQueryResult> {
        sqlx::query(r#"DELETE FROM systemaccess WHERE user_id = $1 AND system_id = $2"#)
            .bind(&self.user_id)
            .bind(&self.system_id)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn find_by_user_id_system_id(conn: &sqlx::Pool<Postgres>, user_id: i32, system_id: i32) -> DatabaseResult<SystemAccessModel> {
        sqlx::query_as!(SystemAccessModel, r#"SELECT * FROM systemaccess WHERE user_id = $1 AND system_id = $2"#, user_id, system_id)
            .fetch_one(conn)
            .await
            .map_err(|err| err.into())
    }

}