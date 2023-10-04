use std::fs;
use chrono::{DateTime, Utc};
use async_trait::async_trait;
use serde_with::serde_as;
use serde::{Deserialize, Serialize};
use twitch_irc::login::{TokenStorage, UserAccessToken};

const FILENAME: &str = "rivabot.toml";

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct RivaTokenStore {
    pub access_token: String,
    pub refresh_token: String,
    #[serde(with = "toml_datetime_compat")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "toml_datetime_compat")]
    pub expires_at: DateTime<Utc>,
}

#[async_trait]
impl TokenStorage for RivaTokenStore {
    type LoadError = std::io::Error;
    type UpdateError = std::io::Error;

    async fn load_token(&mut self) -> Result<UserAccessToken, Self::LoadError> {
        let file = fs::read_to_string(FILENAME)?;
        let riva_token: RivaTokenStore = toml::from_str(&file).unwrap();

        Ok(UserAccessToken {
            access_token: riva_token.access_token,
            refresh_token: riva_token.refresh_token,
            created_at: riva_token.created_at,
            expires_at: Some(riva_token.expires_at),
        })
    }

    async fn update_token(&mut self, token: &UserAccessToken) -> Result<(), Self::UpdateError> {
        self.access_token = token.access_token.to_owned();
        self.refresh_token = token.refresh_token.to_owned();
        self.created_at = token.created_at;
        self.expires_at = token.expires_at.unwrap_or(DateTime::default());

        let file_to_write = toml::to_string(&self).unwrap();
        fs::write(FILENAME, &file_to_write)
    }
}

impl RivaTokenStore {
    pub fn new() -> Self {
        Self {
            access_token: String::new(),
            refresh_token: String::new(),
            created_at: DateTime::default(),
            expires_at: DateTime::default(),
        }
    }
}

