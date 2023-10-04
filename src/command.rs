use twitch_irc::message::{ServerMessage, PrivmsgMessage};

pub struct RivaCommand {
    pub command_text: String,
    pub description: Option<String>,
}

impl RivaCommand {
    pub fn new() -> Self {
        Self {
            command_text: String::new(),
            description: None,
        }
    }

    pub fn get_reply(&self, message: &ServerMessage) -> Option<String> {
        match message {
            ServerMessage::Privmsg(msg) => {
                if let Some(response) = Self::parse_private_message(msg) {
                    Some(response.to_owned())
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    fn parse_private_message(priv_msg: &PrivmsgMessage) -> Option<&str> {
        match priv_msg.message_text.as_ref() {
            "!hello" => Some("Hi there <3"),
            _ => None,
        }
    }
}
