use serde::{
    de::DeserializeOwned,
    Deserialize,
    Serialize,
};
use std::collections::HashMap;

pub type NekosResult<T> = Result<T, NekosError>;

#[derive(Debug)]
pub enum NekosError {
    Network,
    Json,
}

#[derive(Debug)]
pub enum ApiRequest {
    GetRandomImages { nsfw: bool, count: u8 },
}

impl ApiRequest {
    pub fn get_random_image(nsfw: bool, count: u8) -> ApiRequest {
        ApiRequest::GetRandomImages {
            nsfw,
            count: count.min(100),
        }
    }
}

#[derive(Serialize)]
struct GetRandomImageQuery {
    nsfw: bool,
    count: u8,
}

pub struct Client {
    handle: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Client {
            handle: reqwest::Client::new(),
        }
    }

    pub fn send_req<T: DeserializeOwned>(&self, req: &ApiRequest) -> NekosResult<T> {
        match req {
            ApiRequest::GetRandomImages { nsfw, count } => {
                let query = GetRandomImageQuery {
                    nsfw: *nsfw,
                    count: *count,
                };

                self.handle
                    .get("https://nekos.moe/api/v1/random/image")
                    .query(&query)
                    .send()
                    .map_err(|_| NekosError::Network)?
                    .json()
                    .map_err(|_| NekosError::Json)
            }
        }
    }

    pub fn get_random_images(&self, nsfw: bool, count: u8) -> NekosResult<ImageArray> {
        self.send_req(&ApiRequest::get_random_image(nsfw, count))
    }
}

#[derive(Debug, Deserialize)]
pub struct ImageArray {
    pub images: Vec<Image>,

    #[serde(flatten)]
    pub unknown: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    pub id: String,
    pub artist: Option<String>,
    pub nsfw: bool,
    pub tags: Vec<String>,
    pub likes: u32,
    pub favorites: u32,
    pub uploader: ShortUser,
    pub approver: Option<ShortUser>,
    pub comments: Vec<serde_json::Value>,
    #[serde(rename = "originalHash")]
    pub original_hash: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(flatten)]
    pub unknown: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ShortUser {
    pub id: String,
    pub username: String,

    #[serde(flatten)]
    pub unknown: HashMap<String, serde_json::Value>,
}
