pub mod dict;

use crate::prelude::*;

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

/// Registers or unregisters application commands in this guild or globally
#[poise::command(prefix_command, hide_in_help)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

pub fn list_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        test(),
        help(),
        register(),
        poise::Command {
            subcommands: vec![dict::list(), dict::add()],
            ..dict::dict()
        },
    ]
}
