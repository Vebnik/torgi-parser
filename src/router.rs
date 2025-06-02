use actix_web::web;

use crate::cian;
use crate::torgi;

pub fn router(cfg: &mut web::ServiceConfig) {
    // cian scope
    cfg.route(
        "/parse/cian/{pages}/{region}/{type}/{object}",
        web::get().to(cian::search),
    );

    // torgi scope
    cfg.route("/parse/torgi/{query}/{count}", web::get().to(torgi::search));
}
