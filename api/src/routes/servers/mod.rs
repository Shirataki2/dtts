use actix_web::web;

pub mod dict;
pub mod index;
pub mod list;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index);
    cfg.service(list::list);
    cfg.service(dict::get_dict);
    cfg.service(dict::post_dict);
}
