use super::prelude::*;
use actix_web::{HttpRequest, HttpMessage};
use argon2::{self};
use crate::schema::UserRegisterSchema;
use crate::model::UserModel;

pub async fn user_register(web::Form(form): web::Form<UserRegisterSchema>, data: web::Data<AppState>) -> ServerResponse {

    let salt = b"SomeRandomSalt";
    let config = argon2::Config::default();
    let hashed_password = argon2::hash_encoded(form.password.as_bytes(), salt, &config).unwrap();

    let query = 
        sqlx::query(r#"INSERT INTO users(username,password,role_id) VALUES ($1, $2, $3)"#)
        .bind(form.username)
        .bind(hashed_password)
        .bind(1)
        .execute(&data.db)
        .await;

    match query {
        Ok(_) => Ok(HttpResponse::Ok()
                    .json(serde_json::json!({"status": "success","data": "User successfully registered"}))),
        Err(error) => {
            let message = error.to_string();
            if message.contains("duplicate key"){
                return Ok(HttpResponse::BadRequest()
                    .json(serde_json::json!({"status": "error", "message": "User already exists with this name"})));
            }
            Err(error.into()) 
        }
    }

    

}

pub async fn user_login(request: HttpRequest, web::Form(form): web::Form<UserRegisterSchema>, data: web::Data<AppState>) -> ServerResponse {

    if form.username.is_empty() || form.password.is_empty() {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"status": "error", "message": "Missing username or password"})
        ));
    }

    let query = 
        sqlx::query_as!(UserModel, r#"SELECT * from users WHERE username = $1"#, form.username)
        .fetch_one(&data.db)
        .await;

    if let Err(error) = query {
        match error{
            sqlx::Error::RowNotFound => {
                return Ok(HttpResponse::BadRequest()
                    .json(serde_json::json!({"status": "error","message": "Invalid username or password"})));
            }
            _ => {
                return Err(error.into());
            }
        }
    }

    let user = query.unwrap();

    let password_valid = argon2::verify_encoded(&user.password, form.password.as_bytes())?;

    if password_valid {
        Identity::login(&request.extensions(), user.user_id.to_string()).unwrap(); 
        return Ok(HttpResponse::Ok().finish());
    }

    return Ok(HttpResponse::BadRequest()
        .json(serde_json::json!({"status": "error", "message": "Invalid username or password"})));

}


pub async fn user_logout(identity: Option<Identity>) -> ServerResponse {
    match identity {
        Some(identity) => {
            identity.logout();
            Ok(HttpResponse::Ok().finish())
        }
        None => {
            Ok(HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "error", "message": "Not logged in"})))
        }
    }
}