use hyper::{
    net::HttpsConnector,
    Client as HyperClient,
};
use hyper_native_tls::NativeTlsClient;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub enum UrbanError {
    Network,
    JsonError,
}

pub type UrbanResult<T> = Result<T, UrbanError>;

#[derive(Default)]
pub struct Client {
    handle: HyperClient,
}

impl Client {
    pub fn new() -> Self {
        let ssl = NativeTlsClient::new().unwrap(); // TODO: Return result
        let connector = HttpsConnector::new(ssl);
        let handle = HyperClient::with_connector(connector);
        Client { handle }
    }

    pub fn lookup(&self, word: &str) -> UrbanResult<DefintionList> {
        let res = self
            .handle
            .get(&format!(
                "https://api.urbandictionary.com/v0/define?term={}",
                word
            ))
            .send()
            .map_err(|_| UrbanError::Network)?;

        serde_json::from_reader(res).map_err(|_| UrbanError::JsonError)
    }
}

#[derive(Debug, Deserialize)]
pub struct DefintionList {
    list: Vec<Definition>,
    #[serde(flatten)]
    unknown: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Definition {
    author: String,
    current_vote: String,
    defid: u64,
    definition: String,
    example: String,
    permalink: String,
    // sound_urls:
    thumbs_down: u64,
    thumbs_up: u64,
    word: String,
    written_on: String,
    #[serde(flatten)]
    unknown: HashMap<String, Value>,
}
