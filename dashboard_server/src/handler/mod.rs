pub mod prelude {
    pub use crate::error::ServerError;
    pub use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
    pub use serde_json::json;
    pub use crate::AppState;
    pub use actix_session::Session;
    pub use actix_identity::Identity;
    pub use crate::error::ServerResponse;
}
use prelude::*;

mod user_handler;

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
            );

    conf.service(scope);
}