# egui-todo-app

A simple ToDo list application built with egui (Rust).

## Features

- Create todo items (with priority P1-P5)
- Edit todo items (disabled when readonly)
- List todo items (shows priority, readonly status)
- Show detail todo item (displays priority, readonly status)
- Delete todo items (soft delete)
- Mark items as completed
- Mark items as readonly (lock editing)
- Reorder todo items (move up/down)
- Trash menu with restore/permanent delete
- Export database to file (native file dialog, cross-platform)

## Database Schema

```sql
CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    completed INTEGER DEFAULT 0,
    readonly INTEGER DEFAULT 0,
    priority INTEGER DEFAULT 3,
    order_index INTEGER DEFAULT 0,
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