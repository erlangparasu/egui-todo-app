# SQLite Persistence Plan

## Current State
- In-memory storage using `Vec<TodoItem>` for tasks and trashed_tasks
- App state lost on restart

## Goal
Add SQLite database for persistent storage

## Analysis

### Database Schema
```sql
CREATE TABLE todos (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    completed INTEGER DEFAULT 0,
    creation_date INTEGER NOT NULL,
    changed_date INTEGER NOT NULL,
    deletion_date INTEGER
);
```

### Required Changes

#### 1. Dependencies
- [x] Add `rusqlite` with `bundled` feature to Cargo.toml

#### 2. Database Module (src/database.rs)
- [x] Initialize database connection
- [x] Create table if not exists
- [x] Load all todos on app start
- [x] Insert new todo
- [x] Update existing todo
- [x] Soft delete (set deletion_date)
- [x] Restore todo (clear deletion_date)
- [x] Permanent delete
- [x] Empty trash

#### 3. TodoItem Struct
- [x] Keep as-is (matches schema)

#### 4. TodoApp Struct
- [x] Add `conn: Connection` field
- [x] Remove `next_id` field (use SQLite auto-increment)
- [x] Remove in-memory `tasks` and `trashed_tasks` initialization

#### 5. CRUD Operations - Update calls
- [x] Create: Insert into DB, reload list
- [x] Edit: Update in DB
- [x] Toggle Done: Update in DB
- [x] Soft Delete: Update deletion_date in DB
- [x] Restore: Clear deletion_date in DB
- [x] Permanent Delete: Delete from DB
- [x] Empty Trash: Delete all trashed from DB

#### 6. Load Data on Startup
- [x] Load active todos (deletion_date IS NULL)
- [x] Load trashed todos (deletion_date IS NOT NULL)

## Implementation Order
1. [x] Create database.rs with connection and schema
2. [x] Update Cargo.toml
3. [x] Refactor TodoApp to use DB connection
4. [x] Update all CRUD operations to persist
5. [x] Test create, edit, delete, restore, empty trash

## Done! SQLite persistence added.