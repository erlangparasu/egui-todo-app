mod database;

use eframe::egui;
use rusqlite::Connection;
use std::time::{SystemTime, UNIX_EPOCH};

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn format_date(timestamp: u64) -> String {
    let secs = timestamp as i64;
    chrono::DateTime::from_timestamp(secs, 0)
        .unwrap()
        .format("%Y-%m-%d %H:%M")
        .to_string()
}

#[derive(Clone)]
struct TodoItem {
    id: usize,
    title: String,
    description: String,
    completed: bool,
    readonly: bool,
    creation_date: u64,
    changed_date: u64,
    deletion_date: Option<u64>,
}

enum ViewMode {
    List,
    Detail,
    Edit,
    Trash,
}

struct TodoApp {
    conn: Connection,
    tasks: Vec<TodoItem>,
    trashed_tasks: Vec<TodoItem>,
    input_title: String,
    input_description: String,
    selected_id: Option<usize>,
    view_mode: ViewMode,
}

impl TodoApp {
    fn new() -> Self {
        let conn = database::init_database("todo.db").expect("Failed to initialize database");
        let tasks = database::load_active_todos(&conn).expect("Failed to load todos");
        let trashed_tasks = database::load_trashed_todos(&conn).expect("Failed to load trashed todos");

        Self {
            conn,
            tasks,
            trashed_tasks,
            input_title: String::new(),
            input_description: String::new(),
            selected_id: None,
            view_mode: ViewMode::List,
        }
    }
}

impl Default for TodoApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ToDo List");

            ui.horizontal(|ui| {
                match self.view_mode {
                    ViewMode::List | ViewMode::Detail | ViewMode::Edit => {
                        if ui.add_sized([80.0, 30.0], egui::Button::new("Tasks")).clicked() {
                            self.view_mode = ViewMode::List;
                            self.selected_id = None;
                        }
                    }
                    ViewMode::Trash => {
                        if ui.add_sized([80.0, 30.0], egui::Button::new("Back")).clicked() {
                            self.view_mode = ViewMode::List;
                        }
                    }
                }
                let trashed_count = self.trashed_tasks.len();
                if ui.add_sized([80.0, 30.0], egui::Button::new(format!("Trash ({})", trashed_count))).clicked() {
                    self.view_mode = ViewMode::Trash;
                    self.selected_id = None;
                }
                if ui.add_sized([80.0, 30.0], egui::Button::new("Export")).clicked() {
                    if let Err(e) = database::export_database("todo.db", "todo_export.db") {
                        eprintln!("Export failed: {}", e);
                    }
                }
            });

            ui.separator();

            match &self.view_mode {
                ViewMode::List => self.show_list_view(ui),
                ViewMode::Detail => {
                    if let Some(id) = self.selected_id {
                        self.show_detail_view(ui, id);
                    }
                }
                ViewMode::Edit => self.show_edit_view(ui),
                ViewMode::Trash => self.show_trash_view(ui),
            }
        });
    }
}

impl TodoApp {
    fn show_list_view(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Title:");
            ui.text_edit_singleline(&mut self.input_title);
        });
        ui.horizontal(|ui| {
            ui.label("Description:");
            ui.text_edit_singleline(&mut self.input_description);
        });
        ui.add_space(10.0);
        if ui.add_sized([100.0, 35.0], egui::Button::new("Create")).clicked() {
            if !self.input_title.is_empty() {
                let now = current_timestamp();
                if let Ok(id) = database::insert_todo(&self.conn, &self.input_title, &self.input_description, now, now) {
                    self.tasks.insert(0, TodoItem {
                        id,
                        title: self.input_title.clone(),
                        description: self.input_description.clone(),
                        completed: false,
                        readonly: false,
                        creation_date: now,
                        changed_date: now,
                        deletion_date: None,
                    });
                }
                self.input_title.clear();
                self.input_description.clear();
            }
        }
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for task in &self.tasks {
                ui.horizontal(|ui| {
                    if ui.button(format!("View #{}", task.id)).clicked() {
                        self.selected_id = Some(task.id);
                        self.view_mode = ViewMode::Detail;
                    }
                    ui.label(&task.title);
                    if task.completed {
                        ui.label("✓");
                    }
                    if task.readonly {
                        ui.label("🔒");
                    }
                });
            }
        });
    }

    fn show_detail_view(&mut self, ui: &mut egui::Ui, id: usize) {
        if let Some(task) = self.tasks.iter().find(|t| t.id == id) {
            ui.label(format!("ID: {}", task.id));
            ui.label(format!("Title: {}", task.title));
            ui.label(format!("Description: {}", task.description));
            ui.label(format!("Status: {}", if task.completed { "Completed" } else { "Pending" }));
            ui.label(format!("Created: {}", format_date(task.creation_date)));
            ui.label(format!("Changed: {}", format_date(task.changed_date)));
            ui.separator();

            ui.horizontal(|ui| {
                if ui.add_sized([80.0, 35.0], egui::Button::new("Edit")).clicked() {
                    self.view_mode = ViewMode::Edit;
                }
                if ui.add_sized([100.0, 35.0], egui::Button::new("Toggle Done")).clicked() {
                    if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
                        let new_completed = !t.completed;
                        let now = current_timestamp();
                        if database::toggle_todo(&self.conn, id, new_completed, now).is_ok() {
                            t.completed = new_completed;
                            t.changed_date = now;
                        }
                    }
                }
                if ui.add_sized([100.0, 35.0], egui::Button::new("Toggle RO")).clicked() {
                    if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
                        let new_readonly = !t.readonly;
                        let now = current_timestamp();
                        if database::set_readonly(&self.conn, id, new_readonly, now).is_ok() {
                            t.readonly = new_readonly;
                            t.changed_date = now;
                        }
                    }
                }
                if ui.add_sized([100.0, 35.0], egui::Button::new("Delete")).clicked() {
                    let now = current_timestamp();
                    if database::soft_delete_todo(&self.conn, id, now).is_ok() {
                        if let Some(idx) = self.tasks.iter().position(|t| t.id == id) {
                            let mut trashed = self.tasks.remove(idx);
                            trashed.deletion_date = Some(now);
                            self.trashed_tasks.insert(0, trashed);
                        }
                        self.view_mode = ViewMode::List;
                    }
                }
                if ui.add_sized([80.0, 35.0], egui::Button::new("Back")).clicked() {
                    self.view_mode = ViewMode::List;
                    self.selected_id = None;
                }
            });
        } else {
            ui.label("Task not found");
            if ui.button("Back").clicked() {
                self.view_mode = ViewMode::List;
                self.selected_id = None;
            }
        }
    }

    fn show_edit_view(&mut self, ui: &mut egui::Ui) {
        if let Some(id) = self.selected_id {
            if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
                ui.label("Edit Task");
                ui.horizontal(|ui| {
                    ui.label("Title:");
                    ui.text_edit_singleline(&mut task.title);
                });
                ui.horizontal(|ui| {
                    ui.label("Description:");
                    ui.text_edit_singleline(&mut task.description);
                });
                ui.separator();

                ui.horizontal(|ui| {
                    if ui.add_sized([80.0, 35.0], egui::Button::new("Save")).clicked() {
                        let now = current_timestamp();
                        if database::update_todo(&self.conn, id, &task.title, &task.description, now).is_ok() {
                            task.changed_date = now;
                            self.view_mode = ViewMode::Detail;
                        }
                    }
                    if ui.add_sized([80.0, 35.0], egui::Button::new("Cancel")).clicked() {
                        self.view_mode = ViewMode::Detail;
                    }
                });
            }
        }
    }

    fn show_trash_view(&mut self, ui: &mut egui::Ui) {
        ui.label("Trash");
        ui.separator();

        if self.trashed_tasks.is_empty() {
            ui.label("Trash is empty");
            return;
        }

        let mut to_restore: Vec<usize> = Vec::new();
        let mut to_delete: Vec<usize> = Vec::new();

        for task in &self.trashed_tasks {
            ui.horizontal(|ui| {
                ui.label(format!("#{} - {}", task.id, task.title));
                if let Some(date) = task.deletion_date {
                    ui.label(format!("(deleted {})", format_date(date)));
                }
            });
            ui.horizontal(|ui| {
                if ui.add_sized([80.0, 30.0], egui::Button::new("Restore")).clicked() {
                    to_restore.push(task.id);
                }
                if ui.add_sized([120.0, 30.0], egui::Button::new("Permanent Delete")).clicked() {
                    to_delete.push(task.id);
                }
            });
            ui.separator();
        }

        for id in to_restore {
            let now = current_timestamp();
            if database::restore_todo(&self.conn, id, now).is_ok() {
                if let Some(idx) = self.trashed_tasks.iter().position(|t| t.id == id) {
                    let mut restored = self.trashed_tasks.remove(idx);
                    restored.deletion_date = None;
                    restored.changed_date = now;
                    self.tasks.insert(0, restored);
                }
            }
        }

        for id in to_delete {
            if database::permanent_delete_todo(&self.conn, id).is_ok() {
                self.trashed_tasks.retain(|t| t.id != id);
            }
        }

        if ui.add_sized([150.0, 35.0], egui::Button::new("Empty Trash")).clicked() {
            if database::empty_trash(&self.conn).is_ok() {
                self.trashed_tasks.clear();
            }
        }
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native("ToDo App", options, Box::new(|_cc| Ok(Box::new(TodoApp::default()))));
}