-- Add migration script here
CREATE TABLE partners (
    -- Pkey
    id                          bigint         PRIMARY KEY, -- SNOWFLAKE ID
    -- Data
    partner_name                VARCHAR(64)    NOT NULL UNIQUE,
    partner_type                VARCHAR(16)    NOT NULL,
    created_at                  TIMESTAMP      NOT NULL DEFAULT now(),
    enabled                     boolean        NOT NULL DEFAULT true
);
CREATE INDEX partners_idx ON partners USING hash (id);