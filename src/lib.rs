use std::time::Duration;

use const_format::formatcp;
use reqwest::{Client, Method, RequestBuilder, Url};
use serde::{Deserialize, Serialize};

const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
const CRATE_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const USER_AGENT: &str = formatcp!("discordlist-rs/{CRATE_VERSION} ({CRATE_REPOSITORY})");

pub struct DiscordlistClient {
    token: String,
    client: Client,
    bot_id: u64,
}

impl DiscordlistClient {
    /// Create a new client for the discordlist.gg API.
    ///
    /// # Arguments
    ///
    /// - `token`: The token provided to you by discordlist.gg on the Manage > Webhooks page.
    ///   Do not prepend "Bearer" or "Bot" to the token.
    /// - `bot_id`: The numeric ID of your bot, visible in its discordlist.gg URLs.
    /// - `timeout`: The time it takes before the client will give up on a request
    ///
    /// # Errors
    ///
    /// This method fails if a TLS backend cannot be initialized,
    /// or the Reqwest resolver cannot load the system configuration.
    pub fn new(token: String, bot_id: u64, timeout: Duration) -> reqwest::Result<Self> {
        Ok(Self {
            client: Client::builder()
                .user_agent(USER_AGENT)
                .timeout(timeout)
                .build()?,
            token,
            bot_id,
        })
    }

    /// Get the token used to make API calls
    pub fn get_token(&self) -> &str {
        &self.token
    }

    /// Get the bot id of this client
    pub fn get_bot_id(&self) -> u64 {
        self.bot_id
    }

    fn endpoint(path: &str) -> Url {
        Url::parse(&format!("https://api.discordlist.gg/v0{path}")).unwrap()
    }

    fn build_request(&self, method: Method, endpoint: Url) -> RequestBuilder {
        self.client
            .request(method, endpoint)
            .bearer_auth(&self.token)
    }

    /// Set the guild count displayed on your bot's discordlist.gg listing.
    pub async fn set_guild_count(&self, guild_count: u64) -> reqwest::Result<()> {
        let mut endpoint = Self::endpoint(&format!("/bots/{}/guilds", self.bot_id));
        endpoint
            .query_pairs_mut()
            .append_pair("count", &guild_count.to_string());

        self.build_request(Method::PUT, endpoint)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Add a command to your discordlist.gg bot listing page.x
    #[cfg(feature = "undocumented")]
    pub async fn add_bot_command(&self, command: Command) -> reqwest::Result<()> {
        self.build_request(
            Method::POST,
            Self::endpoint(&format!("/bots/{}/commands/{}", self.bot_id, command.id)),
        )
        .json(&command)
        .send()
        .await?
        .error_for_status()?;

        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
/// A command shown on a discordlist.gg bot listing page.
pub struct Command {
    #[serde(skip)]
    id: u64,
    command_name: String,
    description: String,
    syntax: String,
    categories: Vec<String>,
}
