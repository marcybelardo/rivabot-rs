use async_trait::async_trait;
use sqlx::postgres::{PgPool, PgPoolOptions};

#[async_trait]
pub trait Storage {
    type Output;
    type Error;

    /// Read the data into the struct
    async fn read(&mut self) -> Result<(), Self::Error>;

    /// Write the data from the struct into the database
    async fn save(&self) -> Result<(), Self::Error>;
}

pub struct RivaDB {
    db_pool: PgPool,
}

impl RivaDB {
    pub async fn new(url: &str) -> Self {
        let db_pool = PgPoolOptions::new()
            .max_connections(20)
            .connect(url)
            .await
            .unwrap();
        sqlx::migrate!().run(&db_pool).await.unwrap();

        Self { db_pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.db_pool
    }
}
