pub mod prelude {
    pub use crate::error::ServerError;
    pub use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
    pub use serde_json::json;
    pub use crate::AppState;
    pub use actix_session::Session;
    pub use actix_identity::Identity;
    pub use crate::error::{ServerResponse, DatabaseError};
    pub use serde::{Deserialize, Serialize};
    pub use utoipa::OpenApi;
    pub use crate::model::{ErrorModel, ResponseError};
}
use prelude::*;

pub mod user_handler;
pub mod device_handler;
pub mod system_handler;

async fn health_check() -> ServerResponse {
    Ok(HttpResponse::Ok().json(json!({
        "message": "Server is up and running" 
    })))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
            .route("/health-check", web::get().to(health_check))
            .service(web::scope("/user")
                .route("", web::get().to(user_handler::user_auth_model))
                .route("/login", web::post().to(user_handler::user_login))
                .route("/register", web::post().to(user_handler::user_register))
                .route("/logout", web::get().to(user_handler::user_logout))
                .route("/list", web::get().to(user_handler::user_list))
                .route("/hello", web::get().to(user_handler::user_hello))
                .route("/roles", web::get().to(user_handler::list_roles))
            )
            .service(web::scope("/device")
                .route("", web::post().to(device_handler::device_new))
                .route("/list", web::get().to(device_handler::device_list))
                .route("/{device_id}/structure/{structure_id}", web::delete().to(device_handler::device_delete_structure))
                .route("/{device_id}/structure", web::post().to(device_handler::device_insert_structure))
                .route("/{device_id}", web::patch().to(device_handler::device_update))
                .route("/{device_id}", web::get().to(device_handler::get_device))
                .route("/{device_id}", web::delete().to(device_handler::device_delete))
            )
            .service(web::scope("/system")
                .route("", web::post().to(system_handler::system_new))
                .route("/list", web::get().to(system_handler::system_list))
                .route("/{system_id}/user/{user_id}", web::post().to(system_handler::system_add_user))
                .route("/{system_id}/user/{user_id}", web::delete().to(system_handler::system_delete_user))
                .route("/{system_id}/user/list", web::get().to(system_handler::system_user_list))
                .route("/{system_id}", web::get().to(system_handler::system_get))
                .route("/{system_id}", web::patch().to(system_handler::system_update))
                .route("/{system_id}", web::delete().to(system_handler::system_delete))
            );
            

    conf.service(scope);
}