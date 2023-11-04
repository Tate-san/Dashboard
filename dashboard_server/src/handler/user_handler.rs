use super::prelude::*;
use actix_web::{HttpRequest, HttpMessage};
use argon2::{self};
use crate::{
    schema::user_schema::{UserRegisterSchema, UserLoginSchema},
    model::{UserModel, UserListModel}
};

pub async fn user_register(body: web::Json<UserRegisterSchema>, 
                            identity: Option<Identity>,
                            data: web::Data<AppState>) -> ServerResponse {

    if let Some(_) = identity {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"status": "error", "message": "Cant register, already logged in"})
        ));
    }

    if let Ok(_) = UserModel::find_by_name(&data.db, &body.username).await {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"status": "error", "message": "User with this name already exists"})
        ));
    }

    let salt = b"SomeRandomSalt";
    let config = argon2::Config::rfc9106_low_mem();
    let hashed_password = argon2::hash_encoded(body.password.as_bytes(), salt, &config).unwrap();

    let new_user = UserModel::new(body.username.clone(), hashed_password);

    let result = new_user.insert(&data.db).await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok()
                    .json(serde_json::json!({"status": "success","data": "User successfully registered"}))),
        Err(error) => {
                Err(error.into())

        } 
    }


}

pub async fn user_login(request: HttpRequest, 
                        body: web::Json<UserLoginSchema>, 
                        identity: Option<Identity>,
                        data: web::Data<AppState>) -> ServerResponse {

    if let Some(_) = identity {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"status": "error", "message": "Already logged in"})
        ));
    }

    if body.username.is_empty() || body.password.is_empty() {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"status": "error", "message": "Missing username or password"})
        ));
    }

    let user = match UserModel::find_by_name(&data.db, &body.username).await {
        Ok(user) => user,
        Err(error) => {
            match error {
                DatabaseError::NotFound => {
                    return Ok(HttpResponse::BadRequest()
                        .json(serde_json::json!({"status": "error","message": "Invalid username or password"})));
                }
                _ => {
                    return Err(error.into());
                }
            }
        }
    };

    let password_valid = argon2::verify_encoded(&user.password, body.password.as_bytes())?;

    if password_valid {
        Identity::login(&request.extensions(), user.user_id.to_string()).unwrap(); 
        return Ok(HttpResponse::Ok().finish());
    }

    return Ok(HttpResponse::BadRequest()
        .json(serde_json::json!({"status": "error", "message": "Invalid username or password"})));

}

pub async fn user_list(data: web::Data<AppState>,
                        _: Identity) -> ServerResponse {
    let users = UserModel::list(&data.db).await?; 
    Ok(HttpResponse::Ok().json(users))
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

pub async fn user_hello(identity: Option<Identity>, 
                        data: web::Data<AppState>) -> ServerResponse {
    match identity {
        Some(identity) => {
            let user = UserModel::find_by_id(&data.db, identity.id().unwrap().parse().unwrap()).await?;
                Ok(HttpResponse::Ok().body(format!("Hello {}!", user.username)))
        }
        None => {
            Ok(HttpResponse::BadRequest().body("Hello Anonymous!"))
        }
    }
}

pub async fn list_roles() -> ServerResponse {
    Ok(HttpResponse::Ok().json(json!(
        {
            "status": "success",
            "data": [
                {
                    "id": 1,
                    "role": "user"
                },
                {
                    "id": 2,
                    "role": "admin"
                }
            ]
        }
    )))
}