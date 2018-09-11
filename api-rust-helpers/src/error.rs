use http::uri::InvalidUri;
use hyper;
use serde_json;
use std::result;

pub enum Error {
    Http(hyper::Error),
    Json(serde_json::Error),
    Format(String),
    Client(u16, String),
    InvalidUrl,
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Http(err)
    }
}

impl From<InvalidUri> for Error {
    fn from(err: InvalidUri) -> Error {
        Error::InvalidUrl
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

pub type Result<T> = result::Result<T, Error>;
