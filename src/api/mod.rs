use rocket::fairing::AdHoc;

mod note;

use note::{
    create, create_nodb, delete_by_id, delete_by_id_nodb, get_all, get_all_nodb, get_by_id, update,
};

pub fn api_stage() -> AdHoc {
    AdHoc::on_ignite("API", |rocket| async {
        rocket.mount(
            "/api/note",
            routes![
                create,
                create_nodb,
                get_by_id,
                get_all,
                get_all_nodb,
                update,
                delete_by_id,
                delete_by_id_nodb,
            ],
        )
    })
}
