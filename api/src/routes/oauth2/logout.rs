use std::sync::Arc;

use actix_session::Session;
use actix_web::{http::header, HttpRequest, HttpResponse};

use crate::{config::ServerConfig, error::Error, routes::get_data};

#[get("/logout")]
pub async fn logout(sess: Session, req: HttpRequest) -> Result<HttpResponse, Error> {
    let conf = get_data::<Arc<ServerConfig>>(&req)?;
    sess.clear();
    let redirect_url = conf.web_base_url.clone();
    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, redirect_url))
        .finish())
}
