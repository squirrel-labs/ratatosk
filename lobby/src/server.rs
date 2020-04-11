#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

use std::vec::Vec;

use rocket::fairing::AdHoc;
use rocket::http::Header;
use rocket_contrib::json::Json;

use serde_derive::Serialize;

use rocket::{get, routes};

// this is just here for a POC.
// TODO move those into their own file
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GameType {
    name: String,
    icon: String,
    display_name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Game {
    name: String,
    type_: String,
    id: u8,
    max_users: u8,
    user_count: u8,
    has_password: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GameOverview {
    game_types: Vec<GameType>,
    games: Vec<Game>,
}

// routes are here for now :)
#[get("/")]
fn index() -> &'static str {
    "Hello, rask!"
}

#[get("/api/lobby", format = "json")]
fn game_index() -> Json<GameOverview> {
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

pub fn rocket() -> rocket::Rocket {
    let routes = routes![index, game_index];
    rocket::ignite()
        .mount("/", routes)
        .attach(AdHoc::on_response("CORS header for dev env", |req, res| {
            #[cfg(debug_assertions)]
            res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        }))
}

pub fn main() {
    rocket().launch();
}
