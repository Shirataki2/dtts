use actix_web::web;

pub mod billing_portal;
pub mod me;
pub mod payment;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(me::me);
    cfg.service(payment::payment);
    cfg.service(billing_portal::billing_portal);
}
