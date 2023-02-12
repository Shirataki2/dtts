#![allow(dead_code)]
use crate::prelude::*;

pub async fn guild_only(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx.guild().is_some())
}
