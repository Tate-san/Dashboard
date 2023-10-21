use super::prelude::*;
use actix_web::{HttpRequest, HttpMessage};
use argon2::{self};
use sqlx::any::AnyQueryResult;
use crate::schema::UserRegisterSchema;
use crate::model::{UserModel, user};

pub async fn user_register(web::Form(form): web::Form<UserRegisterSchema>, data: web::Data<AppState>) -> impl Responder {

    let salt = b"SomeRandomSalt";
    let config = argon2::Config::default();
    let hashed_password = argon2::hash_encoded(form.password.as_bytes(), salt, &config).unwrap();

    let query = 
        sqlx::query(r#"INSERT INTO users(username,password,role_id) VALUES ($1, $2, $3)"#)
        .bind(form.username)
        .bind(hashed_password)
        .bind(1)
        .execute(&data.db)
        .await
        .map_err(|err| err.to_string());

    if let Err(err) = query {
        if err.contains("duplicate key") {
            return HttpResponse::BadRequest().json(
            serde_json::json!({"status": "error","message": "User with this name already exists"}),
        );
        }

        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
    }

    match query {
        Ok(_) => {
            let response = serde_json::json!({"status": "success","data": "User successfully registered"});

            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

pub async fn user_login(request: HttpRequest, web::Form(form): web::Form<UserRegisterSchema>, data: web::Data<AppState>) -> impl Responder {

    if form.username.is_empty() || form.password.is_empty() {
        return HttpResponse::BadRequest().json(
            serde_json::json!({"status": "error", "message": "Missing username or password"})
        );
    }

    let query = 
        sqlx::query_as!(UserModel, r#"SELECT * from users WHERE username = $1"#, form.username)
        .fetch_one(&data.db)
        .await;

    if let Err(error) = query {
        match error{
            sqlx::Error::RowNotFound => {
                return HttpResponse::BadRequest()
                    .json(serde_json::json!({"status": "error","message": "Invalid username or password"}));
            }
            _ => {
                return HttpResponse::InternalServerError()
                    .json(serde_json::json!({"status": "error","message": format!("{:?}", error.to_string())}));
            }
        }
    }

    let user = query.unwrap();

    let password_verify = argon2::verify_encoded(&user.password, form.password.as_bytes());

    match password_verify {
        Ok(valid) => {
            if valid {
                
                Identity::login(&request.extensions(), user.user_id.to_string()).unwrap(); 
                return HttpResponse::Ok().finish();
            }

            return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "error", "message": "Invalid username or password"}));
        }
        Err(error) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", error.to_string())}));
        }
    }

}


pub async fn user_logout(identity: Option<Identity>) -> impl Responder {
    match identity {
        Some(identity) => {
            identity.logout();
            HttpResponse::Ok().finish()
        }
        None => {
            HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "error", "message": "Not logged in"}))
        }
    }
}