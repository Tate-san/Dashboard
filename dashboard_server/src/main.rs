use std::process::exit;

use actix_web::{HttpServer, App, web, http::header, middleware::Logger};
use actix_cors::Cors;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_identity::{Identity, IdentityMiddleware};
use actix_web::cookie::Key;
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};

mod error;
mod handler;
mod schema;
mod model;

pub struct AppState {
    db: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let session_key = Key::generate();

    println!("🚀 Server started successfully!");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_methods(["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(
                web::Data::new(
                    AppState { 
                        db: pool.clone() 
                    })
            )
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), session_key.clone())
                        .cookie_secure(false)
                        .cookie_name("_r_session_".to_string())
                        .build()
            )
            .wrap(IdentityMiddleware::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
