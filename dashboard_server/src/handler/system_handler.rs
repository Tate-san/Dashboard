use crate::{
    schema::system_schema::{SystemNewSchema, SystemAddUserSchema, SystemDeleteUserSchema}, 
    model::{SystemModel, SystemAccessModel}
};
use super::prelude::*;

#[derive(Deserialize)]
pub struct DeleteQuery {
    pub system_id: i32,
}

pub async fn system_new(body: web::Json<SystemNewSchema>, 
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    let owner_id: i32 = identity.id().unwrap().parse().unwrap();

    if let Ok(_) = SystemModel::find_by_name_and_user_id(&data.db, 
                                                        body.name.clone(),
                                                        owner_id
                                                        ).await {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"status": "error", "message": "System with this name already exists"})
        ));
    }

    let new_system = SystemModel:: new(body.name.clone(), 
                                                    body.description.clone(), 
                                                    owner_id 
                                                );
                    
    let response = new_system.insert(&data.db).await;

    match response {
        Ok(_) => Ok(HttpResponse::Ok()
                    .json(serde_json::json!({"status": "success","data": "System successfully added"}))),
        Err(error) => Err(error.into())
    }

}

pub async fn system_delete(query: web::Path<DeleteQuery>, 
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
                    serde_json::json!({"status": "error", "message": "You are not the owner of the system"})));
            }
        }
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(
                serde_json::json!({"status": "error", "message": "System doesn't exist"})));
        }
    }

    let response = SystemModel::delete(&data.db, query.system_id).await;

    match response {
        Ok(_) => Ok(HttpResponse::Ok()
                    .json(serde_json::json!({"status": "success","data": "System successfully deleted"}))),
        Err(error) => Err(error.into())
    }

}


pub async fn system_add_user(body: web::Json<SystemAddUserSchema>, 
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    let user_id: i32 = identity.id().unwrap().parse().unwrap();

    let system = match SystemModel::find_by_id(&data.db, body.system_id).await {
        Ok(system) => system,
        Err(_) => return Ok(HttpResponse::BadRequest().json(
                    serde_json::json!({"status": "error", "message": "System doesn't exist"})))
    };

    if let Ok(_) = SystemAccessModel::find_by_user_id_system_id(&data.db, body.user_id, body.system_id).await {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"status": "error", "message": "User is already member of the system"})));
    }

    if system.owner_id != user_id {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"status": "error", "message": "You are not the owner of the system"})));
    }

    if system.owner_id == body.user_id {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"status": "error", "message": "Cannot add access to owner, who already has full access"})));
    }

    let new_access = SystemAccessModel::new(body.system_id, body.user_id);
    new_access.insert(&data.db).await?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn system_delete_user(body: web::Json<SystemDeleteUserSchema>, 
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    let user_id: i32 = identity.id().unwrap().parse().unwrap();

    let system = match SystemModel::find_by_id(&data.db, body.system_id).await {
        Ok(system) => system,
        Err(_) => return Ok(HttpResponse::BadRequest().json(
                    serde_json::json!({"status": "error", "message": "System doesn't exist"})))
    };

    if let Err(_) = SystemAccessModel::find_by_user_id_system_id(&data.db, body.user_id, body.system_id).await {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"status": "error", "message": "User is not a member of the system"})));
    }

    if system.owner_id != user_id {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"status": "error", "message": "You are not the owner of the system"})));
    }

    if system.owner_id == body.user_id {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"status": "error", "message": "Cannot add access to owner, who already has full access"})));
    }

    let delete_access = SystemAccessModel::new(body.system_id, body.user_id);
    delete_access.delete(&data.db).await?;

    Ok(HttpResponse::Ok().finish())
}