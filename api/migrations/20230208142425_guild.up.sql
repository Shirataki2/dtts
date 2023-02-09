-- Add up migration script here
CREATE TABLE guild (
    guild_id BIGINT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    icon_url TEXT
);
