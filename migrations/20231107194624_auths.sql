-- Add migration script here
CREATE TABLE IF NOT EXISTS auths (
	id SERIAL PRIMARY KEY,
	user_id INT NOT NULL REFERENCES users(id),
	access_token VARCHAR(255) NOT NULL,
	refresh_token VARCHAR(255) NOT NULL,
	created_at TIMESTAMPTZ NOT NULL,
	expires_at TIMESTAMPTZ
);
