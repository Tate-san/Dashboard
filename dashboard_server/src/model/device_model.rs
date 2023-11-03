use super::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceStructureModel{
    pub devicestructure_id: i32,
    pub device_id: i32,
    pub real_name: String,
    pub alias_name: String,
    pub data_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceModel{
    pub device_id: i32,
    pub name: String,
    pub topic: String,
}

impl DeviceModel{
    pub fn new(name: String, topic: String) -> Self {
        DeviceModel{
            device_id: 0,
            name: name,
            topic: topic,
        }
    }

    pub async fn list_all_devices(conn: &sqlx::Pool<Postgres>) -> DatabaseResult<Vec<DeviceModel>> {
        sqlx::query_as!(DeviceModel, r#"SELECT * from devices"#)
            .fetch_all(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn find_by_name(conn: &sqlx::Pool<Postgres>, name: String) -> DatabaseResult<DeviceModel> {
        sqlx::query_as!(DeviceModel, r#"SELECT * from devices WHERE name = $1"#, name)
            .fetch_one(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn find_by_id(conn: &sqlx::Pool<Postgres>, id: i32) -> DatabaseResult<DeviceModel> {
        sqlx::query_as!(DeviceModel, r#"SELECT * from devices WHERE device_id = $1"#, id)
            .fetch_one(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn insert(&self, conn: &sqlx::Pool<Postgres>) -> DatabaseResult<PgQueryResult> {
        sqlx::query(r#"INSERT INTO devices(name,topic) VALUES ($1, $2)"#)
            .bind(&self.name)
            .bind(&self.topic)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }
}