pub mod prelude {
    pub use crate::error::ServerError;
    pub use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
    pub use serde_json::json;
    pub use crate::AppState;
    pub use actix_session::Session;
    pub use actix_identity::Identity;
    pub use crate::error::{ServerResponse, DatabaseError};
    pub use serde::{Deserialize, Serialize};
}
use prelude::*;

mod user_handler;
mod device_handler;
mod system_handler;

async fn health_check() -> ServerResponse {
    Ok(HttpResponse::Ok().json(json!({
        "status": "success", 
        "result": "Server is up and running" 
    })))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
            .route("/health-check", web::get().to(health_check))
            .service(web::scope("/user")
                .route("/login", web::post().to(user_handler::user_login))
                .route("/register", web::post().to(user_handler::user_register))
                .route("/logout", web::get().to(user_handler::user_logout))
                .route("/list", web::get().to(user_handler::user_list))
                .route("/hello", web::get().to(user_handler::user_hello))
                .route("/roles", web::get().to(user_handler::list_roles))
            )
            .service(web::scope("/device")
                .route("/new", web::post().to(device_handler::device_new))
            )
            .service(web::scope("/system")
                .route("/new", web::post().to(system_handler::system_new))
                .route("/delete/{system_id}", web::delete().to(system_handler::system_delete))
                .route("/user", web::post().to(system_handler::system_add_user))
                .route("/user", web::delete().to(system_handler::system_delete_user))
            );
            

    conf.service(scope);
}