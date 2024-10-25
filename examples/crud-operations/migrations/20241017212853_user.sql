-- Add migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    me VARCHAR NOT NULL,
    age INTEGER
);
