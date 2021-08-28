
use rocket_explore::rocket_builder;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn get_users() {
    let client = Client::new(rocket_builder()).expect("Valid Rocket instance");
    let mut response = client.get("/api/users").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("List of users".into()));
}

#[test]
fn post_users() {
    let client = Client::new(rocket_builder()).expect("Valid Rocket instance");
    let mut response = client.post("/api/users").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("User created!".into()));
}

#[test]
fn get_user_by_id() {
    let client = Client::new(rocket_builder()).expect("Valid Rocket instance");
    let mut response = client.get("/api/users/5").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Details on User 5".into()));
}

#[test]
fn update_user() {
    let client = Client::new(rocket_builder()).expect("Valid Rocket instance");
    let mut response = client.put("/api/users/15").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Updated User 15".into()));
}

#[test]
fn delete_user_by_id() {
    let client = Client::new(rocket_builder()).expect("Valid Rocket instance");
    let mut response = client.delete("/api/users/65").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Deleted User 65".into()));
}