use super::prelude::*;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
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

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct DeviceModel {
    pub device_id: i32,
    pub owner_id: i32,
    pub name: String,
    pub topic: String,
    pub structure: Vec<DeviceStructureModel>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct DeviceListModel {
    pub device_id: i32,
    pub owner_id: i32,
    pub name: String,
    pub topic: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct SystemDevicesModel {
    pub systemdevices_id: i32,
    pub system_id: i32,
    pub device_id: i32,
}

impl DeviceListModel {
    pub async fn add_device(conn: &sqlx::Pool<Postgres>, device_id: i32, system_id: i32) -> DatabaseResult<PgQueryResult> {
        sqlx::query(r#"INSERT INTO systemdevices(system_id, device_id) VALUES ($1, $2)"#)
            .bind(system_id)
            .bind(device_id)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn delete_device(conn: &sqlx::Pool<Postgres>, device_id: i32, system_id: i32) -> DatabaseResult<PgQueryResult> {
        sqlx::query(r#"DELETE FROM systemdevices where device_id = $1 and system_id = $2"#)
            .bind(device_id)
            .bind(system_id)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }
}

impl DeviceListModel {
    pub async fn get_device(conn: &sqlx::Pool<Postgres>, device_id: i32) -> DatabaseResult<DeviceListModel> {
        sqlx::query_as!(DeviceListModel, r#"SELECT device_id,owner_id,name,topic FROM devices WHERE device_id = $1"#, device_id)
            .fetch_one(conn)
            .await
            .map_err(|err| err.into())
    }
    pub async fn get_all_devices(conn: &sqlx::Pool<Postgres>) -> DatabaseResult<Vec<DeviceListModel>> {
        sqlx::query_as!(DeviceListModel, r#"SELECT device_id,owner_id,name,topic FROM devices"#)
            .fetch_all(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn get_user_devices(conn: &sqlx::Pool<Postgres>, user_id: i32) -> DatabaseResult<Vec<DeviceListModel>> {
        sqlx::query_as!(DeviceListModel, r#"SELECT device_id,owner_id,name,topic FROM devices WHERE owner_id = $1"#, user_id)
            .fetch_all(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn find_by_name_and_user_id(conn: &sqlx::Pool<Postgres>, name: &str, user_id: i32) -> DatabaseResult<DeviceListModel> {
        sqlx::query_as!(DeviceListModel, r#"SELECT device_id,owner_id,name,topic FROM devices WHERE name = $1 and owner_id = $2"#, name, user_id)
            .fetch_one(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn get_system_devices(conn: &sqlx::Pool<Postgres>, system_id: i32) -> DatabaseResult<Vec<DeviceListModel>> {
        let system_devices = sqlx::query_as!(SystemDevicesModel, r#"SELECT systemdevices_id, system_id, device_id FROM systemdevices WHERE system_id = $1"#, system_id)
            .fetch_all(conn)
            .await
            .map_err(|err| err)?;

        let mut devices: Vec<DeviceListModel> = vec![];

        for system_device in system_devices {
            let device = DeviceListModel::get_device(conn, system_device.device_id).await?;
            devices.push(device);
        }

        Ok(devices)
    }

    pub async fn get_system_device(conn: &sqlx::Pool<Postgres>, system_id: i32, device_id: i32) -> DatabaseResult<SystemDevicesModel> {
        sqlx::query_as!(SystemDevicesModel, r#"SELECT systemdevices_id, system_id, device_id FROM systemdevices WHERE system_id = $1 and device_id = $2"#, system_id, device_id)
            .fetch_one(conn)
            .await
            .map_err(|err| err.into())

    }
}

impl Default for DeviceModel {
    fn default() -> Self {
        Self {
            device_id: 0,
            owner_id: 0,
            name: String::new(),
            topic: String::new(),
            structure: vec![],
        }
    }
}

impl DeviceModel {

    pub fn new(name: String, topic: String, owner_id: i32) -> Self {
        DeviceModel{
            device_id: 0,
            owner_id: owner_id,
            name: name,
            topic: topic,
            ..Default::default()
        }
    }

    pub async fn find_by_id(conn: &sqlx::Pool<Postgres>, id: i32) -> DatabaseResult<DeviceModel> {
        let result = sqlx::query_as!(DeviceModelDB, r#"SELECT device_id, owner_id, name, topic FROM devices WHERE device_id = $1"#, id)
            .fetch_one(conn)
            .await?; 
        
        let device_structure = DeviceStructureModel::all_by_device_id(conn, id).await?;

        Ok(DeviceModel{
            device_id: result.device_id,
            owner_id: result.owner_id,
            name: result.name,
            topic: result.topic,
            structure: device_structure,
        })
    }

    pub async fn insert(&self, conn: &sqlx::Pool<Postgres>) -> DatabaseResult<PgQueryResult> {
        sqlx::query(r#"INSERT INTO devices(name,topic,owner_id) VALUES ($1, $2, $3)"#)
            .bind(&self.name)
            .bind(&self.topic)
            .bind(&self.owner_id)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn delete(conn: &sqlx::Pool<Postgres>, device_id: i32) -> DatabaseResult<PgQueryResult> {
        sqlx::query(r#"DELETE FROM devices where device_id = $1"#)
            .bind(device_id)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn update(&self, conn: &sqlx::Pool<Postgres>, device_id: i32) -> DatabaseResult<PgQueryResult> {
        sqlx::query(r#"UPDATE devices SET name = $1, topic = $2 WHERE device_id = $3"#)
            .bind(&self.name)
            .bind(&self.topic)
            .bind(device_id)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }
}

impl DeviceStructureModel {
    pub fn new(device_id: i32, real_name: String, alias_name: String, data_type: String) -> Self {
        DeviceStructureModel {
            devicestructure_id: 0,
            device_id,
            real_name,
            alias_name,
            data_type
        }
    }

    pub async fn all_by_device_id(conn: &sqlx::Pool<Postgres>, device_id: i32) -> DatabaseResult<Vec<DeviceStructureModel>> {
        sqlx::query_as!(DeviceStructureModel, r#"SELECT * FROM devicestructure WHERE device_id = $1"#, device_id)
            .fetch_all(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn find_by_id(conn: &sqlx::Pool<Postgres>, devicestructure_id: i32) -> DatabaseResult<DeviceStructureModel> {
        sqlx::query_as!(DeviceStructureModel, r#"SELECT * FROM devicestructure WHERE devicestructure_id = $1"#, devicestructure_id)
            .fetch_one(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn delete(conn: &sqlx::Pool<Postgres>, devicestructure_id: i32) -> DatabaseResult<PgQueryResult> {
        sqlx::query(r#"DELETE FROM devicestructure where devicestructure_id = $1"#)
            .bind(devicestructure_id)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn insert(&self, conn: &sqlx::Pool<Postgres>) -> DatabaseResult<PgQueryResult> {
        sqlx::query(r#"INSERT INTO devicestructure(device_id, real_name, alias_name, data_type) VALUES($1, $2, $3, $4)"#)
            .bind(&self.device_id)
            .bind(&self.real_name)
            .bind(&self.alias_name)
            .bind(&self.data_type)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn update(&self, conn: &sqlx::Pool<Postgres>, devicestructure_id: i32) -> DatabaseResult<PgQueryResult> {
        sqlx::query(r#"UPDATE devicestructure SET real_name = $1, alias_name = $2, data_type = $3 WHERE devicestructure_id = $4"#)
            .bind(&self.real_name)
            .bind(&self.alias_name)
            .bind(&self.data_type)
            .bind(devicestructure_id)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }
}
