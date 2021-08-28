#![feature(proc_macro_hygiene, decl_macro)]

use std::path::{Path, PathBuf};

#[macro_use] use rocket::*;
use rocket_contrib::serve::StaticFiles;

#[get("/echo/<echo>")]
fn echo_fn(echo: String) -> String {
    format!("{}", echo)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![echo_fn])
    .mount("/files", StaticFiles::from("static/"))
    .launch();
}
