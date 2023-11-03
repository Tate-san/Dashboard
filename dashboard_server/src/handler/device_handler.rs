use crate::{
    schema::device_schema::DeviceNewSchema, 
    model::DeviceModel
};
use super::prelude::*;

#[derive(Deserialize)]
struct DeleteQuery {
    device_id: u32,
}

pub async fn device_new(web::Form(form): web::Form<DeviceNewSchema>, 
                        _: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    if let Ok(_) = DeviceModel::find_by_name(&data.db, form.name.clone()).await {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"status": "error", "message": "Device with this name already exists"})
        ));
    }

    let new_device = DeviceModel::new(form.name, form.topic);

    let response = new_device.insert(&data.db).await;

    match response {
        Ok(_) => Ok(HttpResponse::Ok()
                    .json(serde_json::json!({"status": "success","data": "Device successfully added"}))),
        Err(error) => Err(error.into())
    }
}
