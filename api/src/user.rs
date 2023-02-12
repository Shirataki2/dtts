use actix_session::Session;
use actix_web::{http::StatusCode, FromRequest, HttpRequest};

use crate::prelude::*;
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

async fn get_user_from_session(req: HttpRequest) -> Result<serenity::CurrentUser, Error> {
    let sess = actix_session::Session::extract(&req).await?;
    if let Some(user) = sess.get::<serenity::CurrentUser>("user")? {
        Ok(user)
    } else {
        let token = get_access_token(&sess)?;
        let client = crate::client::Client::from_token(&token)?;
        let user = client.current_user().await?;
        sess.insert("user", &user)?;
        Ok(user)
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
            let user = get_user_from_session(req2.as_ref().clone()).await?;
            Ok(User(user))
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member(serenity::PartialMember);

impl Member {
    pub fn inner(&self) -> &serenity::PartialMember {
        &self.0
    }
}

impl ops::Deref for Member {
    type Target = serenity::PartialMember;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest for Member {
    type Error = Error;
    type Future = BoxFutureResult<Self, Self::Error>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let req2 = Arc::new(req.clone());
        Box::pin(async move {
            let user = get_user_from_session(req2.as_ref().clone()).await?;
            let config = get_config(&req2)?;
            let bot_token = config.discord_bot_token.clone();
            let client = crate::client::Client::from_bot_token(&bot_token)?;

            let q = req2.query_string();
            let guild_id = q
                .split('&')
                .find_map(|s| {
                    let mut split = s.split('=');
                    if split.next() == Some("id") {
                        split.next().map(|s| s.parse::<i64>().ok())
                    } else {
                        None
                    }
                })
                .flatten();
            let guild_id = match guild_id {
                Some(id) => id,
                None => {
                    return Err(create_error(
                        StatusCode::BAD_REQUEST,
                        "Missing id query parameter",
                    ))
                }
            };
            let user_id = user.id.0 as i64;

            let member = client.get_member(guild_id, user_id).await.map_err(|e| {
                error!("Failed to get member: {:?}", e);
                create_error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to get member")
            })?;

            let member = match member {
                Some(member) => member,
                None => {
                    return Err(create_error(
                        StatusCode::UNAUTHORIZED,
                        "You are not a member of this guild",
                    ))
                }
            };

            Ok(Member(member))
        })
    }
}

impl Member {
    pub async fn get_perm_bits(&self, guild_id: i64, client: &Client) -> u64 {
        let guild = client.get_server(guild_id).await;
        let guild = match guild {
            Ok(Some(guild)) => guild,
            Ok(None) => {
                error!("Guild not found");
                return 0;
            }
            Err(e) => {
                error!("Failed to get guild: {:?}", e);
                return 0;
            }
        };
        debug!("User: {:?}", &self.inner().user);
        let is_owner = guild.owner_id.0
            == self
                .inner()
                .user
                .as_ref()
                .map(|u| u.id.0)
                .unwrap_or_default();
        if is_owner {
            // Pass all permissions
            return 8;
        }
        let guild_roles = guild.roles;
        debug!("Guild roles: {:?}", guild_roles);
        let user_roles = &self.roles;
        debug!("User roles: {:?}", user_roles);
        let mut perms = 0;
        for role in user_roles {
            let role = guild_roles.get(role);
            let perm_bits = role.map(|r| r.permissions.bits()).unwrap_or_default();
            perms |= perm_bits;
        }
        debug!("User perms: {:#b}", perms);
        perms
    }

    pub async fn is_server_mod(&self, guild_id: i64, client: &Client) -> bool {
        let perms = self.get_perm_bits(guild_id, client).await;
        let is_admin = perms & 0x8 != 0;
        let is_mod = perms & 0x20 != 0;
        is_admin || is_mod
    }
}
