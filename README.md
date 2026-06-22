# egui-todo-app

A simple ToDo list application built with egui (Rust).

## Features

- Create todo items (with priority P1-P5)
- Edit todo items (disabled when readonly)
- List todo items (shows priority, readonly status, tags)
- Show detail todo item (displays priority, readonly status, tags)
- Delete todo items (soft delete)
- Mark items as completed (dynamic "Mark Done"/"Mark Pending" button)
- Mark items as readonly (dynamic "Lock"/"Unlock" button)
- Reorder todo items (^ and v buttons in list view)
- Tags support (add/remove tags as chips, only used tags shown in filter)
- Filter by tags (checkbox filters, only shows tags assigned to at least one active todo)
- Search todo items (by title or description, combined with tag filters)
- Change priority in detail view (dropdown, disabled when readonly)
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

CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE todo_tags (
    todo_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (todo_id, tag_id),
    FOREIGN KEY (todo_id) REFERENCES todos(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
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