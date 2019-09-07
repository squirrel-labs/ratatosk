use reqwest::{Error as ReqError};

#[derive(Debug)]
pub enum ServerError {
    HandshakeRequest,
    Group(String),
    GroupCreation(String),
    WebsocketCreation(std::io::Error),
    BackendRequest(ReqError),
    InvalidProtocol,
    InvalidTokenFormat,
    InvalidToken,
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ServerError::HandshakeRequest => write!(f, "HandshakeRequestError"),
            ServerError::Group(e) => write!(f, "GroupError: {}", e),
            ServerError::GroupCreation(e) => write!(f, "GroupCreationError: {}", e),
            ServerError::WebsocketCreation(e) => write!(f, "WebsocketCreationError: {}", e),
            ServerError::BackendRequest(e) => write!(f, "BackendRequestError: {}", e),
            ServerError::InvalidProtocol => write!(f, "InvalidProtocolError"),
            ServerError::InvalidTokenFormat => write!(f, "InvalidTokenFormat"),
            ServerError::InvalidToken => write!(f, "InvalidTokenError"),
        }
    }
}


impl From<ReqError> for ServerError {
    fn from(e: ReqError) -> Self {
        ServerError::BackendRequest(e)
    }
}
