#![feature(proc_macro_hygiene)]
use lobby::routes::*;

use rocket::fairing::AdHoc;
use rocket::http::{ContentType, Header, Status};
use rocket::local::Client;

#[test]
fn test_index_route() {
    let routes = rocket::routes![index, game_index, token_request];
    let rocket = rocket::ignite()
        .mount("/", routes)
        .attach(AdHoc::on_response("CORS header for dev env", |req, res| {
            #[cfg(debug_assertions)]
            res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        }));

    let client = Client::new(rocket).expect("rocket instance");
    let mut response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, rask!".into()));
}
