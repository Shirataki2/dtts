use crate::{Context, Error};

/// Test command
#[poise::command(prefix_command, slash_command)]
pub async fn test(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("test").await?;
    Ok(())
}

/// Test command
#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "\
Type ?help command for more info on a command.
You can edit your message to the bot and the bot will edit its response.",
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}
