use crate::{
    schema::system_schema::{SystemNewSchema, SystemAddUserSchema, SystemDeleteUserSchema}, 
    model::{SystemModel, SystemAccessModel}
};
use super::prelude::*;

#[derive(Deserialize)]
pub struct SystemDeleteQuery {
    pub system_id: i32,
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

    let owner_id: i32 = identity.id().unwrap().parse().unwrap();

    if let Ok(_) = SystemModel::find_by_name_and_user_id(&data.db, 
                                                        body.name.clone(),
                                                        owner_id
                                                        ).await {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::SystemAlreadyExists.get_error())
        ));
    }

    let new_system = SystemModel:: new(body.name.clone(), 
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
    path = "/api/system/user",
    request_body = SystemAddUserSchema,
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
        (status = 401),
    )
)]
pub async fn system_add_user(body: web::Json<SystemAddUserSchema>, 
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    println!("Post");
    let user_id: i32 = identity.id().unwrap().parse().unwrap();

    let system = match SystemModel::find_by_id(&data.db, body.system_id).await {
        Ok(system) => system,
        Err(_) => return Ok(HttpResponse::BadRequest().json(
                    serde_json::json!(ResponseError::SystemDoesntExist.get_error())))
    };

    if let Ok(_) = SystemAccessModel::find_by_user_id_system_id(&data.db, body.user_id, body.system_id).await {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::SystemAlreadyMember.get_error())))
    }

    if system.owner_id != user_id {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::SystemNotOwner.get_error())));
    }

    if system.owner_id == body.user_id {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::SystemAlreadyOwner.get_error())));
    }

    let new_access = SystemAccessModel::new(body.system_id, body.user_id);
    new_access.insert(&data.db).await?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    delete,
    path = "/api/system/user",
    request_body = SystemDeleteUserSchema,
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
        (status = 401),
    )
)]
pub async fn system_delete_user(body: web::Json<SystemDeleteUserSchema>, 
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    let user_id: i32 = identity.id().unwrap().parse().unwrap();

    let system = match SystemModel::find_by_id(&data.db, body.system_id).await {
        Ok(system) => system,
        Err(_) => return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::SystemDoesntExist.get_error())))
    };

    if system.owner_id != user_id {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::SystemNotOwner.get_error())));
    }

    if let Err(_) = SystemAccessModel::find_by_user_id_system_id(&data.db, body.user_id, body.system_id).await {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::SystemNotMember.get_error())))
    }

    if system.owner_id == body.user_id {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::SystemAlreadyOwner.get_error())));
    }

    let delete_access = SystemAccessModel::new(body.system_id, body.user_id);
    delete_access.delete(&data.db).await?;
 
    Ok(HttpResponse::Ok().finish())
}