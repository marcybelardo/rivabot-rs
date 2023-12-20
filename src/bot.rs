use crate::{
    RivaToken,
    RivaCommand,
    RivaDB,
    RivaUser,
};

use std::env;
use anyhow::Result;
use dotenvy;
use twitch_irc::{
    login::RefreshingLoginCredentials,
    ClientConfig,
    SecureTCPTransport,
    TwitchIRCClient,
};

pub struct RivaBot {
    settings: RivaBotSettings,
    user: RivaUser,
    db: RivaDB,
}

pub struct RivaBotSettings {
    host_name: String,
    watched_channel: String,
    client_id: String,
    client_secret: String,
    db_url: String,
}

impl RivaBot {
    pub async fn new(settings: RivaBotSettings, db: RivaDB, user: RivaUser) -> Self {
        Self {
            settings,
            db,
            user,
        }
    }

    pub async fn run(self) -> Result<()> {
        let credentials = RefreshingLoginCredentials::init(
            self.settings.client_id,
            self.settings.client_secret,
            RivaToken::new(self.settings.host_name, self.db.pool().clone())
        );
        let client_config = ClientConfig::<RefreshingLoginCredentials<RivaToken>>::new_simple(credentials);

        let (mut incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, RefreshingLoginCredentials<RivaToken>>::new(client_config);

        let reply_client = client.clone();
        let join_handle = tokio::spawn(async move {
            while let Some(message) = incoming_messages.recv().await {
                println!("{:#?}", message);
                let command = RivaCommand::new();
                match command.get_reply(self.db.pool(), &message).await {
                    Ok(response) => reply_client.say("marivaaaa".to_owned(), response).await.unwrap(),
                    Err(e) => eprintln!("Error: {e}"),
                }
            }
        });

        let join_client = client.clone();
        join_client.join("marivaaaa".to_owned()).unwrap();

        join_handle.await.unwrap();

        Ok(())
    }
}

impl RivaBotSettings {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();
        let host_name = env::var("HOST_NAME").unwrap();
        let watched_channel = env::var("WATCHED_CHANNEL").unwrap();
        let client_id = env::var("CLIENT_ID").unwrap();
        let client_secret = env::var("CLIENT_SECRET").unwrap();
        let db_url = env::var("DATABASE_URL").unwrap();

        Self {
            host_name,
            watched_channel,
            client_id,
            client_secret,
            db_url,
        }
    }
}

