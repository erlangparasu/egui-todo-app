use rusqlite::{Connection, Result};

use crate::TodoItem;

pub fn init_database(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
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
        )",
        [],
    )?;

    Ok(conn)
}

pub fn load_active_todos(conn: &Connection) -> Result<Vec<TodoItem>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, description, completed, readonly, priority, order_index, creation_date, changed_date, deletion_date
         FROM todos WHERE deletion_date IS NULL ORDER BY order_index ASC"
    )?;

    let todos = stmt.query_map([], |row| {
        Ok(TodoItem {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            completed: row.get::<_, i32>(3)? != 0,
            readonly: row.get::<_, i32>(4)? != 0,
            priority: row.get(5)?,
            order_index: row.get(6)?,
            creation_date: row.get(7)?,
            changed_date: row.get(8)?,
            deletion_date: row.get(9)?,
        })
    })?
    .collect::<Result<Vec<_>>>()?;

    Ok(todos)
}

pub fn load_trashed_todos(conn: &Connection) -> Result<Vec<TodoItem>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, description, completed, readonly, priority, order_index, creation_date, changed_date, deletion_date
         FROM todos WHERE deletion_date IS NOT NULL ORDER BY deletion_date DESC"
    )?;

    let todos = stmt.query_map([], |row| {
        Ok(TodoItem {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            completed: row.get::<_, i32>(3)? != 0,
            readonly: row.get::<_, i32>(4)? != 0,
            priority: row.get(5)?,
            order_index: row.get(6)?,
            creation_date: row.get(7)?,
            changed_date: row.get(8)?,
            deletion_date: row.get(9)?,
        })
    })?
    .collect::<Result<Vec<_>>>()?;

    Ok(todos)
}

pub fn insert_todo(conn: &Connection, title: &str, description: &str, creation_date: u64, changed_date: u64) -> Result<usize> {
    conn.execute(
        "INSERT INTO todos (title, description, completed, readonly, creation_date, changed_date) VALUES (?1, ?2, 0, 0, ?3, ?4)",
        [title, description, &creation_date.to_string(), &changed_date.to_string()],
    )?;
    Ok(conn.last_insert_rowid() as usize)
}

pub fn update_todo(conn: &Connection, id: usize, title: &str, description: &str, changed_date: u64) -> Result<()> {
    conn.execute(
        "UPDATE todos SET title = ?1, description = ?2, changed_date = ?3 WHERE id = ?4",
        [title, description, &changed_date.to_string(), &id.to_string()],
    )?;
    Ok(())
}

pub fn toggle_todo(conn: &Connection, id: usize, completed: bool, changed_date: u64) -> Result<()> {
    conn.execute(
        "UPDATE todos SET completed = ?1, changed_date = ?2 WHERE id = ?3",
        [if completed { "1" } else { "0" }, &changed_date.to_string(), &id.to_string()],
    )?;
    Ok(())
}

pub fn set_readonly(conn: &Connection, id: usize, readonly: bool, changed_date: u64) -> Result<()> {
    conn.execute(
        "UPDATE todos SET readonly = ?1, changed_date = ?2 WHERE id = ?3",
        [if readonly { "1" } else { "0" }, &changed_date.to_string(), &id.to_string()],
    )?;
    Ok(())
}

pub fn soft_delete_todo(conn: &Connection, id: usize, deletion_date: u64) -> Result<()> {
    conn.execute(
        "UPDATE todos SET deletion_date = ?1 WHERE id = ?2",
        [&deletion_date.to_string(), &id.to_string()],
    )?;
    Ok(())
}

pub fn restore_todo(conn: &Connection, id: usize, changed_date: u64) -> Result<()> {
    conn.execute(
        "UPDATE todos SET deletion_date = NULL, changed_date = ?1 WHERE id = ?2",
        [&changed_date.to_string(), &id.to_string()],
    )?;
    Ok(())
}

pub fn permanent_delete_todo(conn: &Connection, id: usize) -> Result<()> {
    conn.execute("DELETE FROM todos WHERE id = ?1", [&id.to_string()])?;
    Ok(())
}

pub fn empty_trash(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM todos WHERE deletion_date IS NOT NULL", [])?;
    Ok(())
}

pub fn export_database(db_path: &str, export_path: &str) -> std::io::Result<()> {
    std::fs::copy(db_path, export_path)?;
    Ok(())
}

pub fn get_db_path() -> String {
    "todo.db".to_string()
}