//! # Multi
//! A read-only module to read data from a specific multi.
pub mod response;
extern crate serde_json;

use crate::models::multi::response::*;

use crate::client::Client;
use crate::util::defaults::default_client;
use crate::util::RouxError;

/// Access multis API.
pub struct Multis {
    client: Client,
    url: String,
}

impl Multis {
    /// Create a new `Multis` instance.
    pub fn new() -> Multis {
        Multis {
            client: default_client(),
            url: "https://www.reddit.com/api/multi".to_owned(),
        }
    }

    /// Create a new authenticated `Multis` instance using an oauth client
    /// from the `Reddit` module.
    pub fn new_oauth(client: &Client) -> Multis {
        Multis {
            client: client.to_owned(),
            url: "https://oauth.reddit.com/api/multi".to_owned(),
        }
    }
        

    /// List multis belonging to the current user (requires authentication).
    #[maybe_async::maybe_async]
    pub async fn mine(&self) -> Result<MultisResponse, RouxError> {
        Ok(self
            .client
            .get(&format!("{}/mine", self.url))
            .send()
            .await?
            .json::<MultisResponse>()
            .await?)
    }

    /// List multis belonging to a specific user.
    #[maybe_async::maybe_async]
    pub async fn user(&self, username: &str) -> Result<MultisResponse, RouxError> {
        Ok(self
            .client
            .get(&format!("{}/user/{}", self.url, username))
            .send()
            .await?
            .json::<MultisResponse>()
            .await?)
    }
}

/// Access multi API.
pub struct Multi {
    /// Multireddit URL path.
    pub multipath: String,
    client: Client,
    url: String,
}

impl Multi {
    /// Create a new `Multi` instance.
    pub fn new(multipath: &str) -> Multi {
        Multi {
            multipath: multipath.to_owned(),
            client: default_client(),
            url: "https://www.reddit.com/api/multi".to_owned(),
        }
    }

    /// Create a new authenticated `Multi` instance using an oauth client
    /// from the `Reddit` module.
    pub fn new_oauth(multipath: &str, client: &Client) -> Multi {
        Multi {
            multipath: multipath.to_owned(),
            client: client.to_owned(),
            url: "https://oauth.reddit.com/api/multi".to_owned(),
        }
    }

    /// Fetch a multi's data and subreddit list by name.
    #[maybe_async::maybe_async]
    pub async fn about(&self) -> Result<MultiResponse, RouxError> {
        Ok(self
            .client
            .get(&format!("{}/{}", self.url, self.multipath))
            .send()
            .await?
            .json::<MultiResponse>()
            .await?)
    }

    /// Get a multi's description.
    #[maybe_async::maybe_async]
    pub async fn description(&self) -> Result<MultiDescriptionResponse, RouxError> {
        Ok(self
            .client
            .get(&format!("{}/{}/description", self.url, self.multipath))
            .send()
            .await?
            .json::<MultiDescriptionResponse>()
            .await?)
    }

    /// Get data about a subreddit in a multi.
    #[maybe_async::maybe_async]
    pub async fn subreddit(&self, srname: &str) -> Result<MultiSubredditData, RouxError> {
        Ok(self
            .client
            .get(&format!("{}/{}/r/{}", self.url, self.multipath, srname))
            .send()
            .await?
            .json::<MultiSubredditData>()
            .await?)
    }
}
