//! Multi responses.
use serde::Deserialize;

use crate::subreddit::response::SubredditData;

/// MultisResponse.
pub type MultisResponse = Vec<MultiResponse>;

/// MultiResponse.
#[derive(Deserialize, Debug)]
pub struct MultiResponse {
    /// Should be `LabeledMulti`.
    pub kind: String,
    /// Data about multi.
    pub data: MultiData,
}

/// MultiDescriptionResponse.
#[derive(Deserialize, Debug)]
pub struct MultiDescriptionResponse {
    /// Should be `LabeledMultiDescription`.
    pub kind: String,
    /// Data about multi.
    pub data: MultiDescriptionData,
}

/// MultiData.
#[derive(Deserialize, Debug)]
pub struct MultiData {
    /// Whether or not this multi can be edited by the current user.
    pub can_edit: Option<bool>,
    /// The display name of the multi. This is the name that is shown on the website.
    pub display_name: Option<String>,
    /// Name of the multi. This is the name that is used in the URL.
    pub name: Option<String>,
    /// Description of the multi.
    pub description_html: Option<String>,
    /// Number of subscribers to the multi.
    pub num_subscribers: Option<u64>,
    /// The URL of the icon of the multi.
    pub icon_url: Option<String>,
    /// The list of subreddits in the multi.
    pub subreddits: Option<Vec<MultiSubredditData>>,
}

/// MultiDescriptionData.
#[derive(Deserialize, Debug)]
pub struct MultiDescriptionData {
    /// HTML description of the multi.
    pub body_html: Option<String>,
    /// Markdown description of the multi.
    pub body_md: Option<String>,
}

/// MultiSubredditData.
#[derive(Deserialize, Debug)]
pub struct MultiSubredditData {
    /// The name of the subreddit.
    pub name: Option<String>,
    /// The data about the subreddit.
    pub data: Option<SubredditData>,
}
