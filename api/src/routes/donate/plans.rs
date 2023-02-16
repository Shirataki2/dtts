use crate::{prelude::*, routes::donate::get_stripe_client};

#[get("/plans")]
pub async fn plans(req: HttpRequest) -> Result<HttpResponse, Error> {
    let client = get_stripe_client(&req)?;
    let config = crate::config::StripeConfig::load().await?;
    let donate = match config.products.get("donation") {
        Some(prod) => prod,
        None => {
            return Err(create_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "No donation product configured",
            ))
        }
    };
    let plans = stripe::Price::list(
        client,
        &stripe::ListPrices {
            active: Some(true),
            product: Some(stripe::IdOrCreate::Id(donate.id.as_str())),
            ..Default::default()
        },
    )
    .await?;
    Ok(HttpResponse::Ok().json(plans))
}
