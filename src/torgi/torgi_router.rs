use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde_json::json;

use super::Lot;

#[allow(dead_code)]
pub async fn search(path: web::Path<(String, i32)>) -> impl Responder {
    let (query, count) = path.into_inner();

    log::info!("Search request with {query} - {count}");

    let lots = Lot::search(query, count).await;

    match lots {
        Ok(lots) => HttpResponse::Ok().json(lots),
        Err(err) => HttpResponse::InternalServerError().json(json!({ "error": err })),
    }
}

#[allow(dead_code)]
pub async fn get_by_id(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json("")
}
