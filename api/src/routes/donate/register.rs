use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterQuery {
    pub session_id: String,
    pub price_id: stripe::PriceId,
}

#[get("/register")]
pub async fn register(
    req: HttpRequest,
    user: User,
    sess: Session,
    query: Query<RegisterQuery>,
) -> Result<HttpResponse, Error> {
    let client = get_stripe_client(&req)?;
    let url = format!("/checkout/sessions/{}", query.session_id);
    let session = client.get::<stripe::CheckoutSession>(&url).await?;
    let session_id = session.id.as_str();

    debug!("session: {:#?}", session);

    // customer returns Some if the session mode is payment or subscription
    let customer = match session.customer.as_ref() {
        Some(c) => c,
        None => return Err(Error::bad_request("Invalid session")),
    };

    let pool = get_data::<PgPool>(&req)?;
    match Account::optional_get(pool, user.id.0 as i64).await? {
        Some(mut account) => {
            account.customer_id = Some(customer.id().to_string());
            account.update(pool).await?;
        }
        None => {
            let new_account = Account {
                id: user.id.0 as i64,
                customer_id: Some(customer.id().to_string()),
            };
            new_account.create(pool).await?;
        }
    };
    sess.insert("session_id", session_id)?;

    let is_subscribed = match session.mode {
        stripe::CheckoutSessionMode::Payment => false,
        stripe::CheckoutSessionMode::Subscription => true,
        _ => false,
    };

    let subscription_id = if is_subscribed {
        let subscription = match session.subscription.as_ref() {
            Some(s) => s,
            None => return Err(Error::bad_request("Invalid session")),
        };
        Some(subscription.id().to_string())
    } else {
        None
    };

    let price_id = query.price_id.to_string();

    let payment = Payment {
        account_id: user.id.0 as i64,
        session_id: session_id.to_string(),
        price_id,
        subscription_id,
        created_at: chrono::Utc::now().naive_utc(),
    };
    payment.create(pool).await?;

    Ok(HttpResponse::Ok().json(payment))
}
