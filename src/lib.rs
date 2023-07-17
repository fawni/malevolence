use reqwest::IntoUrl;
use serde::de::DeserializeOwned;

pub mod matches;

pub type Result<T> = std::result::Result<T, self::Error>;

/// The client used to make requests to the `OpenDota` API
#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    pub(crate) key: Option<String>,
}

impl Client {
    /// Create a new malevolence client
    #[must_use]
    pub fn new(key: Option<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            key,
        }
    }

    /// Create a new malevolence client with the supplied reqwest client
    #[must_use]
    pub const fn with(client: reqwest::Client, key: Option<String>) -> Self {
        Self { client, key }
    }

    pub(crate) async fn json<T, U: IntoUrl + Send>(&self, url: U) -> Result<T>
    where
        T: DeserializeOwned,
    {
        Ok(self.client.get(url).send().await?.json::<T>().await?)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new(None)
    }
}

/// Represents the two teams in a Dota 2 match
#[derive(Debug)]
pub enum Team {
    Radiant,
    Dire,
}

#[derive(Debug)]
pub enum Error {
    Web(reqwest::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Web(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Web(e)
    }
}
