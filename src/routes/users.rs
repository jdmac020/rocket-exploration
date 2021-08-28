use rocket::*;

#[get("/users")]
pub fn get_users_rt() -> String {
    "List of users".to_string()
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