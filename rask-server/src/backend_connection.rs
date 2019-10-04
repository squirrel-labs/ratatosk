use crate::error::ServerError;
use crate::group::GroupId;
use serde::{Deserialize, Serialize};

const API_ENDPOINT: &str = "https://games.kobert.dev/";

/// The group information send as response to
/// a token request.
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    #[serde(rename = "hasPassword")]
    pub password: bool,
    #[serde(rename = "maxUsers")]
    pub user_max: u32,
    #[serde(rename = "userCount")]
    pub user_count: u32,
    #[serde(rename = "id")]
    pub group_id: GroupId,
    #[serde(rename = "type")]
    pub group_type: String,
    #[serde(rename = "name")]
    pub group_name: String,
    #[serde(rename = "username")]
    pub user_name: String,
}

/// Make a plaintext get request to API_ENDPOINT/{location}
pub fn request(location: &str) -> Option<String> {
    let uri = &format!("{}{}", API_ENDPOINT, location);
    reqwest::get(uri)
        .and_then(|mut res| res.text())
        .map_err(|err| log::warn!("request on \"{}\" failed: {}", uri, err))
        .ok()
}

/// Verify the token validity
pub fn verify_token(token: i32) -> Result<TokenResponse, ServerError> {
    let mut res = match reqwest::get(format!("{}api/lobby/tokens/{}", API_ENDPOINT, token).as_str())
    {
        Ok(res) => res,
        Err(e) => return Err(ServerError::BackendRequest(e)),
    };
    let token_res: Result<TokenResponse, reqwest::Error> = res.json();
    token_res.map_err(|e| {
        warn!("{}", e);
        ServerError::InvalidToken(format!(
            "The Backend Response did not contain valid group Information: {:?}",
            res.text()
        ))
    })
}
