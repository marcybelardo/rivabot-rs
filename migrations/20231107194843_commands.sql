-- Add migration script here
CREATE TABLE IF NOT EXISTS commands (
	id SERIAL PRIMARY KEY,
	user_id INT NOT NULL REFERENCES users(id),
	text TEXT NOT NULL,
	description TEXT,
	response TEXT,
	created_at TIMESTAMPTZ NOT NULL
);
