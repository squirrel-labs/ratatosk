use lobby::server::*;

use rocket::http::{ContentType, Status};
use rocket::local::Client;

#[test]
fn test_index_route() {
    let client = Client::new(rocket()).expect("rocket instance");
    let mut response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, rask!".into()));
}
