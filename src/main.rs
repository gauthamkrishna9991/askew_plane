#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use rocket_sync_db_pools::{database, diesel as rc_diesel};

extern crate chrono;
extern crate uuid;

mod models;
mod schema;
mod api;

#[database("psql")]
pub struct PgDB(rc_diesel::PgConnection);

#[get("/")]
pub fn hello() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PgDB::fairing())
        .attach(api::api_stage())
        .mount("/", routes![hello])
}
