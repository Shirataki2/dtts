use crate::prelude::*;

#[get("/billing_portal")]
pub async fn billing_portal(req: HttpRequest, user: User) -> Result<HttpResponse, Error> {
    let pool = get_data::<PgPool>(&req)?;
    let cfg = get_config(&req)?;
    let account = Account::optional_get(pool, user.id.0 as i64).await?;
    let account = match account {
        Some(a) => a,
        None => return Err(Error::bad_request("No customer")),
    };

    let customer_id = match account.customer_id {
        Some(c) => c,
        None => return Err(Error::bad_request("No customer")),
    };

    let stripe_client = get_stripe_client(&req)?;

    let form = BillingPortalSessionRequest {
        customer: customer_id,
        return_url: format!("{}/account", cfg.web_base_url.clone(),),
    };

    let url = "/billing_portal/sessions".to_string();
    let session = match stripe_client
        .post_form::<BillingPortalSession, _>(&url, form)
        .await
    {
        Ok(session) => session,
        Err(err) => {
            error!("{:?}", err);
            return Err(Error::bad_request(
                "Failed to create billing portal session",
            ));
        }
    };

    Ok(HttpResponse::SeeOther()
        .append_header((reqwest::header::LOCATION, session.url))
        .finish())
}

use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BillingPortalSession {
    pub id: String,
    pub configuration: Value,
    pub created: u64,
    pub customer: String,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<Box<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<Box<String>>,
    pub return_url: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BillingPortalSessionRequest {
    pub customer: String,
    pub return_url: String,
}
