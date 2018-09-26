use reqwest;
use serde_json;
use url;

error_chain!{
    foreign_links {
        Http(reqwest::Error);
        Json(serde_json::Error);
        Url(url::ParseError);
    }

    errors {
        Client(status: u16, msg:String) {
            description("could not connect")
            display("could not connect {}: {}", status, msg)
        }
        Mime(mime: String) {
            description("invalid mime type")
            display("invalid mime type: {}", mime)
        }
    }
}
