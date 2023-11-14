use std::process::exit;

use actix_session::config::PersistentSession;
use actix_web::{HttpServer, App, web, http::header, middleware::Logger};
use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, config::BrowserSession};
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
            handler::user_handler::user_auth_model,
            handler::user_handler::user_list,
            handler::user_handler::list_roles,

            handler::system_handler::system_new,
            handler::system_handler::system_delete,
            handler::system_handler::system_list,
            handler::system_handler::system_add_user,
            handler::system_handler::system_delete_user,
            handler::system_handler::system_user_list,

            handler::device_handler::device_new,
            handler::device_handler::device_delete,
            handler::device_handler::device_list,
            handler::device_handler::get_device,
            handler::device_handler::device_insert_structure,
            handler::device_handler::device_delete_structure,
        ), 
        components(
            schemas(
                schema::user_schema::UserRegisterSchema,
                schema::user_schema::UserLoginSchema,
                model::user_model::UserListModel,
                model::user_model::RoleListModel,

                schema::device_schema::DeviceNewSchema,
                schema::device_schema::DeviceStructureNewSchema,
                model::device_model::DeviceModel,
                model::device_model::DeviceListModel,
                model::device_model::DeviceStructureModel,

                schema::system_schema::SystemNewSchema,
                model::system_model::SystemListModel,

                model::ErrorModel,
            )
        )
    )]
    struct ApiDoc;

    let session_key = Key::from(std::env::var("SESSION_KEY").expect("Session key not set").as_bytes());//Key::generate();
    let server_address = std::env::var("SERVER_ADDRESS").expect("Server address not set");
    let server_port = std::env::var("SERVER_PORT").expect("Server port not set");

    println!("🚀 Server started successfully!");
    println!("📡 Listening on {}:{}", 
        &server_address,
        &server_port 
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
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
                        .session_lifecycle(PersistentSession::default())
                        .build()
            )
            .wrap(IdentityMiddleware::default())
    })
    .bind(format!("{}:{}", server_address, server_port))?
    .run()
    .await
}
