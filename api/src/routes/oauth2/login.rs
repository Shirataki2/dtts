use actix_session::Session;
use actix_web::{http::header, HttpRequest, HttpResponse};

use crate::{error::Error, routes::get_data, OAuth2Client};

#[get("/login")]
pub async fn login(sess: Session, req: HttpRequest) -> Result<HttpResponse, Error> {
    let auth = get_data::<OAuth2Client>(&req)?;
    let pkce = auth.create_pkce_auth_url();
    sess.insert("verifier", pkce.verifier)?;
    sess.insert("csrf_token", pkce.csrf_token)?;
    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, pkce.auth_url.to_string()))
        .finish())
}
