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

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo_tags (
            todo_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (todo_id, tag_id),
            FOREIGN KEY (todo_id) REFERENCES todos(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
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

    let todos: Vec<TodoItem> = stmt.query_map([], |row| {
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
            tags: Vec::new(),
        })
    })?
    .collect::<Result<Vec<_>>>()?;

    let mut result = Vec::new();
    for mut todo in todos {
        todo.tags = get_tags_for_todo(conn, todo.id)?;
        result.push(todo);
    }

    Ok(result)
}

pub fn load_trashed_todos(conn: &Connection) -> Result<Vec<TodoItem>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, description, completed, readonly, priority, order_index, creation_date, changed_date, deletion_date
         FROM todos WHERE deletion_date IS NOT NULL ORDER BY deletion_date DESC"
    )?;

    let todos: Vec<TodoItem> = stmt.query_map([], |row| {
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
            tags: Vec::new(),
        })
    })?
    .collect::<Result<Vec<_>>>()?;

    let mut result = Vec::new();
    for mut todo in todos {
        todo.tags = get_tags_for_todo(conn, todo.id)?;
        result.push(todo);
    }

    Ok(result)
}

fn get_tags_for_todo(conn: &Connection, todo_id: usize) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT t.name FROM tags t
         INNER JOIN todo_tags tt ON t.id = tt.tag_id
         WHERE tt.todo_id = ?1"
    )?;

    let tags = stmt.query_map([todo_id], |row| row.get(0))?
        .collect::<Result<Vec<String>>>()?;

    Ok(tags)
}

pub fn insert_todo(conn: &Connection, title: &str, description: &str, priority: u8, order_index: i32, creation_date: u64, changed_date: u64) -> Result<usize> {
    conn.execute(
        "INSERT INTO todos (title, description, completed, readonly, priority, order_index, creation_date, changed_date) VALUES (?1, ?2, 0, 0, ?3, ?4, ?5, ?6)",
        rusqlite::params![title, description, priority, order_index, creation_date, changed_date],
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

pub fn update_order_index(conn: &Connection, id: usize, order_index: i32, changed_date: u64) -> Result<()> {
    conn.execute(
        "UPDATE todos SET order_index = ?1, changed_date = ?2 WHERE id = ?3",
        rusqlite::params![order_index, changed_date, id],
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

pub fn update_priority(conn: &Connection, id: usize, priority: u8, changed_date: u64) -> Result<()> {
    conn.execute(
        "UPDATE todos SET priority = ?1, changed_date = ?2 WHERE id = ?3",
        rusqlite::params![priority, changed_date, id],
    )?;
    Ok(())
}

pub fn get_all_tags(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT name FROM tags ORDER BY name")?;
    let tags = stmt.query_map([], |row| row.get(0))?
        .collect::<Result<Vec<String>>>()?;
    Ok(tags)
}

pub fn get_used_tags(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT DISTINCT t.name FROM tags t
         INNER JOIN todo_tags tt ON t.id = tt.tag_id
         INNER JOIN todos d ON tt.todo_id = d.id
         WHERE d.deletion_date IS NULL
         ORDER BY t.name"
    )?;
    let tags = stmt.query_map([], |row| row.get(0))?
        .collect::<Result<Vec<String>>>()?;
    Ok(tags)
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

pub fn add_tag(conn: &Connection, todo_id: usize, tag_name: &str) -> Result<()> {
    let tag_name = tag_name.trim().to_lowercase();
    if tag_name.is_empty() {
        return Ok(());
    }

    conn.execute("INSERT OR IGNORE INTO tags (name) VALUES (?1)", [&tag_name])?;

    let tag_id: i64 = {
        let mut stmt = conn.prepare("SELECT id FROM tags WHERE name = ?1")?;
        stmt.query_row([&tag_name], |row| row.get(0))?
    };

    conn.execute(
        "INSERT OR IGNORE INTO todo_tags (todo_id, tag_id) VALUES (?1, ?2)",
        rusqlite::params![todo_id, tag_id],
    )?;

    Ok(())
}

pub fn remove_tag(conn: &Connection, todo_id: usize, tag_name: &str) -> Result<()> {
    let tag_name = tag_name.trim().to_lowercase();

    let tag_id: Option<i64> = conn.query_row(
        "SELECT id FROM tags WHERE name = ?1",
        [&tag_name],
        |row| row.get(0),
    ).ok();

    if let Some(tag_id) = tag_id {
        conn.execute(
            "DELETE FROM todo_tags WHERE todo_id = ?1 AND tag_id = ?2",
            rusqlite::params![todo_id, tag_id],
        )?;
    }

    Ok(())
}

pub fn update_todo_tags(conn: &Connection, todo_id: usize, tags: &[String]) -> Result<()> {
    conn.execute("DELETE FROM todo_tags WHERE todo_id = ?1", [todo_id])?;

    for tag_name in tags {
        add_tag(conn, todo_id, tag_name)?;
    }

    Ok(())
}