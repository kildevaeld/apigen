use futures::{Future, Stream};
use reqwest::async::{Client, Decoder};
use reqwest::header::CONTENT_TYPE;
use reqwest::{Method, Request};
use std::mem;

use api_support::error;
use api_support::utils;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct 400Rest {
  test: String
}


pub struct Test {
    client: Client,
    endpoint: String,
}

impl Test {
    pub fn new(endpoint: &str) -> Test {
        Test::with_client(Client::new(), endpoint)
    }

    pub fn with_client(client: Client, endpoint: &str) -> Test {
        Test{
            client: client,
            endpoint: String::from(endpoint),
        }
    }
    
      
  
  pub fn test_mig_ioret_ost(&self, mig_ioret: &str, body: ) -> impl Future<Item=Test<String>, Error=error::Error> {
  
      let url = utils::join(&self.endpoint, &["test", mig_ioret, "ost"])?;
      let mut request = client.request(Method::GET, url);
      
      
      let body_data = utils::encode("application/json", body)?;
      request = request.body(body_data);
      
      request = request.header(CONTENT_TYPE, "application/json");
  
      self.request(request)
  
      /*self.client
          .execute(request.build().unwrap())
          .from_err::<error::Error>()
          .and_then(|mut res| {
              let status = res.status().as_u16();
              let body = mem::replace(res.body_mut(), Decoder::empty());
  
              let mime = res
                  .headers()
                  .get(CONTENT_TYPE)
                  .and_then(|ct| ct.to_str().ok())
                  .unwrap_or("application/json")
                  .to_string();
  
              body.concat2()
                  .from_err::<error::Error>()
                  .and_then(move |b| {
                      Ok(utils::decode::<Test<String>>(&mime, &b)?)
                  })
                  
          })
          .from_err()*/
  }
    

    fn request<T: Serializable + ?Size>(request: Request) -> impl Future<Item=T, Error=error::Error> {
        self.client
        .execute(request.build().unwrap())
        .from_err::<error::Error>()
        .and_then(|mut res| {
            let status = res.status().as_u16();
            let body = mem::replace(res.body_mut(), Decoder::empty());

            let mime = res
                .headers()
                .get(CONTENT_TYPE)
                .and_then(|ct| ct.to_str().ok())
                .unwrap_or("application/json")
                .to_string();

            if res.is_success {
                return body.concat2()
                .from_err::<error::Error>()
                .and_then(move |b| Ok(utils::decode::<T>(&mime, &b)?));
            }

            Err(error::Error::Invalid)
        })
        .from_err()
    }
}
