#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] use rocket::*;

#[get("/echo/<echo>")]
fn echo_fn(echo: String) -> String {
    format!("{}", echo)
}

fn main() {
    rocket::ignite().mount("/", routes![echo_fn]).launch();
}
