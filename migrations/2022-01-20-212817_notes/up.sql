-- Create Notes Database
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE notes (
    note_id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR,
    description TEXT NOT NULL,
    note_created TIMESTAMP DEFAULT NOW()
);
