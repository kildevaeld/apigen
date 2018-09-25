use futures::{Future, Stream};
use reqwest::async::{Client, Decoder};
use reqwest::header::CONTENT_TYPE;
use reqwest::{Method, Request};
use std::mem;

use api_support::error;
use api_support::utils;


pub struct Blog {
    client: Client,
    endpoint: String,
}

impl Blog {
    pub fn new(endpoint: &str) -> Blog {
        Blog::with_client(Client::new(), endpoint)
    }

    pub fn with_client(client: Client, endpoint: &str) -> Blog {
        Blog{
            client: client,
            endpoint: String::from(endpoint),
        }
    }
    
      
  
  pub fn blog(&self, ) -> impl Future<Item=, Error=error::Error> {
  
      let url = utils::join(&self.endpoint, &["blog"])?;
      let mut request = client.request(Method::GET, url);
      
      
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
                      Ok(utils::decode::<>(&mime, &b)?)
                  })
                  
          })
          .from_err()*/
  }
    
      
  
  pub fn blog(&self, body: Anonym) -> impl Future<Item=, Error=error::Error> {
  
      let url = utils::join(&self.endpoint, &["blog"])?;
      let mut request = client.request(Method::POST, url);
      
      
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
                      Ok(utils::decode::<>(&mime, &b)?)
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
