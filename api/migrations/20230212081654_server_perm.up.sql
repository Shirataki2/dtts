-- Add up migration script here
CREATE TABLE server_permission (
    guild_id BIGINT NOT NULL,
    tag TEXT NOT NULL,
    permission_bit BIGINT NOT NULL,
    PRIMARY KEY (guild_id, tag),
    FOREIGN KEY (guild_id) REFERENCES guild(guild_id) ON DELETE CASCADE
);
