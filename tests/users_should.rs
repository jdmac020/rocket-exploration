use rocket::http::{ContentType, Status};

mod common;

#[test]
fn get_users() {
    let client = common::setup();
    let mut response = client.get("/api/users").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"List of users\"}".into()));
}

#[test]
fn post_users() {
    let client = common::setup();
    let mut response = client.post("/api/users").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"User created!\"}".into()));
}

#[test]
fn get_user_by_id() {
    let client = common::setup();
    let mut response = client.get("/api/users/5").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"Details on User 5\"}".into()));
}

#[test]
fn update_user() {
    let client = common::setup();
    let mut response = client.put("/api/users/15").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"Updated User 15\"}".into()));
}

#[test]
fn delete_user_by_id() {
    let client = common::setup();
    let mut response = client.delete("/api/users/65").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"Deleted User 65\"}".into()));
}