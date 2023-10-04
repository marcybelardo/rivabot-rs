use rivabot_rs::bot::RivaBot;

use std::{
    env,
    error::Error,
    process::exit,
};
use dotenvy;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let client_id = env::var("CLIENT_ID")?;
    let client_secret = env::var("CLIENT_SECRET")?;

    let rivabot = RivaBot::new(&client_id, &client_secret);

    if let Err(run) = rivabot.run().await {
        eprintln!("Error running rivabot: {run}");
        exit(1);
    }

    Ok(())
}

