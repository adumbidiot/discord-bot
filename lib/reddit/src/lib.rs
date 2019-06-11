extern crate hyper;
extern crate hyper_native_tls;
#[macro_use]
extern crate serde_derive;

use hyper::{
    client::RedirectPolicy,
    header::Location,
    net::HttpsConnector,
    status::StatusCode,
    Client as HyperClient,
};
use hyper_native_tls::NativeTlsClient;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub enum RedditError {
    Network,
    InvalidStatusCode(u16),
    NotFound,
    JsonError,
}

pub type RedditResult<T> = Result<T, RedditError>;

pub struct Client {
    handle: HyperClient,
}

impl Client {
    pub fn new() -> Self {
        let ssl = NativeTlsClient::new().unwrap(); // TODO: Return result
        let connector = HttpsConnector::new(ssl);
        let mut handle = HyperClient::with_connector(connector);
        handle.set_redirect_policy(RedirectPolicy::FollowNone);
        Client { handle }
    }

    pub fn get_subreddit(&self, subreddit: &str) -> RedditResult<SubRedditListing> {
        let res = self
            .handle
            .get(&format!("https://www.reddit.com/r/{}.json", subreddit))
            .send()
            .map_err(|_| RedditError::Network)?;

        if !res.status.is_success() {
            return match res.status {
                StatusCode::Found => match res.headers.get::<Location>() {
                    Some(link) => {
                        if link.starts_with("https://www.reddit.com/subreddits/search.json?") {
                            Err(RedditError::NotFound)
                        } else {
                            Err(RedditError::InvalidStatusCode(res.status.to_u16()))
                        }
                    }
                    None => Err(RedditError::InvalidStatusCode(res.status.to_u16())),
                },
                _ => Err(RedditError::InvalidStatusCode(res.status.to_u16())),
            };
        }

        serde_json::from_reader(res).map_err(|_| RedditError::JsonError)
    }
}

#[derive(Debug, Deserialize)]
pub struct SubRedditListing {
    pub kind: String,
    pub data: SubRedditListingData,

    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct SubRedditListingData {
    pub after: String,
    pub before: Option<String>,
    pub children: Vec<SubRedditEntry>,
    pub dist: u32,

    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct SubRedditEntry {
    pub kind: String,
    pub data: SubRedditEntryData,

    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct SubRedditEntryData {
    pub archived: bool,
    pub author: String,
    pub author_flair_css_class: Option<String>,
    pub author_flair_template_id: Option<String>,
    pub author_flair_text: Option<String>,
    pub author_flair_text_color: Option<String>,
    pub author_flair_type: String,
    pub author_fullname: String,
    pub author_patreon_flair: bool,
    pub can_gild: bool,
    pub can_mod_post: bool,
    pub clicked: bool,
    pub contest_mode: bool,
    pub created: f64,
    pub created_utc: f64,
    pub domain: String,
    pub downs: u64,
    pub edited: bool,
    pub gilded: u32,
    pub hidden: bool,
    pub hide_score: bool,
    pub id: String,
    pub is_crosspostable: bool,
    pub is_meta: bool,
    pub is_original_content: bool,
    pub is_reddit_media_domain: bool,
    pub is_robot_indexable: bool,
    pub is_self: bool,
    pub is_video: bool,
    pub link_flair_text_color: String,
    pub link_flair_type: String,
    pub locked: bool,
    pub media_only: bool,
    pub name: String,
    pub no_follow: bool,
    pub num_comments: u32,
    pub num_crossposts: u32,
    pub over_18: bool,
    pub parent_whitelist_status: String,
    pub permalink: String,
    pub pinned: bool,
    pub post_hint: String,
    pub pwls: u32,
    pub quarantine: bool,
    pub saved: bool,
    pub score: u32,
    pub send_replies: bool,
    pub spoiler: bool,
    pub stickied: bool,
    pub subreddit: String,
    pub subreddit_id: String,
    pub subreddit_name_prefixed: String,
    pub subreddit_subscribers: u64,
    pub subreddit_type: String,
    pub suggested_sort: String,
    pub thumbnail: String,
    pub thumbnail_height: u32,
    pub thumbnail_width: u32,
    pub title: String,
    pub ups: u32,
    pub url: String,
    pub visited: bool,
    pub whitelist_status: String,
    pub wls: u32,

    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}
