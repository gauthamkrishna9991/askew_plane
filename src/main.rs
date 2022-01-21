/// Askew_Plane: REST API with Rust using Rocket Framework
///
///     Copyright (C) 2021  Goutham Krishna K V
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU Affero General Public License as published
/// by the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
/// 
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU Affero General Public License for more details.
/// 
/// You should have received a copy of the GNU Affero General Public License
/// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
