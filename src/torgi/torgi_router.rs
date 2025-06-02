use actix_web::{HttpResponse, Responder, web};
use serde_json::json;

use super::Lot;

#[allow(dead_code)]
pub async fn search(path: web::Path<(String, i32)>) -> impl Responder {
    let (query, pages) = path.into_inner();

    log::info!("Search request with {query} - {pages}");

    let lots = Lot::search(query, pages).await;

    match lots {
        Ok(lots) => HttpResponse::Ok().json(lots),
        Err(err) => HttpResponse::InternalServerError().json(json!({ "error": err })),
    }
}
