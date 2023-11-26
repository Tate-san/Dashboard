use serde_derive::{Serialize, Deserialize};
use sqlx::{Postgres, postgres::PgQueryResult};
use crate::error::MqttError;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceStructureModel{
    pub devicestructure_id: i32,
    pub device_id: i32,
    pub real_name: String,
    pub alias_name: String,
    pub data_type: String,
}

#[derive(sqlx::FromRow, Debug)]
pub struct DeviceModelDB {
    pub device_id: i32,
    pub owner_id: i32,
    pub name: String,
    pub topic: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceModel {
    pub device_id: i32,
    pub owner_id: i32,
    pub name: String,
    pub topic: String,
    pub structure: Vec<DeviceStructureModel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceData {
    pub device_id: i32,
    pub devicestructure_id: i32,
    pub value: String,
    pub timestamp: chrono::DateTime<chrono::Utc>, 
}

impl DeviceModel {
    pub async fn all_by_topic(conn: &sqlx::Pool<Postgres>, topic: String) -> Result<Vec<DeviceModel>, MqttError> {
        let devices: Vec<DeviceModelDB> = sqlx::query_as!(DeviceModelDB, r#"SELECT * FROM devices WHERE topic = $1"#, topic)
            .fetch_all(conn)
            .await
            .map_err(|err| MqttError::SqlxError(err))?;

        let mut result: Vec<DeviceModel> = vec![];

        for device in devices {
            result.push(DeviceModel {
                device_id: device.device_id,
                owner_id: device.owner_id,
                name: device.name,
                topic: device.topic,
                structure: DeviceStructureModel::all_by_device_id(conn, device.device_id).await?
            });
        }

        Ok(result)
    }
}

impl DeviceStructureModel {
    pub async fn all_by_device_id(conn: &sqlx::Pool<Postgres>, device_id: i32) -> Result<Vec<DeviceStructureModel>, MqttError> {
        sqlx::query_as!(DeviceStructureModel, r#"SELECT * FROM devicestructure WHERE device_id = $1"#, device_id)
            .fetch_all(conn)
            .await
            .map_err(|err| err.into())
    }
}

impl DeviceData {
    pub async fn insert(&self, conn: &sqlx::Pool<Postgres>) -> Result<PgQueryResult, MqttError> {
        sqlx::query(r#"INSERT INTO devicedata(device_id, devicestructure_id, value, timestamp) VALUES($1, $2, $3, $4)"#)
            .bind(&self.device_id)
            .bind(&self.devicestructure_id)
            .bind(&self.value)
            .bind(&self.timestamp)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }


}