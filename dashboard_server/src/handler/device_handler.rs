use crate::{
    schema::device_schema::{
        DeviceNewSchema, 
        DeviceStructureNewSchema, 
        DeviceUpdateSchema, 
        DeviceStructureUpdateSchema
    }, 
    model::{
        DeviceModel, 
        DeviceListModel, 
        DeviceStructureModel
    }
};
use super::prelude::*;

#[derive(Deserialize)]
pub struct DeviceDeleteQuery {
    pub device_id: i32,
}

#[derive(Deserialize)]
pub struct DeviceGetQuery {
    pub device_id: i32,
}

#[derive(Deserialize)]
pub struct DeviceStructureDeleteQuery {
    pub device_id: i32,
    pub structure_id: i32,
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

    if let Ok(_) = DeviceListModel::find_by_name_and_user_id(&data.db, &body.name, user_id).await {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::DeviceAlreadyExists.get_error())
        ));
    }

    let new_device = DeviceModel::new(body.name.clone(), body.topic.clone(), user_id);
    new_device.insert(&data.db).await?;
    let new_device = DeviceListModel::find_by_name_and_user_id(&data.db, &body.name, user_id).await?;

    for s in body.0.structure {
        let new_structure = DeviceStructureModel::new(new_device.device_id, 
                                                                            s.real_name, 
                                                                            s.alias_name, 
                                                                            s.data_type);
        new_structure.insert(&data.db).await?;
    }

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    delete,
    path = "/api/device/{device_id}",
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
    params(
            ("device_id" = i32, Path, description = ""),
        )
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

#[utoipa::path(
    patch,
    path = "/api/device/{device_id}",
    request_body = DeviceUpdateSchema,
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
    params(
            ("device_id" = i32, Path, description = ""),
        )
)]
pub async fn device_update(body: web::Json<DeviceUpdateSchema>, 
                        query: web::Path<DeviceGetQuery>,
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    let user_id: i32 = identity.id().unwrap().parse().unwrap();

    let device = match DeviceModel::find_by_id(&data.db, query.device_id).await {
        Ok(device) => device,
        Err(_) => return Ok(HttpResponse::BadRequest().json(
                    serde_json::json!(ResponseError::DeviceDoesntExist.get_error())))
    };

    if let Ok(found) = DeviceListModel::find_by_name_and_user_id(&data.db, &body.name, user_id).await {
        if found.device_id != device.device_id {
            return Ok(HttpResponse::BadRequest().json(
                serde_json::json!(ResponseError::DeviceAlreadyExists.get_error())));
        }
    }

    if device.owner_id != user_id {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::DeviceNotOwner.get_error())));
    }

    let new_device = DeviceModel::new(body.name.clone(), body.topic.clone(), user_id);
    new_device.update(&data.db, query.device_id).await?;

    let current_structure = DeviceStructureModel::all_by_device_id(&data.db, query.device_id).await?;
    let mut updated_structure = body.0.structure;

    // Delete unused structures
    for current in current_structure.iter() {
        if updated_structure.iter().any(|x| x.devicestructure_id == current.devicestructure_id) {
            continue;
        }

        DeviceStructureModel::delete(&data.db, current.devicestructure_id).await?;
    }

    // Iterate through updated and edit or add missing
    for updated in updated_structure.iter() {
        if let Some(existing) = current_structure.iter().find(|x| x.devicestructure_id == updated.devicestructure_id) {
            let new_updated = DeviceStructureModel::new(0, 
                                                                    updated.real_name.clone(), 
                                                                    updated.alias_name.clone(), 
                                                                    updated.data_type.clone());
            new_updated.update(&data.db, updated.devicestructure_id).await?;
        }
        else {
            let new_structure = DeviceStructureModel::new(device.device_id, 
                                                                        updated.real_name.clone(), 
                                                                        updated.alias_name.clone(), 
                                                                        updated.data_type.clone());
            new_structure.insert(&data.db).await?;
        }
    }


    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    get,
    path = "/api/device/list",
    responses(
        (status = 200, body = Vec<DeviceListModel>),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
)]
pub async fn device_list(identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    let user_id: i32 = identity.id().unwrap().parse().unwrap();

    let device_list = DeviceListModel::get_user_devices(&data.db, user_id).await?; 

    Ok(HttpResponse::Ok().json(device_list))
}

#[utoipa::path(
    get,
    path = "/api/device/{device_id}",
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
    params(
            ("device_id" = i32, Path, description = ""),
        )
)]
pub async fn get_device(query: web::Path<DeviceGetQuery>,
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    let user_id: i32 = identity.id().unwrap().parse().unwrap();

    let device = DeviceModel::find_by_id(&data.db, query.device_id).await?;   

    // TODO check for owner/user has access
    /*
    if user_id != device.owner_id {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::DeviceNotOwner.get_error())));
    }
    */

    Ok(HttpResponse::Ok().json(device))
}

#[utoipa::path(
    post,
    path = "/api/device/{device_id}/structure",
    responses(
        (status = 200, body = Vec<DeviceStructureNewSchema>),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
    params(
            ("device_id" = i32, Path, description = ""),
        )
)]
pub async fn device_insert_structure(query: web::Path<DeviceGetQuery>,
                        body: web::Json<Vec<DeviceStructureNewSchema>>,
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    let user_id: i32 = identity.id().unwrap().parse().unwrap();

    let device = match DeviceModel::find_by_id(&data.db, query.device_id).await {
        Ok(result) => result,
        Err(_) => return Ok(HttpResponse::BadRequest().json(
                                serde_json::json!(ResponseError::DeviceDoesntExist.get_error())))
    };

    if user_id != device.owner_id {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::DeviceNotOwner.get_error())));
    }

    for s in body.0 {
        let structure = DeviceStructureModel::new(query.device_id, 
                                                                        s.real_name,
                                                                        s.alias_name,
                                                                        s.data_type);
        structure.insert(&data.db).await?;
    }

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    delete,
    path = "/api/device/{device_id}/structure/{structure_id}",
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
    params(
            ("device_id" = i32, Path, description = ""),
            ("structure_id" = i32, Path, description = ""),
        )
)]
pub async fn device_delete_structure(query: web::Path<DeviceStructureDeleteQuery>,
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    let user_id: i32 = identity.id().unwrap().parse().unwrap();

    let device = match DeviceModel::find_by_id(&data.db, query.device_id).await {
        Ok(result) => result,
        Err(_) => return Ok(HttpResponse::BadRequest().json(
                                serde_json::json!(ResponseError::DeviceDoesntExist.get_error())))
    };

    if user_id != device.owner_id {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::DeviceNotOwner.get_error())));
    }

    if let Err(_) = DeviceStructureModel::find_by_id(&data.db, query.structure_id).await {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::DeviceStructureDoesntExist.get_error())));
    }

    DeviceStructureModel::delete(&data.db, query.structure_id).await?;

    Ok(HttpResponse::Ok().finish())
}