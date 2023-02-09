use actix_web::HttpResponse;

use crate::{error::Error, user::User};

#[get("/me")]
pub async fn me(user: User) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(user.inner()))
}
