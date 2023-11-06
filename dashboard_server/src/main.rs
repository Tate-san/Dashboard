use std::process::exit;

use actix_web::{HttpServer, App, web, http::header, middleware::Logger};
use actix_cors::Cors;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_identity::{Identity, IdentityMiddleware};
use actix_web::cookie::Key;
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

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
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    
    #[derive(OpenApi)]
    #[openapi(
        paths(
            handler::user_handler::user_register,
            handler::user_handler::user_login,
            handler::user_handler::user_logout,
            handler::user_handler::user_list,
            handler::user_handler::list_roles,
            handler::system_handler::system_new,
            handler::system_handler::system_delete,
            handler::system_handler::system_add_user,
            handler::system_handler::system_delete_user,
            handler::device_handler::device_new,
        ), 
        components(
            schemas(
                schema::user_schema::UserRegisterSchema,
                schema::user_schema::UserLoginSchema,
                model::user_model::UserListModel,
                model::user_model::RoleListModel,
                schema::system_schema::SystemNewSchema,
                schema::system_schema::SystemAddUserSchema,
                schema::system_schema::SystemDeleteUserSchema,
                schema::device_schema::DeviceNewSchema,
                model::ErrorModel,
            )
        )
    )]
    struct ApiDoc;

    let session_key = Key::generate();
    let server_address = std::env::var("SERVER_ADDRESS").expect("Server address not set");
    let server_port = std::env::var("SERVER_PORT").expect("Server port not set");

    println!("🚀 Server started successfully!");
    println!("📡 Listening on {}:{}", 
        &server_address,
        &server_port 
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8080")
            .allowed_origin("http://localhost:8080")
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
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![
                (
                    Url::new("api1", "/api-docs/openapi1.json"),
                    ApiDoc::openapi(),
                )
            ]))
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
    .bind(format!("{}:{}", server_address, server_port))?
    .run()
    .await
}
