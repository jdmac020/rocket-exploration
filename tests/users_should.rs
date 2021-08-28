
use rocket_explore::rocket_builder;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn get_users() {
    let client = Client::new(rocket_builder()).expect("Valid Rocket instance");
    let mut response = client.get("/users").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("List of users".into()));
}