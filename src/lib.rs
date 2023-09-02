#[cfg(feature = "undocumented")]
use crate::search::SearchOptions;
use const_format::formatcp;
use reqwest::{Client, Method, RequestBuilder, Url};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[cfg(feature = "undocumented")]
mod bitflags;
#[cfg(feature = "undocumented")]
pub mod search;

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

    /// Set the guild count displayed on your bot's listing page.
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

    /// Add a command to your bot listing page.
    #[cfg(feature = "undocumented")]
    pub async fn add_bot_command(&self, command: Command) -> reqwest::Result<()> {
        self.build_request(
            Method::POST,
            Self::endpoint(&format!("/bots/{}/commands/{}", self.bot_id, command.id)),
        )
        .json(&command)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
    }

    /// Fetch a bot listing page
    #[cfg(feature = "undocumented")]
    pub async fn get_bot(&self, id: u64) -> reqwest::Result<Bot> {
        self.build_request(Method::POST, Self::endpoint(&format!("/bots/{id}")))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    /// Fetch a user on discordlist.gg
    #[cfg(feature = "undocumented")]
    pub async fn get_user(&self, id: u64) -> reqwest::Result<User> {
        self.build_request(Method::POST, Self::endpoint(&format!("/users/{id}")))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    /// Search for Discord bots listed on discordlist.gg
    #[cfg(feature = "undocumented")]
    pub async fn search(&self, mut options: SearchOptions) -> reqwest::Result<SearchResults> {
        const SEARCH_HOST: &str = "https://search.discordlist.gg";
        let mut endpoint = Self::endpoint("/bots/search");
        endpoint.set_host(Some(SEARCH_HOST)).unwrap();

        if options.query.is_none() {
            options.query = Some("*".to_string());
        }

        self.build_request(Method::POST, endpoint)
            .json(&options)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
/// A bot's listing page on discordlist.gg.
pub struct Bot {
    flags: u64,
    bot_id: Option<String>,
    features: u64,
    id: String,
    username: String,
    avatar: String,
    discriminator: u64,
    prefix: String,
    is_packable: bool,
    is_hidden: bool,
    is_forced_into_hiding: bool,
    invite_url: String,
    webhook_url: Option<String>,
    webhook_auth: Option<String>,
    website_url: String,
    repo_url: String,
    twitter_url: String,
    instagram_url: String,
    support_server_url: String,
    slug: String,
    tags: Vec<String>,
    created_on: String,
    owner_id: String,
    co_owner_ids: Vec<String>,
    brief_description: String,
    long_description: String,
    guild_count: u64,
    votes: u64,
    all_time_votes: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
/// A user on discordlist.gg.
pub struct User {
    avatar: Option<String>,
    banner: Option<String>,
    bio: Option<String>,
    bots: Vec<String>,
    claps: u64,
    co_owned_bots: Vec<String>,
    co_owned_guilds: Vec<String>,
    created_on: String,
    #[deprecated = "user discriminators are being phased out from Discord"]
    discriminator: u64,
    display_name: Option<String>,
    flags: u64,
    guilds: Vec<String>,
    id: String,
    packs: Vec<String>,
    slug: Option<String>,
    username: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// A search on discordlist.gg.
pub struct SearchResults {
    hits: Vec<SearchHit>,
    limit: u64,
    nb_hits: u64,
    offset: u64,
    query: String,
    tag_distribution: HashMap<String, u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
/// A search hit on discordlist.gg.
pub struct SearchHit {
    avatar: String,
    brief_description: String,
    co_owner_ids: Vec<String>,
    created_on: String,
    discriminator: u64,
    features: String,
    flags: String,
    guild_count: u64,
    id: String,
    invite_url: String,
    owner_id: String,
    prefix: String,
    tags: String,
    username: String,
    votes: String,
}
