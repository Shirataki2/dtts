use actix_web::web;

pub mod dict;
pub mod index;
pub mod list;
pub mod perms;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index);
    cfg.service(list::list);
    cfg.service(dict::get_dict);
    cfg.service(dict::post_dict);
    cfg.service(perms::get_perms);
    cfg.service(perms::patch_perms);
    cfg.service(perms::test);
    cfg.service(perms::check_perms);
}
