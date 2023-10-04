-- Add migration script here
CREATE TABLE IF NOT EXISTS commands
(
    id SERIAL NOT NULL PRIMARY KEY,
    command_text TEXT UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
);

