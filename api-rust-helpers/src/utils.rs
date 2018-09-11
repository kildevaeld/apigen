use error;
use hyper::rt::{Future, Stream};
use hyper::{Body, Uri};
use serde;
use serde_json;

// pub fn decode<'a, T>(mime: &str, body: Body) -> impl Future<Item = T, Error = error::Error>
// where
//     T: serde::Deserialize<'a>,
// {
//     let mime = mime.to_string();

//     body.concat2()
//         .from_err::<error::Error>()
//         .and_then(|b| {
//             // let data = match mime.as_str() {
//             //     "application/json" => serde_json::from_slice::<T>(&b)?,
//             //     _ => return Err(error::Error::Format(format!("invalid format: '{}'", mime))),
//             // };

//             let data = serde_json::from_slice(&b)?;

//             Ok(data)
//         })
//         .from_err()
// }

pub fn decode<'a, T>(mime: &str, value: &'a [u8]) -> error::Result<T>
where
    T: serde::Deserialize<'a>,
{
    match mime {
        "application/json" => Ok(serde_json::from_slice(value)?),
        _ => Err(error::Error::Format(format!("invalid format: '{}'", mime))),
    }
}

pub fn encode<T>(mime: &str, value: &T) -> error::Result<String>
where
    T: serde::Serialize,
{
    match mime {
        "application/json" => Ok(serde_json::to_string(value)?),
        _ => Err(error::Error::Format(format!("invalid format: '{}'", mime))),
    }
}

pub fn join(endpoint: &str, path: &[&str]) -> error::Result<Uri> {}
