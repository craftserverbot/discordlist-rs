use std::{env, time::Duration};

use discordlist::DiscordlistClient;

const USAGE: &str = "Usage: update_guilds <guild_count>";

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    _ = dotenvy::dotenv();

    let mut args = env::args();
    _ = args.next();
    let guild_count = args
        .next()
        .expect(USAGE)
        .parse::<u64>()
        .expect("Guild count should be a valid u64");

    let token = env::var("DLIST_TOKEN").expect("DLIST_TOKEN should be set");
    let bot_id = env::var("BOT_ID")
        .expect("BOT_ID should be set")
        .parse()
        .expect("BOT_ID should be a valid u64");

    let client = DiscordlistClient::new(token, bot_id, Duration::from_secs(5)).unwrap();
    client.set_guild_count(guild_count).await?;

    Ok(())
}
