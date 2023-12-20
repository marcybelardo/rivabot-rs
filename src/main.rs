use rivabot_rs::{
    bot::{
        RivaBot,
        RivaBotSettings
    },
    RivaDB,
    RivaUser,
};

use std::process::exit;

#[tokio::main]
async fn main() {
    let settings = RivaBotSettings::new();
    let db = RivaDB::new();
    let user = RivaUser::new();

    let rivabot = RivaBot::new(settings, db, user).await;

    if let Err(run) = rivabot.run().await {
        eprintln!("Error running rivabot: {run}");
        exit(1);
    }

    Ok(())
}
