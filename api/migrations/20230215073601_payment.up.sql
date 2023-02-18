-- Add up migration script here
CREATE TABLE account (
    id BIGINT PRIMARY KEY,
    customer_id VARCHAR(32)
);

CREATE TABLE payment (
    account_id BIGINT NOT NULL REFERENCES account(id),
    name VARCHAR(64) NOT NULL,
    price INT NOT NULL,
    session_id VARCHAR(128) NOT NULL,
    price_id VARCHAR(32) NOT NULL,
    subscription_id VARCHAR(64),
    created_at TIMESTAMP NOT NULL,
    CONSTRAINT payment_account_id_session_id_key UNIQUE (account_id, session_id),
    PRIMARY KEY (account_id, session_id)
);
