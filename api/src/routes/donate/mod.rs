pub mod checkout;
pub mod plans;
pub mod register;

use crate::prelude::*;

pub fn get_stripe_client(req: &HttpRequest) -> Result<&stripe::Client, Error> {
    let data = get_data::<stripe::Client>(req)?;
    Ok(data)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(plans::plans);
    cfg.service(checkout::checkout);
    cfg.service(register::register);
}
