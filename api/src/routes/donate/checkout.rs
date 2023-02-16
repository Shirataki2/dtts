use std::str::FromStr;

use stripe::PriceId;

use crate::{prelude::*, routes::donate::get_stripe_client};

#[derive(Deserialize)]
pub struct CheckoutQuery {
    pub plan: PriceId,
}

#[get("/checkout")]
pub async fn checkout(
    req: HttpRequest,
    user: User,
    query: Query<CheckoutQuery>,
) -> Result<HttpResponse, Error> {
    let cfg = get_config(&req)?;
    let client = get_stripe_client(&req)?;
    let price = stripe::Price::retrieve(client, &query.plan, &[]).await?;
    let recurring = price.recurring.is_some();
    let card = stripe::CreateCheckoutSessionPaymentMethodTypes::Card;
    let base_url = &cfg.web_base_url;
    let cancel_url = format!("{}/donate/cancel", base_url);
    let success_url = format!(
        "{}/donate/success?session_id={{CHECKOUT_SESSION_ID}}&price_id={}",
        base_url, price.id
    );
    let mut params = stripe::CreateCheckoutSession::new(&success_url);
    let item = stripe::CreateCheckoutSessionLineItems {
        price: Some(price.id.to_string()),
        quantity: Some(1),
        ..Default::default()
    };
    params.line_items = Some(vec![item]);
    params.payment_method_types = Some(vec![card]);
    params.cancel_url = Some(&cancel_url);
    if recurring {
        params.mode = Some(stripe::CheckoutSessionMode::Subscription);
    } else {
        params.mode = Some(stripe::CheckoutSessionMode::Payment);
    }

    let pool = get_data::<PgPool>(&req)?;
    let account = Account::optional_get(pool, user.id.0 as i64).await?;
    if let Some(account) = account {
        if let Some(customer_id) = account.customer_id {
            let customer_id = match stripe::CustomerId::from_str(&customer_id) {
                Ok(id) => id,
                Err(_) => {
                    return Err(create_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Invalid customer id",
                    ))
                }
            };
            params.customer = Some(customer_id);
        }
    }

    let session = stripe::CheckoutSession::create(client, params).await?;
    let session_url = match session.url {
        Some(url) => url,
        None => {
            return Err(create_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "No session url",
            ))
        }
    };

    Ok(HttpResponse::SeeOther()
        .append_header((reqwest::header::LOCATION, session_url))
        .finish())
}
