use crate::models::{Game, GameOverview, GameType, TokenResponse};
use rocket::get;
use rocket_contrib::json::Json;

// routes are here for now :)
#[get("/")]
pub fn index() -> &'static str {
    "Hello, rask!"
}

#[get("/api/lobby", format = "json")]
pub fn game_index() -> Json<GameOverview> {
    let mock_data = GameOverview {
        game_types: vec![
            (GameType {
                name: "rask".to_string(),
                icon: "./resources/icon_rask.png".to_string(),
                display_name: "Rask".to_string(),
            }),
        ],
        games: vec![
            (Game {
                name: "Rask".to_string(),
                type_: "rask".to_string(),
                id: 1,
                max_users: 5,
                user_count: 0,
                has_password: true,
            }),
        ],
    };

    Json(mock_data)
}
use std::time::{SystemTime, UNIX_EPOCH};

#[get("/api/lobby/tokens/<token>", format = "json")]
pub fn token_request(token: u32) -> Result<Json<TokenResponse>, rocket::http::Status> {
    if token != 42 {
        return Err(rocket::http::Status::new(
            404,
            "The requested Token is not valid",
        ));
    }
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let mock_data = TokenResponse {
        username: format!("Anonymous{:?}", since_the_epoch),
        name: "Rask".into(),
        type_: "rask".into(),
        id: 1,
        max_users: 5,
        user_count: 0,
        has_password: true,
    };

    Ok(Json(mock_data))
}
