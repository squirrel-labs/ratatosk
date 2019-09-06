use crate::error::ServerError;
use crate::group::GroupId;
use serde::{Deserialize, Serialize};

const API_ENDPOINT: &str = "https://games.kobert.dev/";

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    #[serde(rename = "hasPassword")]
    pub password: bool,  
    #[serde(rename = "maxUsers")]
    pub user_max: u32,  
    #[serde(rename = "userCount")]
    pub user_count: u32,  
    #[serde(rename = "id")]
    pub group_id: GroupId,  //Id
    #[serde(rename = "type")]
    pub group_type: String, //Type
    #[serde(rename = "name")]
    pub group_name: String,  //Name
}

pub fn request(location: &str) -> String {
    match reqwest::get(location) {
        Ok(mut res) => res.text().unwrap(),
        Err(err) => format!("{}", err),
    }
}

pub fn verify_token(token: i32) -> Result<TokenResponse, ServerError> {
    let res: Result<TokenResponse, reqwest::Error> =
        match reqwest::get(format!("{}api/lobby/tokens/{}", API_ENDPOINT, token).as_str()) {
            Ok(mut res) => res.json(),
            Err(_) => return Err(ServerError::InvalidToken),
        };
    res.map_err(|e| {warn!("{}", e); ServerError::InvalidTokenFormat})
}
