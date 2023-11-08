use crate::{
    schema::device_schema::DeviceNewSchema, 
    model::DeviceModel
};
use super::prelude::*;

#[derive(Deserialize)]
pub struct DeviceDeleteQuery {
    pub device_id: i32,
}

#[utoipa::path(
    post,
    path = "/api/device",
    request_body = DeviceNewSchema,
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
)]
pub async fn device_new(body: web::Json<DeviceNewSchema>, 
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    let user_id: i32 = identity.id().unwrap().parse().unwrap();

    if let Ok(_) = DeviceModel::find_by_name(&data.db, body.name.clone()).await {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::DeviceAlreadyExists.get_error())
        ));
    }

    let new_device = DeviceModel::new(body.name.clone(), body.topic.clone(), user_id);

    let response = new_device.insert(&data.db).await;

    match response {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(error) => Err(error.into())
    }
}

#[utoipa::path(
    delete,
    path = "/api/device",
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
)]
pub async fn device_delete(query: web::Path<DeviceDeleteQuery>, 
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

   let user_id: i32 = identity.id().unwrap().parse().unwrap();

    let found_device = DeviceModel::find_by_id(&data.db, 
                                                                query.device_id 
                                                            ).await;

    match found_device {
        Ok(device) => {
            if device.owner_id != user_id {
                return Ok(HttpResponse::BadRequest().json(
                    serde_json::json!(ResponseError::DeviceNotOwner.get_error())));
            }
        }
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(
                    serde_json::json!(ResponseError::DeviceDoesntExist.get_error())));
        }
    }

    let response = DeviceModel::delete(&data.db, query.device_id).await;

    match response {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(error) => Err(error.into())
    }
}