use reqwest::{Error as ReqError, Response, UrlError};

#[derive(Debug)]
pub enum ServerError {
    Bind(std::io::Error),
    HandshakeRequest,
    InvalidProtocol,
    Accept(std::io::Error),
    Group(String),
    GroupCreation(String),

    Url(UrlError),
    BackendRequest(ReqError),
    // todo store token
    InvalidTokenFormat,
    InvalidToken,
    BadBackendResponse(Response),
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ServerError::Bind(e) => write!(f, "BindError: {}", e),
            ServerError::HandshakeRequest => write!(f, "HandshakeRequestError"),
            ServerError::InvalidProtocol => write!(f, "InvalidProtocolError"),
            ServerError::Accept(e) => write!(f, "AcceptError: {}", e),
            ServerError::Group(e) => write!(f, "GroupError: {}", e),
            ServerError::GroupCreation(e) => write!(f, "GroupCreationError: {}", e),

            ServerError::Url(e) => write!(f, "UrlError: {}", e),
            ServerError::BackendRequest(e) => write!(f, "BackendRequest: {}", e),
            ServerError::InvalidTokenFormat => write!(f, "InvalidTokenFormat"),
            ServerError::InvalidToken => write!(f, "InvalidTokenError"),
            ServerError::BadBackendResponse(e) => write!(f, "BadResponseError: {:#?}", e),
        }
    }
}

impl From<UrlError> for ServerError {
    fn from(e: UrlError) -> Self {
        ServerError::Url(e)
    }
}

impl From<ReqError> for ServerError {
    fn from(e: ReqError) -> Self {
        ServerError::BackendRequest(e)
    }
}
