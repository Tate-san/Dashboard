pub mod prelude {
    pub use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
    pub use serde_json::json;
}

use prelude::*;

#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "success", 
        "result": "Server is up and running" 
    }))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_check);

    conf.service(scope);
}