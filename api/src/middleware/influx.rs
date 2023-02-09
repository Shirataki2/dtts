use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use futures::future::LocalBoxFuture;
use reqwest::StatusCode;
use std::future::{ready, Ready};

use crate::logs::{create_influx_client, AccessMetrics};

pub struct Influx;

impl<S, B> Transform<S, ServiceRequest> for Influx
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = InfluxMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(InfluxMiddleware { service }))
    }
}

pub struct InfluxMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for InfluxMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_string();

        let fut = self.service.call(req);

        Box::pin(async move {
            let timer = tokio::time::Instant::now();
            let resp = fut.await?;
            let status = resp.status();
            if let Some(error) = resp.response().error() {
                error!(
                    "Request to {} failed with status {}: {:?}",
                    path, status, error
                );
            }
            let elapsed = timer.elapsed().as_millis() as i64;
            if status == StatusCode::NOT_FOUND {
                return Ok(resp);
            }
            let client = match create_influx_client() {
                Ok(client) => client,
                Err(e) => {
                    error!("Failed to create influx client: {:?}", e);
                    return Ok(resp);
                }
            };
            if let Err(e) = AccessMetrics::save(&client, path, elapsed).await {
                error!("Failed to save access metrics: {:?}", e);
            }

            Ok(resp)
        })
    }
}
