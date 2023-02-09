use actix_web::web;

pub mod login;
pub mod logout;
pub mod redirect;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login::login);
    cfg.service(logout::logout);
    cfg.service(redirect::redirect);
}
