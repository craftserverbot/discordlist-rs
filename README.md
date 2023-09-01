# discordlist-rs

> Integrate your Discord bot with the [discordlist.gg](1) API.

## Installation

Install the crate using `cargo add`:

```sh
cargo add --git https://github.com/craftserverbot/discordlist-rs
```

Update the crate using `cargo update`:

```sh
cargo update -p discordlist-rs
```

## Overview

This Rust library provides a REST client for the Discordlist API. The primary use of this API is to show a real-time server count on your bot's listing page.

Discordlist-rs uses `reqwest` to make HTTPS requests, which has two options for TLS implementations. These are exposed as features:

- `rustls-tls` (default): Use the `rustls` crate to make HTTPS requests.
- `native-tls`: Use the platform-specific TLS implementation to make HTTPS requests.

Discordlist also has some undocumented API endpoints which can be enabled with a feature:

- `undocumented`: Enable making API calls that are not documented by Discordlist. Considered less stable.

## Usage

To use the client included with this crate, you will need to know your Discord bot's ID and its Discordlist token. You can retrieve the latter from the Discordlist "Manage" > "Webhooks" screen on your bot's listing.

You can update your bot's guild count using the `DiscordlistClient::set_guild_count` method.

```rs
use std::{env, time::Duration};
use discordlist::DiscordlistClient;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let guild_count = 400;
    let token = "...".to_string();
    let bot_id = 1044791133416464384;

    let client = DiscordlistClient::new(token, bot_id, Duration::from_secs(5)).unwrap();
    client.set_guild_count(guild_count).await?;

    Ok(())
}
```

### Timeouts

You can adjust the time it takes before requests are aborted by changing the `Duration` passed to the client. Setting the timeout to a very low value could cause the client's methods to fail.

## Support

Contact `@doinkythederp` on Discord if you have questions or concerns about this library. You can find me on the [Discordlist server](https://discord.com/invite/XbuJ6VH) or the [CraftServer support server](https://craftserver.net).
