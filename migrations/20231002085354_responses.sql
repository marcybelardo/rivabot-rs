-- Add migration script here
CREATE TABLE IF NOT EXISTS responses
(
    id SERIAL NOT NULL PRIMARY KEY,
    response_text TEXT NOT NULL,
    created_at TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    command_id INTEGER NOT NULL,

    CONSTRAINT fk_command FOREIGN KEY(command_id) REFERENCES commands(id)
);
