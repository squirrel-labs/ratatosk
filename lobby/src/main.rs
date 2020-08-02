#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;

use crate::routes::*;
use rocket::fairing::AdHoc;
use rocket::http::Header;
use rocket_contrib::databases::diesel as rdiesel;

#[database("postgres")]
struct LobbyBase(rdiesel::PgConnection);

pub fn rocket() -> rocket::Rocket {
    let routes = rocket::routes![index, game_index, token_request];
    rocket::ignite()
        .mount("/", routes)
        .attach(LobbyBase::fairing())
        .attach(AdHoc::on_response(
            "CORS header for dev env",
            |_req, res| {
                #[cfg(debug_assertions)]
                res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            },
        ))
}

pub fn main() {
    rocket().launch();
}
