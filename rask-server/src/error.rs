use reqwest::Error as ReqError;

#[derive(Debug)]
pub enum ServerError {
    Group(String),
    GroupCreation(String),
    WebsocketCreation(std::io::Error),
    BackendRequest(ReqError),
    InvalidProtocol,
    InvalidTokenFormat,
    InvalidToken(String),
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ServerError::Group(e) => write!(f, "GroupError: {}", e),
            ServerError::GroupCreation(e) => write!(f, "GroupCreationError: {}", e),
            ServerError::WebsocketCreation(e) => write!(f, "WebsocketCreationError: {}", e),
            ServerError::BackendRequest(e) => write!(f, "BackendRequestError: {}", e),
            ServerError::InvalidProtocol => write!(f, "InvalidProtocolError"),
            ServerError::InvalidTokenFormat => write!(f, "InvalidTokenFormat"),
            ServerError::InvalidToken(e) => write!(f, "InvalidTokenError: {}", e),
        }
    }
}

impl From<ReqError> for ServerError {
    fn from(e: ReqError) -> Self {
        ServerError::BackendRequest(e)
    }
}
