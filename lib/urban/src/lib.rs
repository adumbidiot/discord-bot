use reqwest::Client as ReqwestClient;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub enum UrbanError {
    Network,
    JsonError,
}

pub type UrbanResult<T> = Result<T, UrbanError>;

pub struct Client {
    handle: ReqwestClient,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        Client {
            handle: ReqwestClient::new(),
        }
    }

    pub fn lookup(&self, word: &str) -> UrbanResult<DefintionList> {
        self.handle
            .get(&format!(
                "https://api.urbandictionary.com/v0/define?term={}",
                word
            ))
            .send()
            .map_err(|_| UrbanError::Network)?
            .json()
            .map_err(|_| UrbanError::JsonError)
    }
}

#[derive(Debug, Deserialize)]
pub struct DefintionList {
    pub list: Vec<Definition>,
    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Definition {
    pub author: String,
    pub current_vote: String,
    pub defid: u64,
    pub definition: String,
    pub example: String,
    pub permalink: String,
    // sound_urls:
    pub thumbs_down: u64,
    pub thumbs_up: u64,
    pub word: String,
    pub written_on: String,
    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}
