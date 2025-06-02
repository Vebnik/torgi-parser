use actix_web::{HttpResponse, Responder, web};
use serde_json::json;

use super::Lot;

pub async fn search(path: web::Path<(i32, i32, String, i32)>) -> impl Responder {
    let (pages, region, _type, object) = path.into_inner();

    log::info!("Search request with {pages} - {region} - {_type} - {object}");

    let lots = Lot::search(_type, pages, region, object).await;

    match lots {
        Ok(lots) => HttpResponse::Ok().json(lots),
        Err(err) => HttpResponse::InternalServerError().json(json!({ "error": err })),
    }
}
