-- Add migration script here
CREATE TABLE user_v2 (
    -- Pkey
    id                          UUID         PRIMARY KEY, 
    -- Data
    user_name                VARCHAR(64)    NOT NULL UNIQUE,
    user_type                VARCHAR(16)    NOT NULL,
    created_at                  TIMESTAMP      NOT NULL DEFAULT now(),
    active                     boolean        NOT NULL DEFAULT true
);
CREATE INDEX user_v2_idx ON user_v2 USING hash (id);