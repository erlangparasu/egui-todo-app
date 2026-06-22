# egui-todo-app

A simple ToDo list application built with egui (Rust).

## Features

- Create todo items
- Edit todo items
- List todo items
- Show detail todo item
- Delete todo items (soft delete)
- Mark items as completed
- Mark items as readonly (lock editing)
- Trash menu with restore/permanent delete
- Export database to file

## Database Schema

The app uses SQLite with the following schema:

```sql
CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    completed INTEGER DEFAULT 0,
    readonly INTEGER DEFAULT 0,
    creation_date INTEGER NOT NULL,
    changed_date INTEGER NOT NULL,
    deletion_date INTEGER
);
```

## Run

```bash
cargo run
```

## Build

```bash
cargo build
```