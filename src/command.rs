use sqlx::{postgres::PgPool, Error};
use twitch_irc::message::ServerMessage;

pub struct RivaCommand {
    pub id: i32,
    pub command_text: String,
    pub description: Option<String>,
}

impl RivaCommand {
    pub fn new() -> Self {
        Self {
            id: 500,
            command_text: String::new(),
            description: None,
        }
    }

    pub async fn get_reply(&self, pool: &PgPool, message: &ServerMessage) -> Result<String, Error> {
        match message {
            ServerMessage::Privmsg(msg) => {
                println!("MESSAGE TEXT: {}", &msg.message_text);
                let command = sqlx::query_as!(
                    RivaCommand,
                    r#"
                        SELECT id, command_text, description
                        FROM commands
                        WHERE command_text = $1
                    "#,
                    &msg.message_text
                )
                .fetch_one(pool)
                .await?;

                let reply = sqlx::query!(
                    "SELECT response_text FROM responses WHERE command_id = $1",
                    command.id
                )
                .fetch_one(pool)
                .await?;

                Ok(reply.response_text)
            }
            _ => Err(Error::RowNotFound),
        }
    }
}
