-- Add up migration script here
CREATE TABLE dictionary (
    guild_id BIGINT NOT NULL REFERENCES guild(guild_id),
    dict TEXT NOT NULL,
    CONSTRAINT dictionary_pk PRIMARY KEY (guild_id)
);
