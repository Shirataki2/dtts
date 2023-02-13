#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error from Discord connection: {0}")]
    Serenity(poise::serenity_prelude::SerenityError),
    #[error("Error from database: {0}")]
    Database(sqlx::Error),
    #[error("Guild only")]
    GuildOnly,
}

impl From<poise::serenity_prelude::SerenityError> for Error {
    fn from(e: poise::serenity_prelude::SerenityError) -> Self {
        error!("Serenity error: {:?}", e);
        Error::Serenity(e)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        error!("Database error: {:?}", e);
        Error::Database(e)
    }
}
