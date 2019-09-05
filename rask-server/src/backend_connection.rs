use crate::error::ServerError;
use crate::group::GroupId;
use std::collections::HashMap;

const API_ENDPOINT: &str = "https://games.kobert.dev/api/";

pub struct TokenResponse {
    pub group_id: GroupId,  //Id
    pub group_type: String, //Type
    pub user_name: String,  //Name
}

pub fn request(location: &str) -> String {
    match reqwest::get(location) {
        Ok(mut res) => res.text().unwrap(),
        Err(err) => format!("{}", err),
    }
}

pub fn verify_token(token: i32) -> Result<TokenResponse, ServerError> {
    let res: HashMap<String, String> =
        match reqwest::get(format!("{}tokens/{}", API_ENDPOINT, token).as_str()) {
            Ok(mut res) => res.json()?,
            Err(_) => return Err(ServerError::InvalidToken),
        };
    let to_find = ["Id", "Type", "Name"];
    if !to_find.iter().all(|x| res.contains_key(&x.to_string())) {
        return Err(ServerError::InvalidTokenFormat);
    }
    if let Ok(id) = (*res.get("Id").unwrap()).parse() {
        return Ok(TokenResponse {
            group_id: id,
            group_type: res.get("Type").unwrap().to_string(),
            user_name: res.get("Name").unwrap().to_string(),
        });
    } else {
        return Err(ServerError::InvalidTokenFormat);
    }
}
