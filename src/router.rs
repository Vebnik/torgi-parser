use actix_web::web;

use crate::torgi;

pub fn router(cfg: &mut web::ServiceConfig) {
    // cian scope
    cfg.route("/parse/cian/{query}/{count}", web::get().to(torgi::search));

    // torgi scope
    cfg.route("/parse/torgi/{query}/{count}", web::get().to(torgi::search));
}
