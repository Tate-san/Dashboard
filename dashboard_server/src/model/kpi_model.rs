use super::prelude::*;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct KpiModel{
    pub kpi_id: i32,
    pub parameter: String,
    pub limitation: String,
    pub value: String,
    pub owner_id: i32,
}

impl Default for KpiModel {
    fn default() -> Self {
        Self {
            kpi_id: 0,
            parameter: String::new(),
            limitation: String::new(),
            value: String::new(),
            owner_id: 0,
        }
    }
}

impl KpiModel {

    pub fn new(parameter: String, limitation: String, value: String, owner_id: i32) -> Self {
        KpiModel {
            parameter,
            limitation,
            value,
            owner_id,
            ..Default::default()
        }
    }

    pub async fn insert(&self, conn: &sqlx::Pool<Postgres>) -> DatabaseResult<PgQueryResult> {
        sqlx::query(r#"INSERT INTO kpi(parameter, limitation, value, owner_id) VALUES ($1, $2, $3, $4)"#)
            .bind(&self.parameter)
            .bind(&self.limitation)
            .bind(&self.value)
            .bind(&self.owner_id)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn delete(conn: &sqlx::Pool<Postgres>, kpi_id: i32) -> DatabaseResult<PgQueryResult> {
        sqlx::query(r#"DELETE FROM kpi where kpi_id = $1"#)
            .bind(kpi_id)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }

    pub async fn update(&self, conn: &sqlx::Pool<Postgres>, kpi_id: i32) -> DatabaseResult<PgQueryResult> {
        sqlx::query(r#"UPDATE kpi SET parameter = $1, limitation = $2, value = $3 WHERE kpi_id = $4"#)
            .bind(&self.parameter)
            .bind(&self.limitation)
            .bind(&self.value)
            .bind(kpi_id)
            .execute(conn)
            .await
            .map_err(|err| err.into())
    }


    pub async fn get(conn: &sqlx::Pool<Postgres>, kpi_id: i32) -> DatabaseResult<KpiModel> {
        sqlx::query_as!(KpiModel, r#"SELECT kpi_id,parameter,limitation,value,owner_id FROM kpi WHERE kpi_id = $1"#, kpi_id)
            .fetch_one(conn)
            .await
            .map_err(|err| err.into())
    }


    pub async fn get_all_user(conn: &sqlx::Pool<Postgres>, owner_id: i32) -> DatabaseResult<Vec<KpiModel>> {
        sqlx::query_as!(KpiModel, r#"SELECT kpi_id,parameter,limitation,value,owner_id FROM kpi WHERE owner_id = $1"#, owner_id)
            .fetch_all(conn)
            .await
            .map_err(|err| err.into())
    }

}