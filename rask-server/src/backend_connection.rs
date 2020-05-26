use crate::error::ServerError;
use crate::group::GroupId;
use log::warn;
use serde::{Deserialize, Serialize};

const API_ENDPOINT: &str = "http://localhost:8000/";

/// The group information sent as response to a token request.
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

#[allow(dead_code)]
/// Make a plaintext get request to API_ENDPOINT/{location}.
pub fn request(location: &str) -> Option<String> {
    let uri = &format!("{}{}", API_ENDPOINT, location);
    reqwest::get(uri)
        .and_then(|mut res| res.text())
        .map_err(|err| log::warn!("request on \"{}\" failed: {}", uri, err))
        .ok()
}

/// Verify the token validity.
pub fn verify_token(token: i32) -> Result<TokenResponse, ServerError> {
    let mut res = reqwest::get(&format!("{}api/lobby/tokens/{}", API_ENDPOINT, token))
        .map_err(ServerError::BackendRequest)?;
    let token_res: Result<TokenResponse, _> = res.json();
    token_res.map_err(|e| {
        warn!("{}", e);
        ServerError::InvalidToken(format!(
            "The Backend Response did not contain valid group information: {:?}",
            res.text()
        ))
    })
}
