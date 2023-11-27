use crate::{
    schema::system_schema::SystemNewSchema, 
    model::{SystemModel, SystemAccessModel, UserModel, DeviceModel, DeviceListModel, SystemDetailModel}
};
use super::prelude::*;

#[derive(Deserialize)]
pub struct SystemDeleteQuery {
    pub system_id: i32,
}

#[derive(Deserialize)]
pub struct SystemQuery {
    pub system_id: i32,
}

#[derive(Deserialize)]
pub struct SystemUserQuery {
    pub system_id: i32,
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct SystemDeviceQuery {
    pub system_id: i32,
    pub device_id: i32,
}

#[utoipa::path(
    post,
    path = "/api/system",
    request_body = SystemNewSchema,
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
        (status = 401),
    )
)]
pub async fn system_new(body: web::Json<SystemNewSchema>, 
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    if body.0.name.is_empty() {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::SystemNameEmpty.get_error())));
    }

    let owner_id: i32 = identity.id().unwrap().parse().unwrap();

    if let Ok(_) = SystemModel::find_by_name_and_user_id(&data.db, 
                                                        body.name.clone(),
                                                        owner_id
                                                        ).await {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::SystemAlreadyExists.get_error())
        ));
    }

    let new_system = SystemModel::new(body.name.clone(), 
                                                    body.description.clone(), 
                                                    owner_id 
                                                );
                    
    let response = new_system.insert(&data.db).await;

    match response {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(error) => Err(error.into())
    }

}

#[utoipa::path(
    delete,
    path = "/api/system/{system_id}",
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
    params(
            ("system_id" = i32, Path, description = ""),
        )
)]
pub async fn system_delete(query: web::Path<SystemDeleteQuery>, 
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    let user_id: i32 = identity.id().unwrap().parse().unwrap();

    let found_system = SystemModel::find_by_id(&data.db, 
                                                                query.system_id 
                                                            ).await;

    match found_system {
        Ok(system) => {
            if system.owner_id != user_id {
                return Ok(HttpResponse::BadRequest().json(
                    serde_json::json!(ResponseError::SystemNotOwner.get_error())));
            }
        }
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(
                    serde_json::json!(ResponseError::SystemDoesntExist.get_error())));
        }
    }

    let response = SystemModel::delete(&data.db, query.system_id).await;

    match response {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(error) => Err(error.into())
    }

    
}
#[utoipa::path(
    patch,
    path = "/api/system/{system_id}",
    request_body = SystemNewSchema,
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
    params(
            ("system_id" = i32, Path, description = ""),
        )
)]
pub async fn system_update(body: web::Json<SystemNewSchema>, 
                        query: web::Path<SystemQuery>,
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    if body.0.name.is_empty() {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::SystemNameEmpty.get_error())));
    }

    let owner_id: i32 = identity.id().unwrap().parse().unwrap();

    if let Err(_) = SystemModel::find_by_id(&data.db, query.system_id).await {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::SystemDoesntExist.get_error())
        ));
    }

    if let Ok(found_system) = SystemModel::find_by_name_and_user_id(&data.db, 
                                                        body.name.clone(),
                                                        owner_id
                                                        ).await {
        if query.system_id != found_system.system_id {
            return Ok(HttpResponse::BadRequest().json(
                serde_json::json!(ResponseError::SystemAlreadyExists.get_error())
            ));
        }
    }

    let new_system = SystemModel::new(body.name.clone(), 
                                                    body.description.clone(), 
                                                    0);
                    
    match new_system.update(&data.db, query.system_id).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(error) => Err(error.into())
    }

}

#[utoipa::path(
    get,
    path = "/api/system/list",
    responses(
        (status = 200, body = Vec<SystemListModel>),
        (status = 400, body = ErrorModel),
        (status = 401),
    )
)]
pub async fn system_list(data: web::Data<AppState>) -> ServerResponse {

    let system_list = SystemModel::get_all_systems(&data.db).await?;

    Ok(HttpResponse::Ok().json(system_list))
}

#[utoipa::path(
    post,
    path = "/api/system/{system_id}/user/{user_id}",
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
    params(
            ("system_id" = i32, Path, description = ""),
            ("user_id" = i32, Path, description = ""),
        )
)]
pub async fn system_add_user(query: web::Path<SystemUserQuery>, 
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    let user_id: i32 = identity.id().unwrap().parse().unwrap();

    let system = match SystemModel::find_by_id(&data.db, query.system_id).await {
        Ok(system) => system,
        Err(_) => return Ok(HttpResponse::BadRequest().json(
                    serde_json::json!(ResponseError::SystemDoesntExist.get_error())))
    };

    if let Ok(_) = SystemAccessModel::find_by_user_id_system_id(&data.db, query.user_id, query.system_id).await {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::SystemAlreadyMember.get_error())))
    }

    if system.owner_id != user_id {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::SystemNotOwner.get_error())));
    }

    if system.owner_id == query.user_id {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::SystemAlreadyOwner.get_error())));
    }

    let new_access = SystemAccessModel::new(query.system_id, query.user_id);
    new_access.insert(&data.db).await?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    delete,
    path = "/api/system/{system_id}/user/{user_id}",
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
    params(
            ("system_id" = i32, Path, description = ""),
            ("user_id" = i32, Path, description = ""),
        )
)]
pub async fn system_delete_user(query: web::Path<SystemUserQuery>, 
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    let user_id: i32 = identity.id().unwrap().parse().unwrap();

    let system = match SystemModel::find_by_id(&data.db, query.system_id).await {
        Ok(system) => system,
        Err(_) => return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::SystemDoesntExist.get_error())))
    };

    if system.owner_id != user_id {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::SystemNotOwner.get_error())));
    }

    if let Err(_) = SystemAccessModel::find_by_user_id_system_id(&data.db, query.user_id, query.system_id).await {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::SystemNotMember.get_error())))
    }

    if system.owner_id == query.user_id {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::SystemAlreadyOwner.get_error())));
    }

    let delete_access = SystemAccessModel::new(query.system_id, query.user_id);
    delete_access.delete(&data.db).await?;
 
    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    get,
    path = "/api/system/{system_id}/user/list",
    responses(
        (status = 200,),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
    params(
            ("system_id" = i32, Path, description = ""),
        )
)]
pub async fn system_user_list(query: web::Path<SystemQuery>,
                            data: web::Data<AppState>) -> ServerResponse {

    let user_list = UserModel::list_users_in_system(&data.db, query.system_id).await?;

    Ok(HttpResponse::Ok().json(user_list))
}

#[utoipa::path(
    get,
    path = "/api/system/{system_id}",
    responses(
        (status = 200, body = SystemDetailModel),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
    params(
            ("system_id" = i32, Path, description = ""),
        )
)]
pub async fn system_get(query: web::Path<SystemQuery>,
                            identity: Identity,
                            data: web::Data<AppState>) -> ServerResponse {

    let user_id: i32 = identity.id().unwrap().parse().unwrap();

    let system = match SystemModel::find_by_id(&data.db, query.system_id).await {
        Ok(system) => {
            if system.owner_id != user_id {
                return Ok(HttpResponse::BadRequest().json(
                    serde_json::json!(ResponseError::SystemNotOwner.get_error())));
            }
            system
        }
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(
                serde_json::json!(ResponseError::SystemDoesntExist.get_error())));
        }
    };

    let devices = DeviceListModel::get_system_devices(&data.db, system.system_id).await?;

    let system = SystemDetailModel {
        owner_id: system.owner_id,
        system_id: system.system_id,
        name: system.name,
        description: system.description,
        devices: devices
    };

    Ok(HttpResponse::Ok().json(system))
}

#[utoipa::path(
    post,
    path = "/api/system/{system_id}/device/{device_id}",
    responses(
        (status = 200,),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
    params(
            ("system_id" = i32, Path, description = ""),
            ("device_id" = i32, Path, description = ""),
        )
)]
pub async fn system_device_add(query: web::Path<SystemDeviceQuery>,
                            identity: Identity,
                            data: web::Data<AppState>) -> ServerResponse {

    let user_id: i32 = identity.id().unwrap().parse().unwrap();

    let system = match SystemModel::find_by_id(&data.db, query.system_id).await {
        Ok(system) => {
            if system.owner_id != user_id {
                return Ok(HttpResponse::BadRequest().json(
                    serde_json::json!(ResponseError::SystemNotOwner.get_error())));
            }
            system
        }
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(
                serde_json::json!(ResponseError::SystemDoesntExist.get_error())));
        }
    };

    if let Ok(_) = DeviceListModel::get_system_device(&data.db, query.system_id, query.device_id).await {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::DeviceAlreadyExistsInSystem.get_error())));
    }


    DeviceListModel::add_device(&data.db, query.device_id, query.system_id).await?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    delete,
    path = "/api/system/{system_id}/device/{device_id}",
    responses(
        (status = 200,),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
    params(
            ("system_id" = i32, Path, description = ""),
            ("device_id" = i32, Path, description = ""),
        )
)]
pub async fn system_device_delete(query: web::Path<SystemDeviceQuery>,
                            identity: Identity,
                            data: web::Data<AppState>) -> ServerResponse {

    let user_id: i32 = identity.id().unwrap().parse().unwrap();

    match SystemModel::find_by_id(&data.db, query.system_id).await {
        Ok(system) => {
            if system.owner_id != user_id {
                return Ok(HttpResponse::BadRequest().json(
                    serde_json::json!(ResponseError::SystemNotOwner.get_error())));
            }
        }
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(
                serde_json::json!(ResponseError::SystemDoesntExist.get_error())));
        }
    };

    if let Err(_) = DeviceListModel::get_system_device(&data.db, query.system_id, query.device_id).await {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::DeviceDoesntExistInSystem.get_error())));
    }

    DeviceListModel::delete_device(&data.db, query.device_id, query.system_id).await?;

    Ok(HttpResponse::Ok().finish())
}