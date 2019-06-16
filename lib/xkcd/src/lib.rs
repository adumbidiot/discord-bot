use reqwest::Client as ReqwestClient;

#[derive(Debug)]
pub enum XkcdError {
    Network,
    InvalidStatusCode(u16),
    MissingLocationHeader,
}

pub type XkcdResult<T> = Result<T, XkcdError>;

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
        let handle = reqwest::Client::builder()
            .redirect(reqwest::RedirectPolicy::none())
            .build()
            .unwrap();
        Client { handle }
    }

    pub fn get_random(&self) -> XkcdResult<String> {
        let res = self
            .handle
            .get("https://c.xkcd.com/random/comic/")
            .send()
            .map_err(|_| XkcdError::Network)?;

        let status = res.status();
        if status != reqwest::StatusCode::FOUND {
            return Err(XkcdError::InvalidStatusCode(status.as_u16()));
        }

        res.headers()
            .get(reqwest::header::LOCATION)
            .map(|h| h.to_str().unwrap().to_string())
            .ok_or(XkcdError::MissingLocationHeader)
    }
}
