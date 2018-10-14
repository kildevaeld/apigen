use error;
use serde;
use serde_json;
use std::fmt;
use url::Url;
pub fn decode<'a, T>(mime: &str, value: &'a [u8]) -> error::Result<T>
where
    T: serde::Deserialize<'a>,
{
    match mime {
        "application/json" => Ok(serde_json::from_slice(value)?),
        _ => Err(error::ErrorKind::Mime(mime.to_string()).into()),
    }
}

pub fn encode<T>(mime: &str, value: &T) -> error::Result<String>
where
    T: serde::Serialize,
{
    match mime {
        "application/json" => Ok(serde_json::to_string(value)?),
        _ => Err(error::ErrorKind::Mime(mime.to_string()).into()),
    }
}

pub fn join(endpoint: &str, path: &[&str]) -> error::Result<Url> {
    let mut url: Url = endpoint.parse()?;
    let mut full_path = path.join("/");
    //url.set_path(&mut full_path);

    Ok(url.join(&full_path)?)
    //Ok(url)
}

// pub fn join_query(endpoint: &str, path: &[&str]) -> error::Result<Url> {
//     let mut url: Url = endpoint.parse()?;
//     let mut full_path = path.join("/");
//     url.set_path(&mut full_path);

//     Ok(url)
// }

pub fn set_header<T: fmt::Display>(
    request: reqwest::async::RequestBuilder,
    name: &str,
    value: T,
) -> reqwest::async::RequestBuilder {
    request.header(name, format!("{}", value))
}
