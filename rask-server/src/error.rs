use crate::group;
use rask_engine::error;
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
    RaskError(error::EngineError),
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
            ServerError::RaskError(e) => write!(f, "RaskError: {}", e),
        }
    }
}

macro_rules! derive_from {
    ($type:ty, $kind:ident) => {
        impl From<$type> for ServerError {
            fn from(error: $type) -> Self {
                ServerError::$kind(error)
            }
        }
    };
}
derive_from!(Box<dyn std::error::Error>, StdErr);
derive_from!(ReqError, BackendRequest);
derive_from!(SendError<group::Message>, MessageSend);

impl std::error::Error for ServerError {}
