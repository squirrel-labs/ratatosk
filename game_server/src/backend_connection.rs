use reqwest::{Response, Client, Url, UrlError, Error as ReqError};

pub struct BackendConnection {
    host: String,
    client: Client,
    last_response: Option<Result<Response, ReqError>>
}

impl BackendConnection {
    pub fn new(host: &str) -> Self {
        BackendConnection {
            host: host.to_string(),
            client: Client::new(),
            last_response: None
        }
    }

    pub fn request(&mut self, location: &str) -> Result<(), UrlError> {
        Ok(self.last_response =
                Some(self.client.get(Url::parse(&format!("{}{}", self.host, location))?)
                           .send()))
    }

    pub fn get_response(&self) -> &Option<Result<Response, ReqError>> {
        &self.last_response
    }
    
    pub fn host_name<'a>(&'a self) -> &'a str {
        &self.host
    }
}
