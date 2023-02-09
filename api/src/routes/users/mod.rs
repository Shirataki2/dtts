use actix_web::web;

pub mod me;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(me::me);
}
