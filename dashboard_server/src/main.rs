use std::process::exit;

use actix_cors::Cors;
use actix_web::{HttpServer, App, http::header, middleware::Logger};
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};

mod error;
mod handler;

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
            .app_data(AppState{db: pool.clone()})
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
