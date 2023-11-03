use super::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemModel{
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

    pub async fn find_by_name_and_user_id(conn: &sqlx::Pool<Postgres>, name: String, user_id: i32) -> DatabaseResult<SystemModel> {
        sqlx::query_as!(SystemModel, r#"SELECT * from systems WHERE name = $1 AND owner_id = $2"#, name, user_id)
            .fetch_one(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn find_by_id(conn: &sqlx::Pool<Postgres>, system_id: i32) -> DatabaseResult<SystemModel> {
        sqlx::query_as!(SystemModel, r#"SELECT * from systems WHERE system_id = $1"#, system_id)
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
        sqlx::query(r#"DELETE FROM systems where system_id = $1"#)
            .bind(system_id)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }
}