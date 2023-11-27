use crate::{schema::kpi_schema::{KpiNewSchema, KpiUpdateSchema}, model::KpiModel};
use super::prelude::*;

#[derive(Deserialize)]
pub struct KpiQuery {
    pub kpi_id: i32,
}

#[utoipa::path(
    post,
    path = "/api/kpi",
    request_body = KpiNewSchema,
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
)]
pub async fn kpi_new(body: web::Json<KpiNewSchema>, 
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    let user_id: i32 = identity.id().unwrap().parse().unwrap();

    let new_kpi = KpiModel::new(body.parameter.clone(), 
                    body.limitation.clone(), 
                    body.value.clone(), 
                    user_id);
    
    new_kpi.insert(&data.db).await?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    delete,
    path = "/api/kpi/{kpi_id}",
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
    params(
            ("kpi_id" = i32, Path, description = ""),
        )
)]
pub async fn kpi_delete( query: web::Path<KpiQuery>,
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    let user_id: i32 = identity.id().unwrap().parse().unwrap();

    match KpiModel::get(&data.db, query.kpi_id).await {
        Ok(kpi) => {
            if kpi.owner_id != user_id {
                return Ok(HttpResponse::BadRequest().json(
                    serde_json::json!(ResponseError::KpiNotOwner.get_error())));
            }
        }
        Err(_) => {

        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!(ResponseError::KpiDoesntExist.get_error())));
        }
    }

    KpiModel::delete(&data.db, query.kpi_id).await?;
    
    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    post,
    path = "/api/kpi/{kpi_id}",
    request_body = KpiUpdateSchema,
    responses(
        (status = 200),
        (status = 400, body = ErrorModel),
        (status = 401),
    ),
    params(
            ("kpi_id" = i32, Path, description = ""),
        )
)]
pub async fn kpi_update(body: web::Json<KpiUpdateSchema>, 
                        query: web::Path<KpiQuery>,
                        identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    let user_id: i32 = identity.id().unwrap().parse().unwrap();


    let kpi = match KpiModel::get(&data.db, query.kpi_id).await {
        Ok(kpi) => {
            if kpi.owner_id != user_id {
                return Ok(HttpResponse::BadRequest().json(
                    serde_json::json!(ResponseError::KpiNotOwner.get_error())));
            }
            kpi
        }
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(
                serde_json::json!(ResponseError::KpiDoesntExist.get_error())));
        }
    };

    let kpi = KpiModel {
        parameter: body.parameter.clone(),
        limitation: body.limitation.clone(),
        value: body.value.clone(),
        ..kpi
    };

    kpi.update(&data.db, kpi.kpi_id).await?;
    
    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    get,
    path = "/api/kpi/list",
    responses(
        (status = 200, body = Vec<KpiModel>),
        (status = 400, body = ErrorModel),
        (status = 401),
    )
)]
pub async fn kpi_list(identity: Identity,
                        data: web::Data<AppState>) -> ServerResponse {

    let user_id: i32 = identity.id().unwrap().parse().unwrap();

    let kpi_list = KpiModel::get_all_user(&data.db, user_id).await?;

    
    Ok(HttpResponse::Ok().json(kpi_list))
}