use crate::{
    schema::device_schema::DeviceNewSchema, 
    model::DeviceModel
};
use super::prelude::*;

#[derive(Deserialize)]
struct DeleteQuery {
    device_id: u32,
}

#[utoipa::path(
    post,
    path = "/api/device/new",
    request_body = DeviceNewSchema,
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
)]
pub async fn device_new(body: web::Json<DeviceNewSchema>, 
                        _: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    if let Ok(_) = DeviceModel::find_by_name(&data.db, body.name.clone()).await {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::DeviceAlreadyExists.get_error())
        ));
    }

    let new_device = DeviceModel::new(body.name.clone(), body.topic.clone());

    let response = new_device.insert(&data.db).await;

    match response {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(error) => Err(error.into())
    }
}
