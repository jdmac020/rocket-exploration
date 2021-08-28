use rocket::*;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    status: String,
    message: String,
}
impl Response {
    fn ok(msg: &str) -> Self {
        Response {
            status: "Success".to_string(),
            message: msg.to_string(),
        }
    }
    fn err(msg: &str) -> Self {
        Response {
            status: "Error".to_string(),
            message: msg.to_string(),
        }
    }
}

#[get("/users")]
pub fn get_users_rt() -> Json<Response> {
    Json(Response::ok("List of users"))
}

#[post("/users")]
pub fn create_user_rt() -> String {
    "User created!".to_string()
}

#[get("/users/<id>")]
pub fn get_user_rt(id: String) -> String {
    format!("Details on User {}", id)
}

#[put("/users/<id>")]
pub fn update_user_rt(id: String) -> String {
    format!("Updated User {}", id)
}

#[delete("/users/<id>")]
pub fn delete_user_rt(id: String) -> String {
    format!("Deleted User {}", id)
}