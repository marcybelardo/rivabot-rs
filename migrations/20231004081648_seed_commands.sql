-- Add migration script here
WITH commands_responses AS (
	INSERT INTO commands (command_text, description) VALUES ('hello', 'Say hello~') RETURNING id
)
INSERT INTO responses (response_text, command_id) SELECT 'Hi there <3', id FROM commands_responses;

