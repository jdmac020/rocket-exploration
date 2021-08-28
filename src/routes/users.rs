use rocket::*;

#[get("/users")]
pub fn get_users_fn() -> String {
    "List of users".to_string()
}

#[post("/users")]
pub fn create_user_fn() -> String {
    "User created!".to_string()
}