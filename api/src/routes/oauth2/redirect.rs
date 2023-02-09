use actix_session::Session;
use actix_web::{http::header, web, HttpRequest, HttpResponse};
use oauth2::{reqwest::async_http_client, AuthorizationCode, CsrfToken, PkceCodeVerifier};

use crate::{error::Error, routes::get_data, OAuth2Client};

#[derive(Deserialize, Debug)]
pub struct AuthRequestQuery {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[get("/redirect")]
pub async fn redirect(
    sess: Session,
    req: HttpRequest,
    query: web::Query<AuthRequestQuery>,
) -> Result<HttpResponse, Error> {
    let (code, state) = match (query.code.clone(), query.state.clone()) {
        (Some(code), Some(state)) => (code, state),
        _ => {
            info!(
                "Error in OAuth2 redirect: {:?} (Desc: {:?})",
                query.error, query.error_description
            );
            return Ok(HttpResponse::Found()
                .append_header((header::LOCATION, "/"))
                .finish());
        }
    };
    let code = AuthorizationCode::new(code.clone());
    let csrf_token = CsrfToken::new(state.clone());
    let expected_csrf_token = match sess.get::<CsrfToken>("csrf_token") {
        Ok(Some(token)) => token,
        other => {
            info!("No CSRF token found in session: {:?}", other);
            return Err(Error::Unauthorized(
                "No CSRF token found in session".to_string(),
            ));
        }
    };
    if csrf_token.secret() != expected_csrf_token.secret() {
        info!("CSRF token mismatch");
        return Err(Error::Unauthorized("CSRF token mismatch".to_string()));
    }
    let pkce_verifier = match sess.get::<PkceCodeVerifier>("verifier") {
        Ok(Some(token)) => token,
        other => {
            info!("No PKCE verifier found in session: {:?}", other);
            return Err(Error::Unauthorized(
                "No PKCE verifier found in session".to_string(),
            ));
        }
    };
    let auth_client = get_data::<OAuth2Client>(&req)?;
    let token_result = auth_client
        .client
        .exchange_code(code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await;
    let token = match token_result {
        Ok(token) => token,
        Err(e) => {
            info!("Error exchanging code for token: {:?}", e);
            return Err(Error::Unauthorized(
                "Error exchanging code for token".to_string(),
            ));
        }
    };

    info!("Got token: {:?}", token);
    sess.insert("token", &token)?;
    info!("Inserted token into session");

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, "/"))
        .finish())
}
