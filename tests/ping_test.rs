use rocket::http::Status;

mod common;

#[test]
fn ping_test() {
    let client = common::setup();
    let mut response = client.get("/ping").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("PONG!".into()));
}