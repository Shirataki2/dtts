use crate::prelude::*;

/// 辞書関連
#[poise::command(prefix_command, slash_command, check = "check::guild_only")]
pub async fn dict(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// 登録した辞書の一覧を表示します
#[poise::command(prefix_command, slash_command, check = "check::guild_only")]
pub async fn list(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// 辞書に新しい単語を追加します
#[poise::command(prefix_command, slash_command, check = "check::guild_only")]
pub async fn add(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
