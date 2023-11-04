use crate::{
    schema::system_schema::SystemNewSchema, 
    model::SystemModel
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

/*
pub async fn system_add_user(, 
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

}
*/