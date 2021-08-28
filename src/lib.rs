#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_attributes)]

#[macro_use] use rocket::*;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::helmet::SpaceHelmet;

mod routes;

pub fn rocket_builder() -> rocket::Rocket {
    rocket::ignite().attach(SpaceHelmet::default())
    .mount("/", routes![routes::ping::ping_fn])
    .mount("/api", routes![
        routes::users::get_users_rt,
        routes::users::create_user_rt,
        routes::users::get_user_rt,
        routes::users::update_user_rt,
        routes::users::delete_user_rt
    ])
    .mount("/files", StaticFiles::from("static/"))
}
