use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{query, query_as, PgPool};
use twitch_irc::login::{TokenStorage, UserAccessToken};

use crate::Storage;

#[derive(Debug)]
pub struct RivaToken {
    store: RivaTokenStore,
    host: String,
    db_pool: PgPool,
}

#[derive(Debug)]
struct RivaTokenStore {
    access_token: String,
    refresh_token: String,
    created_at: DateTime<Utc>,
    expires_at: Option<DateTime<Utc>>,
}

#[async_trait]
impl TokenStorage for RivaToken {
    type LoadError = sqlx::Error;
    type UpdateError = sqlx::Error;

    async fn load_token(&mut self) -> Result<UserAccessToken, Self::LoadError> {
        self.store.load(&self.host, self.db_pool).await?;

        Ok(UserAccessToken {
            access_token: self.store.access_token,
            refresh_token: self.store.refresh_token,
            created_at: self.store.created_at,
            expires_at: self.store.expires_at,
        })
    }

    async fn update_token(&mut self, token: &UserAccessToken) -> Result<(), Self::UpdateError> {
        self.store.access_token = token.access_token;
        self.store.refresh_token = token.refresh_token;
        self.store.created_at = token.created_at;
        self.store.expires_at = token.expires_at;

        self.store.save(&self.host, self.db_pool).await?;

        Ok(())
    }
}

#[async_trait]
impl Storage for RivaToken {
    type Output = RivaTokenStore;
    type Error = sqlx::Error;

    async fn read(&mut self) -> Result<(), Self::Error> {
        let user = query!("SELECT id FROM users WHERE name = $1", self.host)
            .fetch_one(self.db_pool)
            .await?;

        let data = query_as!(
            RivaTokenStore,
            r#"
                SELECT access_token, refresh_token, created_at, expires_at
                FROM tokens
                WHERE user_id = $1
            "#,
            user.id
        )
        .fetch_one(self.db_pool)
        .await?;

        self.store.access_token = data.access_token;
        self.store.refresh_token = data.refresh_token;
        self.store.created_at = data.created_at;
        self.store.expires_at = data.expires_at;

        Ok(())
    }

    async fn save(&self) -> Result<(), Self::Error> {
        query!(
            r#"
                UPDATE tokens SET
                access_token = $1,
                refresh_token = $2,
                created_at = $3,
                expires_at = $4
                WHERE name = $5
            "#,
            self.token.access_token,
            self.token.refresh_token,
            self.token.created_at,
            self.token.expires_at,
            self.host,
        )
        .execute(&self.db_pool)
        .await?
    }
}

impl RivaToken {
    pub fn new(host: String, pool: PgPool) -> Self {
        Self {
            store: RivaTokenStore::new(),
            host,
            db_pool: pool,
        }
    }
}

impl RivaTokenStore {
    pub fn new() -> Self {
        Self {
            access_token: String::new(),
            refresh_token: String::new(),
            created_at: DateTime::default(),
            expires_at: None,
        }
    }

    async fn load(&mut self, host: &str, pool: PgPool) -> Result<(), sqlx::Error> {
    }

    async fn save(&self, host: &str, pool: PgPool) -> Result<(), sqlx::Error> {}
}
