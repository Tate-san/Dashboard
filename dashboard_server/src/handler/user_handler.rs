use super::prelude::*;
use actix_web::{HttpRequest, HttpMessage};
use argon2::{self};
use crate::{
    schema::user_schema::{UserRegisterSchema, UserLoginSchema},
    model::{UserModel, UserListModel, RoleListModel}
};

#[utoipa::path(
    post,
    path = "/api/user/register",
    request_body = UserRegisterSchema,
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
    )
)]
pub async fn user_register(body: web::Json<UserRegisterSchema>, 
                            identity: Option<Identity>,
                            data: web::Data<AppState>) -> ServerResponse {

    if let Some(_) = identity {
        return Ok(HttpResponse::BadRequest()
                    .json(serde_json::json!(ResponseError::UserLoggedIn.get_error())));
    }

    if let Ok(_) = UserModel::find_by_name(&data.db, &body.username).await {
        return Ok(HttpResponse::BadRequest()
                    .json(serde_json::json!(ResponseError::UserAlreadyExists.get_error())));
    }

    let salt = b"SomeRandomSalt";
    let config = argon2::Config::rfc9106_low_mem();
    let hashed_password = argon2::hash_encoded(body.password.as_bytes(), salt, &config).unwrap();

    let new_user = UserModel::new(body.username.clone(), hashed_password);

    let result = new_user.insert(&data.db).await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(error) => {
                Err(error.into())

        } 
    }


}

#[utoipa::path(
    post,
    path = "/api/user/login",
    request_body = UserLoginSchema,
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
    )
)]
pub async fn user_login(request: HttpRequest, 
                        body: web::Json<UserLoginSchema>, 
                        identity: Option<Identity>,
                        data: web::Data<AppState>) -> ServerResponse {

    if let Some(_) = identity {
        return Ok(HttpResponse::BadRequest()
                    .json(serde_json::json!(ResponseError::UserLoggedIn.get_error())));
    }

    if body.username.is_empty() || body.password.is_empty() {
        return Ok(HttpResponse::BadRequest()
                    .json(serde_json::json!(ResponseError::UserLoginInvalid.get_error())));
    }

    let user = match UserModel::find_by_name(&data.db, &body.username).await {
        Ok(user) => user,
        Err(error) => {
            match error {
                DatabaseError::NotFound => {
                    return Ok(HttpResponse::BadRequest()
                                .json(serde_json::json!(ResponseError::UserLoginInvalid.get_error())));
            
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
                .json(serde_json::json!(ResponseError::UserLoginInvalid.get_error())));

}

#[utoipa::path(
    get,
    path = "/api/user/list",
    responses(
        (status = 200, body = Vec<UserListModel>),
        (status = 401),
    )
)]
pub async fn user_list(data: web::Data<AppState>,
                        _: Identity) -> ServerResponse {
    let users = UserModel::list(&data.db).await?; 
    Ok(HttpResponse::Ok().json(users))
}

#[utoipa::path(
    get,
    path = "/api/user/logout",
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
    )
)]
pub async fn user_logout(identity: Identity) -> ServerResponse {
    identity.logout();
    Ok(HttpResponse::Ok().finish())
}


#[utoipa::path(
    get,
    path = "/api/user/hello",
    responses(
        (status = 200),
    )
)]
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

#[utoipa::path(
    get,
    path = "/api/user/roles",
    responses(
        (status = 200, body = Vec<RoleListModel>),
    )
)]
pub async fn list_roles() -> ServerResponse {
    Ok(HttpResponse::Ok().json(
        vec![
            RoleListModel{
                id: 1,
                name: "user".to_string()
            },
            RoleListModel{
                id: 2,
                name: "admin".to_string()
            },
        ]
    ))
}