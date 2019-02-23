extern crate hyper;
extern crate hyper_native_tls;

use hyper::{
    client::RedirectPolicy,
    header::Location,
    net::HttpsConnector,
    status::StatusCode,
    Client as HyperClient,
};

use hyper_native_tls::NativeTlsClient;

#[derive(Debug)]
pub enum XkcdError {
    Network,
    InvalidStatusCode(u16),
    MissingLocationHeader,
}

pub type XkcdResult<T> = Result<T, XkcdError>;

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

    pub fn get_random(&self) -> XkcdResult<String> {
        let res = self
            .handle
            .get("https://c.xkcd.com/random/comic/")
            .send()
            .map_err(|_| XkcdError::Network)?;

        if res.status != StatusCode::Found {
            return Err(XkcdError::InvalidStatusCode(res.status.to_u16()));
        }

        res.headers
            .get::<Location>()
            .map(|h| h.0.clone())
            .ok_or(XkcdError::MissingLocationHeader)
    }
}
