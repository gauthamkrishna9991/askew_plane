use rocket::serde::{json::Json, Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{NewNote, Note};
use crate::PgDB;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NoteResult {
    status: bool,
    note: Option<Note>,
    error: Option<String>,
}

// -- CREATE --

#[post("/", data = "<new_note>")]
pub async fn create(conn: PgDB, new_note: Json<NewNote>) -> Json<NoteResult> {
    conn.run(move |c| match new_note.clone().create(c) {
        Ok(new_note) => Json::from(NoteResult {
            status: true,
            note: Some(new_note),
            error: None,
        }),
        Err(new_note_err) => Json::from(NoteResult {
            status: false,
            note: None,
            error: Some(format!("NEW_NOTE_ERR: {}", new_note_err)),
        }),
    })
    .await
}

#[post("/", rank = 2)]
pub async fn create_nodb() -> Json<NoteResult> {
    Json::from(NoteResult {
        status: false,
        note: None,
        error: Some("DATABASE_NOT_CONNECTED".to_string()),
    })
}

// -- READ --

#[get("/<id>")]
pub async fn get_by_id(conn: PgDB, id: String) -> Json<NoteResult> {
    conn.run(move |c| match Uuid::parse_str(id.as_str()) {
        Ok(note_uid) => match Note::get_by_uid(c, note_uid) {
            Ok(note) => Json::from(NoteResult {
                status: true,
                note: Some(note),
                error: None,
            }),
            Err(note_retrieve_err) => Json::from(NoteResult {
                status: false,
                note: None,
                error: Some(format!("NOTE_RETRIEVE_ERROR: {}", note_retrieve_err)),
            }),
        },
        Err(uuid_parse_err) => Json::from(NoteResult {
            status: false,
            note: None,
            error: Some(format!("UUID_PARSE_ERROR: {}", uuid_parse_err)),
        }),
    })
    .await
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NotesResult {
    status: bool,
    notes: Option<Vec<Note>>,
    error: Option<String>,
}

#[get("/")]
pub async fn get_all(conn: PgDB) -> Json<NotesResult> {
    conn.run(move |c| match Note::get(c) {
        Ok(notes) => Json::from(NotesResult {
            status: true,
            notes: Some(notes),
            error: None,
        }),
        Err(err) => Json::from(NotesResult {
            status: false,
            notes: None,
            error: Some(format!("{}", err)),
        }),
    })
    .await
}

#[get("/", rank = 2)]
pub async fn get_all_nodb() -> Json<NotesResult> {
    Json::from(NotesResult {
        status: false,
        notes: None,
        error: Some("DATABASE_NOT_CONNECTED".to_string()),
    })
}

// -- UPDATE --

#[put("/", data = "<updated_note>")]
pub async fn update(conn: PgDB, updated_note: Json<Note>) -> Json<NoteResult> {
    conn.run(move |c| match updated_note.clone().update(c) {
        Ok(updated_note) => Json::from(NoteResult {
            status: true,
            note: Some(updated_note),
            error: None,
        }),
        Err(update_note_err) => Json::from(NoteResult {
            status: false,
            note: None,
            error: Some(format!("UPDATE_NOTE_ERROR: {}", update_note_err)),
        }),
    })
    .await
}

// -- DELETE --

#[delete("/<id>")]
pub async fn delete_by_id(conn: PgDB, id: String) -> Json<NoteResult> {
    conn.run(move |c| match Uuid::parse_str(id.as_str()) {
        Ok(note_uid) => match Note::delete(c, note_uid) {
            Ok(note) => Json::from(NoteResult {
                status: true,
                note: Some(note),
                error: None,
            }),
            Err(delete_note_err) => Json::from(NoteResult {
                status: false,
                note: None,
                error: Some(format!("DELETE_NOTE_ERR: {}", delete_note_err)),
            }),
        },
        Err(uuid_parse_error) => Json::from(NoteResult {
            status: false,
            note: None,
            error: Some(format!("UUID_PARSE_ERROR: {}", uuid_parse_error)),
        }),
    })
    .await
}

#[delete("/", rank = 2)]
pub async fn delete_by_id_nodb() -> Json<NoteResult> {
    Json::from(NoteResult {
        status: false,
        note: None,
        error: Some("DATABASE_NOT_CONNECTED".to_string()),
    })
}
