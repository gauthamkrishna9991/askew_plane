use rocket::serde::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::PgConnection;
use std::time::SystemTime;
use uuid::Uuid;

use crate::schema::notes;
use crate::schema::notes::dsl::notes as notes_schema;

#[derive(Queryable, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Note {
    note_id: Uuid,
    title: Option<String>,
    description: String,
    note_created: Option<SystemTime>,
}

impl Note {
    pub fn get(conn: &PgConnection) -> Result<Vec<Note>, diesel::result::Error> {
        notes_schema.load::<Self>(conn)
    }

    pub fn get_by_uid(conn: &PgConnection, id: Uuid) -> Result<Note, diesel::result::Error> {
        notes_schema.find(id).first::<Self>(conn)
    }

    pub fn delete(conn: &PgConnection, id_to_delete: uuid::Uuid) -> Result<Note, diesel::result::Error> {
        diesel::delete(notes_schema.find(id_to_delete)).get_result(conn)
    }

    pub fn update(self, conn: &PgConnection) -> Result<Note, diesel::result::Error> {
        diesel::update(notes_schema.find(self.note_id.clone())).set((
            notes::title.eq(self.title),
            notes::description.eq(self.description)
        )).get_result::<Note>(conn)
    }
}

#[derive(Insertable, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
#[table_name = "notes"]
pub struct NewNote {
    title: String,
    description: String,
}

impl NewNote {
    pub fn create(self, conn: &PgConnection) -> Result<Note, diesel::result::Error> {
        diesel::insert_into(notes_schema)
            .values(&self)
            .get_result::<Note>(conn)
    }
}
