use actix_session::Session;
use actix_web::{http::StatusCode, FromRequest};

use crate::{
    error::{create_error, Error},
    serenity, AccessToken, BoxFutureResult,
};
use std::{ops, sync::Arc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User(serenity::CurrentUser);

impl User {
    pub fn inner(&self) -> &serenity::CurrentUser {
        &self.0
    }
}

impl ops::Deref for User {
    type Target = serenity::CurrentUser;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn get_access_token(sess: &Session) -> Result<AccessToken, crate::error::Error> {
    match sess.get::<AccessToken>("token") {
        Ok(Some(token)) => Ok(token),
        Ok(None) => Err(crate::error::Error::Unauthorized("No Access Token".into())),
        Err(e) => {
            error!("Failed to get access token: {:?}", e);
            Err(create_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Setup Failed",
            ))
        }
    }
}

impl FromRequest for User {
    type Error = Error;
    type Future = BoxFutureResult<Self, Self::Error>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let req2 = Arc::new(req.clone());
        Box::pin(async move {
            let sess = actix_session::Session::extract(&req2).await?;
            if let Some(user) = sess.get::<serenity::CurrentUser>("user")? {
                Ok(User(user))
            } else {
                let token = get_access_token(&sess)?;
                let client = crate::client::Client::from_token(&token)?;
                let user = client.current_user().await?;
                sess.insert("user", &user)?;
                Ok(User(user))
            }
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserToken(crate::AccessToken);

impl UserToken {
    pub fn inner(&self) -> &crate::AccessToken {
        &self.0
    }
}

impl FromRequest for UserToken {
    type Error = Error;
    type Future = BoxFutureResult<Self, Self::Error>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let req2 = Arc::new(req.clone());
        Box::pin(async move {
            let sess = actix_session::Session::extract(&req2).await?;
            let token = get_access_token(&sess)?;
            Ok(UserToken(token))
        })
    }
}
