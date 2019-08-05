use crate::group::GroupId;
use crate::server::{Token, UserId};
use reqwest::{Client, Error as ReqError, Response, Url, UrlError};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub struct BackendConnection {
    host: String,
    req_sender: Sender<RequestData>,
    res_rec: Receiver<ResponseResult>,
    max_uid: u32,
}

#[derive(Debug)]
pub enum BackendError {
    UrlError(UrlError),
    RequestError(ReqError),
    InvalidTokenFormat,
    InvalidToken,
    BadResponse(Response),
}

pub type TokenValidity = Result<TokenResponse, BackendError>;
pub type RequestData = Url;
pub type ResponseResult = Result<Response, ReqError>;

pub struct TokenResponse {
    pub group_id: GroupId,
    pub group_type: String,
    pub group_name: String,
    pub user_id: UserId,
}

impl BackendConnection {
    fn run_background(req_rec: Receiver<RequestData>, res_sender: Sender<ResponseResult>) {
        let client = Client::new();
        loop {
            let request_data = req_rec.recv().unwrap();
            let location = request_data;
            let request = client.get(location);
            let response = request.send();
            res_sender.send(response).unwrap();
        }
    }

    pub fn new(host: &str) -> Self {
        let (req_sender, req_rec): (Sender<RequestData>, Receiver<RequestData>) = mpsc::channel();
        let (res_sender, res_rec): (Sender<ResponseResult>, Receiver<ResponseResult>) =
            mpsc::channel();
        std::thread::spawn(move || Self::run_background(req_rec, res_sender));
        BackendConnection {
            host: host.to_string(),
            req_sender,
            res_rec,
            max_uid: 420,
        }
    }

    pub fn request(&self, location: &str) -> Result<(), UrlError> {
        Ok(self
            .req_sender
            .send(Url::parse(&format!("{}{}", self.host, location))?)
            .unwrap())
    }

    pub fn get_response(&self) -> ResponseResult {
        self.res_rec.recv().unwrap()
    }

    pub fn validate_token(&mut self, token: &Token) -> TokenValidity {
        let location = format!("/api/lobby/tokens/{}", token);
        self.request(&location)
            .map_err(|err| BackendError::UrlError(err))?;
        let response = self
            .get_response()
            .map_err(|err| BackendError::RequestError(err))?;
        match response.status() {
            status if status.is_success() => {
                // zu Testzwecken werden noch keine JSON-Daten deserialisiert
                // Dennis Server gibt ja noch nix zurück
                self.max_uid += 1;
                Ok(TokenResponse {
                    group_id: 12,
                    group_type: "scribble".to_string(),
                    group_name: "Scribble".to_string(),
                    user_id: self.max_uid - 1,
                })
            },
            reqwest::StatusCode::NOT_FOUND => Err(BackendError::InvalidToken),
            status if status.is_client_error() => Err(BackendError::InvalidTokenFormat),
            _ => Err(BackendError::BadResponse(response))
        }
    }
}
