use reqwest::Client as ReqwestClient;
use serde::{
    de::DeserializeOwned,
    Deserialize,
};
use std::{
    borrow::Cow,
    collections::HashMap,
};

#[derive(Debug)]
pub enum FmlError {
    Network,
    InvalidStatusCode(u16),
    Api(String),
    Json(String),
}

pub type FmlResult<T> = Result<T, FmlError>;

pub enum Method {
    Get,
}

pub struct Request<'a> {
    pub url: Cow<'a, str>,
    pub method: Method,
}

impl Into<http::method::Method> for Method {
    fn into(self) -> http::method::Method {
        match self {
            Method::Get => http::method::Method::GET,
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Err {
        error: String,
    },
    Ok {
        data: T,
        #[serde(flatten)]
        unknown: HashMap<String, serde_json::Value>,
    },
}

impl<T> Into<FmlResult<((), T, ())>> for ApiResponse<T> {
    fn into(self) -> FmlResult<((), T, ())> {
        let res: Result<_, _> = self.into();
        res.map_err(|e| FmlError::Api(e))
    }
}

impl<T> Into<Result<((), T, ()), String>> for ApiResponse<T> {
    fn into(self) -> Result<((), T, ()), String> {
        match self {
            ApiResponse::Ok { data, .. } => Ok(((), data, ())),
            ApiResponse::Err { error } => Err(error),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Article {
    pub apikey: Option<String>,
    pub area: Option<String>,
    pub author: String,
    pub bitly: Option<String>,
    pub city: Option<String>,
    pub content: String,
    pub content_hidden: String,
    pub country: Option<String>,
    pub countrycode: Option<String>,
    pub created: String,
    pub flag: u32,
    pub gender: Option<u8>,
    pub id: u64,
    pub images: Vec<ArticleImage>,
    pub ip: Option<String>,
    pub keywords: Vec<ArticleKeyword>,
    pub layout: u32,
    pub metrics: ArticleMetrics,
    pub openview: u32,
    pub origin: Option<String>,
    pub paragraphs: Vec<serde_json::Value>,
    pub published: String,
    pub site: u32,
    pub siteorig: Option<serde_json::Value>,
    pub slug: String,
    #[serde(rename = "socialTruncate")]
    pub social_truncate: bool,
    pub spicy: bool,
    pub status: u32,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub article_type: u32,
    pub updated: String,
    pub url: String,
    pub user: u64,
    pub usermetrics: ArticleUsermetrics,
    pub videos: Vec<serde_json::Value>,
    pub vote: u32,

    #[serde(flatten)]
    pub unknown: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ArticleImage {
    pub copyright: Option<String>,
    pub height: u32,
    pub legend: Option<serde_json::Value>,
    pub name: String,
    pub url: String,
    pub usage: u32,
    pub width: u32,

    #[serde(flatten)]
    pub unknown: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ArticleKeyword {
    pub label: String,
    pub rub: bool,
    pub uid: u32,

    #[serde(flatten)]
    pub unknown: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ArticleMetrics {
    pub article: u64,
    pub comment: u32,
    pub favorite: u32,
    pub mod_negative: u32,
    pub mod_positive: u32,
    pub reports: u32,
    pub smiley_amusing: u32,
    pub smiley_funny: u32,
    pub smiley_hilarious: u32,
    pub smiley_weird: u32,
    pub votes_down: u32,
    pub votes_up: u32,

    #[serde(flatten)]
    pub unknown: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ArticleUsermetrics {
    pub favorite: bool,
    pub smiley: Option<serde_json::Value>,
    pub votes: Option<serde_json::Value>,
    #[serde(flatten)]
    pub unknown: HashMap<String, serde_json::Value>,
}

pub struct Client {
    handle: ReqwestClient,
}

impl Client {
    pub fn new(key: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "X-VDM-Api-Key",
            http::header::HeaderValue::from_str(key).unwrap(),
        );
        let handle = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Client { handle }
    }

    pub fn send_req<T: DeserializeOwned>(&self, req: Request) -> FmlResult<T> {
        let res: Result<((), T, ()), _> = self
            .handle
            .request(req.method.into(), &*req.url)
            .send()
            .map_err(|_| FmlError::Network)?
            .json::<ApiResponse<T>>()
            .map_err(|e| FmlError::Json(e.to_string()))?
            .into();
        res.map(|data| data.1)
    }

    pub fn list_random(&self, n: usize) -> FmlResult<Vec<Article>> {
        let req = Request {
			method: Method::Get,
			url: format!("https://www.fmylife.com/api/v2/article/list?page[number]=1&page[bypage]={}&orderby[RAND()]=ASC", n).into(),
		};

        self.send_req(req)
    }
}
