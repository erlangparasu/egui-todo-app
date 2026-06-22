-- 001: Initial schema
CREATE TABLE IF NOT EXISTS todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    completed INTEGER DEFAULT 0,
    creation_date INTEGER NOT NULL,
    changed_date INTEGER NOT NULL,
    deletion_date INTEGER
);