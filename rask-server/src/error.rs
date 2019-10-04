use crate::group;
use reqwest::Error as ReqError;
use std::sync::mpsc::SendError;

#[derive(Debug)]
pub enum ServerError {
    Group(String),
    GroupCreation(String),
    GameCreation(std::io::Error),
    WebsocketCreation(std::io::Error),
    BackendRequest(ReqError),
    InvalidProtocol,
    InvalidTokenFormat,
    InvalidToken(String),
    StdErr(Box<dyn std::error::Error>),
    MessageSend(SendError<group::Message>),
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ServerError::Group(e) => write!(f, "GroupError: {}", e),
            ServerError::GroupCreation(e) => write!(f, "GroupCreationError: {}", e),
            ServerError::GameCreation(e) => write!(f, "GameCreationError: {}", e),
            ServerError::WebsocketCreation(e) => write!(f, "WebsocketCreationError: {}", e),
            ServerError::BackendRequest(e) => write!(f, "BackendRequestError: {}", e),
            ServerError::InvalidProtocol => write!(f, "InvalidProtocolError"),
            ServerError::InvalidTokenFormat => write!(f, "InvalidTokenFormat"),
            ServerError::InvalidToken(e) => write!(f, "InvalidTokenError: {}", e),
            ServerError::StdErr(e) => write!(f, "StdErrorError: {}", e),
            ServerError::MessageSend(e) => write!(f, "MessageSendError: {}", e),
        }
    }
}
impl From<Box<dyn std::error::Error>> for ServerError {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        ServerError::StdErr(e)
    }
}

impl From<ReqError> for ServerError {
    fn from(e: ReqError) -> Self {
        ServerError::BackendRequest(e)
    }
}

impl From<SendError<group::Message>> for ServerError {
    fn from(e: SendError<group::Message>) -> Self {
        ServerError::MessageSend(e)
    }
}

impl std::error::Error for ServerError {}
