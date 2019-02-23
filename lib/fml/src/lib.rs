extern crate hyper;
extern crate hyper_native_tls;
extern crate select;

use hyper::{
    net::HttpsConnector,
    Client as HyperClient,
};

use hyper_native_tls::NativeTlsClient;
use select::{
    document::Document,
    predicate::{
        And,
        Class,
        Name,
        Text,
    },
};

#[derive(Debug)]
pub enum FmlError {
    Network,
    InvalidStatusCode(u16),
    InvalidBody,
    MissingField(&'static str),
}

pub type FmlResult<T> = Result<T, FmlError>;

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

    pub fn get_random(&self) -> FmlResult<Vec<FmlEntry>> {
        let res = self
            .handle
            .get("https://www.fmylife.com/random")
            .send()
            .map_err(|_| FmlError::Network)?;

        if !res.status.is_success() {
            return Err(FmlError::InvalidStatusCode(res.status.to_u16()));
        }

        Document::from_read(res)
            .map_err(|_| FmlError::InvalidBody)?
            .find(And(Name("div"), Class("col-sm-8")))
            .last()
            .ok_or(FmlError::MissingField("main body"))?
            .find(And(Name("article"), Class("art-panel")))
            .filter(|el| {
                el.find(And(Name("figure"), Class("text-center")))
                    .last()
                    .is_some()
            })
            .map(|el| {
                let text = el
                    .find(And(Name("p"), Class("block")))
                    .last()
                    .ok_or(FmlError::MissingField("text p"))?
                    .find(Name("a"))
                    .last()
                    .ok_or(FmlError::MissingField("text link"))?
                    .find(Text)
                    .last()
                    .ok_or(FmlError::MissingField("text element"))?
                    .as_text()
                    .ok_or(FmlError::MissingField("text"))?
                    .trim()
                    .to_string();

                let mut base = el
                    .find(And(Name("div"), Class("text-left")))
                    .last()
                    .ok_or(FmlError::MissingField("sucks/deeserved div"))?
                    .find(Name("span"));

                let sucks = base
                    .next()
                    .ok_or(FmlError::MissingField("sucks span"))?
                    .find(Name("button"))
                    .last()
                    .ok_or(FmlError::MissingField("sucks button"))?
                    .find(Text)
                    .last()
                    .ok_or(FmlError::MissingField("sucks text element"))?
                    .as_text()
                    .ok_or(FmlError::MissingField("sucks text"))?
                    .parse()
                    .map_err(|_| FmlError::MissingField("sucks num"))?;

                let deserved = base
                    .next()
                    .ok_or(FmlError::MissingField("deserved span"))?
                    .find(Name("button"))
                    .last()
                    .ok_or(FmlError::MissingField("deserved button"))?
                    .find(Text)
                    .last()
                    .ok_or(FmlError::MissingField("deserved text element"))?
                    .as_text()
                    .ok_or(FmlError::MissingField("deserved text element"))?
                    .parse()
                    .map_err(|_| FmlError::MissingField("deserved num"))?;

                let mut reaction_base = el
                    .find(And(Name("div"), Class("clearfix")))
                    .last()
                    .ok_or(FmlError::MissingField("reaction main div"))?
                    .find(And(Name("div"), Class("smileys")))
                    .last()
                    .ok_or(FmlError::MissingField("reaction inner div"))?
                    .find(Name("button"));

                let amusing = reaction_base
                    .next()
                    .ok_or(FmlError::MissingField("amusing button"))?
                    .find(And(Name("div"), Class("count")))
                    .last()
                    .ok_or(FmlError::MissingField("amusing div"))?
                    .find(Text)
                    .last()
                    .ok_or(FmlError::MissingField("amusing text element"))?
                    .as_text()
                    .ok_or(FmlError::MissingField("amusing text"))?
                    .trim()
                    .parse()
                    .map_err(|_| FmlError::MissingField("amusing num"))?;

                let funny = reaction_base
                    .next()
                    .ok_or(FmlError::MissingField("funny button"))?
                    .find(And(Name("div"), Class("count")))
                    .last()
                    .ok_or(FmlError::MissingField("funny div"))?
                    .find(Text)
                    .last()
                    .ok_or(FmlError::MissingField("funnt text element"))?
                    .as_text()
                    .ok_or(FmlError::MissingField("funny text"))?
                    .trim()
                    .parse()
                    .map_err(|_| FmlError::MissingField("funny num"))?;

                let weird = reaction_base
                    .next()
                    .ok_or(FmlError::MissingField("weird button"))?
                    .find(And(Name("div"), Class("count")))
                    .last()
                    .ok_or(FmlError::MissingField("weird div"))?
                    .find(Text)
                    .last()
                    .ok_or(FmlError::MissingField("weird text element"))?
                    .as_text()
                    .ok_or(FmlError::MissingField("weird text"))?
                    .trim()
                    .parse()
                    .map_err(|_| FmlError::MissingField("weird num"))?;

                let hilarious = reaction_base
                    .next()
                    .ok_or(FmlError::MissingField("hilarious button"))?
                    .find(And(Name("div"), Class("count")))
                    .last()
                    .ok_or(FmlError::MissingField("hilarious div"))?
                    .find(Text)
                    .last()
                    .ok_or(FmlError::MissingField("hilarious text element"))?
                    .as_text()
                    .ok_or(FmlError::MissingField("hilarious text"))?
                    .trim()
                    .parse()
                    .map_err(|_| FmlError::MissingField("hilarious num"))?;

                Ok(FmlEntry {
                    text,
                    sucks,
                    deserved,
                    amusing,
                    funny,
                    weird,
                    hilarious,
                })
            })
            .collect::<FmlResult<_>>()
    }
}

#[derive(Debug)]
pub struct FmlEntry {
    pub text: String,
    pub sucks: u32,    // I agree, your life sucks
    pub deserved: u32, // You deserved it
    pub amusing: u32, //
    pub funny: u32,
    pub weird: u32,
    pub hilarious: u32,
}
