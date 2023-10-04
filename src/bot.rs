use crate::{
    RivaTokenStore,
    RivaCommand,
};

use std::error::Error;
use twitch_irc::{
    login::RefreshingLoginCredentials,
    ClientConfig,
    SecureTCPTransport,
    TwitchIRCClient,
};

pub struct RivaBot {
    client_config: ClientConfig<RefreshingLoginCredentials<RivaTokenStore>>,
}

impl RivaBot {
    pub fn new(id: &str, secret: &str) -> Self {
        let credentials = RefreshingLoginCredentials::init(id.to_owned(), secret.to_owned(), RivaTokenStore::new());
        let client_config = ClientConfig::<RefreshingLoginCredentials<RivaTokenStore>>::new_simple(credentials);

        Self {
            client_config, 
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        let (mut incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, RefreshingLoginCredentials<RivaTokenStore>>::new(self.client_config);

        let reply_client = client.clone();
        let join_handle = tokio::spawn(async move {
            while let Some(message) = incoming_messages.recv().await {
                println!("{:#?}", message);
                let command = RivaCommand::new();
                if let Some(response) = command.get_reply(&message) {
                    reply_client.say("marivaaaa".to_owned(), response).await.unwrap();
                }
            }
        });

        let join_client = client.clone();
        join_client.join("marivaaaa".to_owned()).unwrap();

        join_handle.await.unwrap();

        Ok(())
    }
}

