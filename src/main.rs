use std::env;

use actix_web::{App, HttpResponse, HttpServer, web};
use serde_json::json;

mod cian;
mod router;
mod torgi;

mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if env::var("RUST_LOG").is_err() {
        unsafe { env::set_var("RUST_LOG", "info") }
    }
    env_logger::init();
    log::info!("Logger initialized");

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").configure(router::router))
            .route(
                "/health",
                web::get().to(|| async { HttpResponse::Ok().json(json!({ "status": "OK" })) }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
