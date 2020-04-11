#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

use std::vec::Vec;

use rocket::fairing::AdHoc;
use rocket::http::Header;
use rocket_contrib::json::Json;

use serde_derive::Serialize;

use rocket::{get,routes};

// this is just here for a POC.
// TODO move those into their own file
#[derive(Debug, Serialize)]
struct GameType {
    name: String,
    icon: String,
    displayName: String,
}

#[derive(Debug, Serialize)]
struct Game {
    name: String,
    type_: String,
    id: u8,
    maxUsers: u8,
    userCount: u8,
    hasPassword: bool,
}

#[derive(Debug, Serialize)]
struct GameOverview {
    gameTypes: Vec<GameType>,
    games: Vec<Game>,
}

// routes are here for now :)
#[get("/")]
fn index() -> &'static str {
    "Hello, rask!"
}

#[get("/api/lobby", format = "json")]
fn gameIndex() -> Json<GameOverview> {
    let mock_data = GameOverview {
        gameTypes: vec![
            (GameType {
                name: "rask".to_string(),
                icon: "./resources/icon_rask.png".to_string(),
                displayName: "Rask".to_string(),
            }),
        ],
        games: vec![
            (Game {
                name: "Rask".to_string(),
                type_: "rask".to_string(),
                id: 1,
                maxUsers: 5,
                userCount: 0,
                hasPassword: true,
            }),
        ],
    };

    Json(mock_data)
}

pub fn rocket() -> rocket::Rocket {
    let routes = routes![index, gameIndex];
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
