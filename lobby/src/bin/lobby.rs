#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]
use lobby::routes::*;
use rocket::fairing::AdHoc;
use rocket::http::Header;

pub fn rocket() -> rocket::Rocket {
    let routes = rocket::routes![index, game_index, token_request];
    rocket::ignite()
        .mount("/", routes)
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
