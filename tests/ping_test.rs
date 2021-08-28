
use rocket_explore::rocket_builder;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn ping_test() {
    let client = Client::new(rocket_builder()).expect("Valid Rocket instance");
    let mut response = client.get("/ping").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("PONG!".into()));
}