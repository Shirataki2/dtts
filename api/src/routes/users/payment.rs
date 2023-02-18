use crate::prelude::*;

#[get("/payment")]
pub async fn payment(req: HttpRequest, user: User) -> Result<HttpResponse, Error> {
    let pool = get_data::<PgPool>(&req)?;
    let payments = Payment::get_user_payments(pool, user.id.0 as i64).await?;
    Ok(HttpResponse::Ok().json(payments))
}
