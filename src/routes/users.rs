use rocket::*;

#[get("/users")]
pub fn get_users_fn() -> String {
    "List of users".to_string()
}