use actix_web::{http::StatusCode, web, HttpRequest};

use crate::error::create_error;

pub mod donate;
pub mod oauth2;
pub mod servers;
pub mod users;

pub fn get_data<T>(req: &HttpRequest) -> Result<&T, crate::error::Error>
where
    T: 'static,
{
    match req.app_data::<T>() {
        Some(data) => Ok(data),
        None => {
            error!("Failed to get application data: {}", stringify!(T));
            Err(create_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Setup Failed",
            ))
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/oauth2").configure(oauth2::init_routes));
    cfg.service(web::scope("/users").configure(users::init_routes));
    cfg.service(web::scope("/servers").configure(servers::init_routes));
    cfg.service(web::scope("/donate").configure(donate::init_routes));
}
