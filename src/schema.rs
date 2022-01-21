table! {
    notes (note_id) {
        note_id -> Uuid,
        title -> Nullable<Varchar>,
        description -> Text,
        note_created -> Nullable<Timestamp>,
    }
}
