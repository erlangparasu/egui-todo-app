mod database;

use eframe::egui;
use rusqlite::Connection;
use std::collections::HashMap;
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
    priority: u8,
    order_index: i32,
    creation_date: u64,
    changed_date: u64,
    deletion_date: Option<u64>,
    tags: Vec<String>,
}

impl TodoItem {
    fn priority_label(&self) -> String {
        format!("P{}", self.priority)
    }
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
    input_tag: String,
    input_priority: u8,
    search_text: String,
    selected_tags: Vec<String>,
    all_tags: Vec<String>,
    selected_id: Option<usize>,
    view_mode: ViewMode,
    drag_source_id: Option<usize>,
}

impl TodoApp {
    fn new() -> Self {
        let db_path = database::get_db_path();
        let conn = database::init_database(&db_path).expect("Failed to initialize database");
        let tasks = database::load_active_todos(&conn).expect("Failed to load todos");
        let trashed_tasks = database::load_trashed_todos(&conn).expect("Failed to load trashed todos");
        let all_tags = database::get_used_tags(&conn).unwrap_or_default();

        Self {
            conn,
            tasks,
            trashed_tasks,
            input_title: String::new(),
            input_description: String::new(),
            input_tag: String::new(),
            input_priority: 3,
            search_text: String::new(),
            selected_tags: Vec::new(),
            all_tags,
            selected_id: None,
            view_mode: ViewMode::List,
            drag_source_id: None,
        }
    }

    fn refresh_tags(&mut self) {
        self.all_tags = database::get_used_tags(&self.conn).unwrap_or_default();
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
                    if let Some(path) = rfd::FileDialog::new()
                        .set_file_name("todo_export.db")
                        .add_filter("SQLite Database", &["db"])
                        .save_file()
                    {
                        if let Err(e) = database::export_database("todo.db", path.to_str().unwrap_or("todo_export.db")) {
                            eprintln!("Export failed: {}", e);
                        }
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
        ui.horizontal(|ui| {
            ui.label("Priority:");
            egui::ComboBox::from_id_salt("priority_select")
                .selected_text(format!("P{}", self.input_priority))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.input_priority, 1, "P1");
                    ui.selectable_value(&mut self.input_priority, 2, "P2");
                    ui.selectable_value(&mut self.input_priority, 3, "P3");
                    ui.selectable_value(&mut self.input_priority, 4, "P4");
                    ui.selectable_value(&mut self.input_priority, 5, "P5");
                });
        });
        ui.add_space(10.0);
        if ui.add_sized([100.0, 35.0], egui::Button::new("Create")).clicked() {
            if !self.input_title.is_empty() {
                let now = current_timestamp();
                let max_order = self.tasks.iter().map(|t| t.order_index).max().unwrap_or(-1);
                if let Ok(id) = database::insert_todo(&self.conn, &self.input_title, &self.input_description, self.input_priority, max_order + 1, now, now) {
                    self.tasks.insert(0, TodoItem {
                        id,
                        title: self.input_title.clone(),
                        description: self.input_description.clone(),
                        completed: false,
                        readonly: false,
                        priority: self.input_priority,
                        order_index: max_order + 1,
                        creation_date: now,
                        changed_date: now,
                        deletion_date: None,
                        tags: Vec::new(),
                    });
                }
                self.input_title.clear();
                self.input_description.clear();
            }
        }
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Search:");
            ui.text_edit_singleline(&mut self.search_text);
        });

        if !self.all_tags.is_empty() {
            ui.label("Filter by tags:");
            ui.push_id("tag_filter_scroll", |ui| {
                egui::ScrollArea::horizontal().show(ui, |ui| {
                    for tag in &self.all_tags {
                        let mut selected = self.selected_tags.contains(tag);
                        if ui.checkbox(&mut selected, tag).changed() {
                            if selected {
                                self.selected_tags.push(tag.clone());
                            } else {
                                self.selected_tags.retain(|t| t != tag);
                            }
                        }
                    }
                });
            });
        }

        let search_lower = self.search_text.to_lowercase();
        let selected_tags_clone = self.selected_tags.clone();
        let filtered_tasks: Vec<&TodoItem> = self.tasks.iter()
            .filter(|t| {
                let matches_search = search_lower.is_empty()
                    || t.title.to_lowercase().contains(&search_lower)
                    || t.description.to_lowercase().contains(&search_lower);
                let matches_tags = selected_tags_clone.is_empty()
                    || selected_tags_clone.iter().any(|st| t.tags.contains(st));
                matches_search && matches_tags
            })
            .collect();

        ui.label(format!("Showing {}/{} tasks", filtered_tasks.len(), self.tasks.len()));

        let task_ids: Vec<usize> = filtered_tasks.iter().map(|t| t.id).collect();
        let task_id_to_info: std::collections::HashMap<usize, (String, bool, bool)> = filtered_tasks.iter()
            .map(|t| (t.id, (t.priority_label(), t.completed, t.readonly)))
            .collect();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for task_id in task_ids {
                let (priority_label, completed, readonly) = task_id_to_info[&task_id].clone();
                let task_title = self.tasks.iter().find(|t| t.id == task_id).map(|t| t.title.clone()).unwrap_or_default();
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        if ui.add_sized([20.0, 18.0], egui::Button::new("^").small()).clicked() {
                            self.move_task(task_id, -1);
                        }
                        if ui.add_sized([20.0, 18.0], egui::Button::new("v").small()).clicked() {
                            self.move_task(task_id, 1);
                        }
                    });
                    if ui.button(format!("#{} {}", priority_label, task_title)).clicked() {
                        self.selected_id = Some(task_id);
                        self.view_mode = ViewMode::Detail;
                    }
                    if completed {
                        ui.label("✓");
                    }
                    if readonly {
                        ui.label("🔒");
                    }
                });
            }
        });
    }

    fn show_detail_view(&mut self, ui: &mut egui::Ui, id: usize) {
        let task_opt = self.tasks.iter().find(|t| t.id == id).cloned();

        if let Some(task) = task_opt {
            let is_readonly = task.readonly;

            ui.label(format!("ID: {}", task.id));
            ui.label(format!("Title: {}", task.title));
            ui.label(format!("Description: {}", task.description));
            ui.label(format!("Status: {}", if task.completed { "Completed" } else { "Pending" }));
            ui.horizontal(|ui| {
                ui.label("Priority:");
                if is_readonly {
                    ui.label(format!("P{}", task.priority));
                } else {
                    let mut priority = task.priority;
                    egui::ComboBox::from_id_salt(format!("priority_detail_{}", id))
                        .selected_text(format!("P{}", priority))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut priority, 1, "P1");
                            ui.selectable_value(&mut priority, 2, "P2");
                            ui.selectable_value(&mut priority, 3, "P3");
                            ui.selectable_value(&mut priority, 4, "P4");
                            ui.selectable_value(&mut priority, 5, "P5");
                        });
                    if priority != task.priority {
                        if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
                            let now = current_timestamp();
                            if database::update_priority(&self.conn, id, priority, now).is_ok() {
                                t.priority = priority;
                                t.changed_date = now;
                            }
                        }
                    }
                }
            });
            ui.label(format!("Readonly: {}", if task.readonly { "Yes 🔒" } else { "No" }));
            ui.label(format!("Created: {}", format_date(task.creation_date)));
            ui.label(format!("Changed: {}", format_date(task.changed_date)));
            ui.label("Tags:");
            let tags_clone = task.tags.clone();
            for tag in &tags_clone {
                ui.horizontal(|ui| {
                    ui.label(format!("[{}]", tag));
                    if !is_readonly {
                        if ui.add_sized([20.0, 20.0], egui::Button::new("x").small()).clicked() {
                            if database::remove_tag(&self.conn, id, tag).is_ok() {
                                if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
                                    t.tags.retain(|x| x != tag);
                                }
                                self.refresh_tags();
                            }
                        }
                    }
                });
            }
            if !is_readonly {
                ui.separator();
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.input_tag);
                    if ui.add_sized([60.0, 30.0], egui::Button::new("Add Tag")).clicked() {
                        if !self.input_tag.is_empty() {
                            if database::add_tag(&self.conn, id, &self.input_tag).is_ok() {
                                if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
                                    t.tags.push(self.input_tag.trim().to_lowercase());
                                }
                                self.refresh_tags();
                            }
                            self.input_tag.clear();
                        }
                    }
                });
            }
            ui.separator();

            ui.horizontal(|ui| {
                if ui.add_sized([80.0, 35.0], egui::Button::new("Back")).clicked() {
                    self.view_mode = ViewMode::List;
                    self.selected_id = None;
                }
                let done_button_text = if task.completed { "Mark Pending" } else { "Mark Done" };
                if ui.add_sized([100.0, 35.0], egui::Button::new(done_button_text)).clicked() {
                    if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
                        let new_completed = !t.completed;
                        let now = current_timestamp();
                        if database::toggle_todo(&self.conn, id, new_completed, now).is_ok() {
                            t.completed = new_completed;
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
                let ro_button_text = if task.readonly { "Unlock" } else { "Lock" };
                if ui.add_sized([100.0, 35.0], egui::Button::new(ro_button_text)).clicked() {
                    if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
                        let new_readonly = !t.readonly;
                        let now = current_timestamp();
                        if database::set_readonly(&self.conn, id, new_readonly, now).is_ok() {
                            t.readonly = new_readonly;
                            t.changed_date = now;
                        }
                    }
                }
                if !is_readonly {
                    if ui.add_sized([80.0, 35.0], egui::Button::new("Edit")).clicked() {
                        self.view_mode = ViewMode::Edit;
                    }
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

    fn move_task(&mut self, id: usize, direction: i32) {
        if let Some(current_idx) = self.tasks.iter().position(|t| t.id == id) {
            let new_idx = (current_idx as i32 + direction) as usize;
            if new_idx < self.tasks.len() {
                let new_order = self.tasks[new_idx].order_index;
                let old_order = self.tasks[current_idx].order_index;
                let other_id = self.tasks[new_idx].id;
                let now = current_timestamp();

                if database::update_order_index(&self.conn, id, new_order, now).is_ok() {
                    let _ = database::update_order_index(&self.conn, other_id, old_order, now);
                }

                if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
                    task.order_index = new_order;
                }
                if let Some(other) = self.tasks.iter_mut().find(|t| t.id == other_id) {
                    other.order_index = old_order;
                }

                self.tasks.sort_by_key(|t| t.order_index);
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