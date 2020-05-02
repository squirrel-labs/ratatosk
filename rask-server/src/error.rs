use crate::group;
use rask_engine::error;
use reqwest::Error as ReqError;
use std::sync::mpsc::SendError;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ServerError {
    Group(String),
    GroupCreation(String),
    GameCreation(std::io::Error),
    WebsocketCreation(std::io::Error),
    WebsocketError(ws::Error),
    BackendRequest(ReqError),
    InvalidProtocol,
    InvalidTokenFormat,
    InvalidToken(String),
    StdErr(Box<dyn std::error::Error>),
    MessageSend(SendError<group::Message>),
    RaskError(error::EngineError),
    FileError(std::io::Error),
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ServerError::Group(e) => write!(f, "GroupError: {}", e),
            ServerError::GroupCreation(e) => write!(f, "GroupCreationError: {}", e),
            ServerError::GameCreation(e) => write!(f, "GameCreationError: {}", e),
            ServerError::WebsocketCreation(e) => write!(f, "WebsocketCreationError: {}", e),
            ServerError::WebsocketError(e) => write!(f, "WebsocketError: {}", e),
            ServerError::BackendRequest(e) => write!(f, "BackendRequestError: {}", e),
            ServerError::InvalidProtocol => write!(f, "InvalidProtocolError"),
            ServerError::InvalidTokenFormat => write!(f, "InvalidTokenFormat"),
            ServerError::InvalidToken(e) => write!(f, "InvalidTokenError: {}", e),
            ServerError::StdErr(e) => write!(f, "StdErrorError: {}", e),
            ServerError::MessageSend(e) => write!(f, "MessageSendError: {}", e),
            ServerError::RaskError(e) => write!(f, "RaskError: {}", e),
            ServerError::FileError(e) => write!(f, "FileError: {}", e),
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
derive_from!(ws::Error, WebsocketError);
derive_from!(std::io::Error, FileError);
derive_from!(Box<dyn std::error::Error>, StdErr);
derive_from!(ReqError, BackendRequest);
derive_from!(SendError<group::Message>, MessageSend);

impl std::error::Error for ServerError {}
