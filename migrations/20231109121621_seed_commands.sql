-- Add migration script here
INSERT INTO users (username) VALUES ('marivaaaa') RETURNING id;

INSERT INTO auths (user_id, access_token, refresh_token, created_at, expires_at)
VALUES
(
	id,
	'3yepaykjbbsd8qg27ci2j6ic1n4v9u',
	'zwgofju996d7rkdy1eixrtn64f16tk7kpfqxok0a69ohtbnjyh',
	'2023-10-15T21:00:44.64066Z'::TIMESTAMPTZ,
	'2023-10-16T01:24:16.64066Z'::TIMESTAMPTZ
);

INSERT INTO commands(user_id, text, description, response)
VALUES
(
	id,
	'hello',
	'Say hello!',
	'Hi there <3'
);
