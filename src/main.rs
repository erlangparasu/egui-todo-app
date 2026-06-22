use eframe::egui;

#[derive(Clone)]
struct TodoItem {
    id: usize,
    title: String,
    description: String,
    completed: bool,
}

struct TodoApp {
    tasks: Vec<TodoItem>,
    next_id: usize,
    input_title: String,
    input_description: String,
    selected_id: Option<usize>,
    edit_mode: bool,
}

impl Default for TodoApp {
    fn default() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
            input_title: String::new(),
            input_description: String::new(),
            selected_id: None,
            edit_mode: false,
        }
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ToDo List");
            ui.separator();

            if self.selected_id.is_some() && self.edit_mode {
                self.show_edit_view(ui);
            } else if let Some(id) = self.selected_id {
                self.show_detail_view(ui, id);
            } else {
                self.show_list_view(ui);
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
                self.tasks.push(TodoItem {
                    id: self.next_id,
                    title: self.input_title.clone(),
                    description: self.input_description.clone(),
                    completed: false,
                });
                self.next_id += 1;
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
                    }
                    ui.label(&task.title);
                    if task.completed {
                        ui.label("✓");
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
            ui.separator();

            ui.horizontal(|ui| {
                if ui.add_sized([80.0, 35.0], egui::Button::new("Edit")).clicked() {
                    self.edit_mode = true;
                }
                if ui.add_sized([100.0, 35.0], egui::Button::new("Toggle Done")).clicked() {
                    if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
                        t.completed = !t.completed;
                    }
                }
                if ui.add_sized([100.0, 35.0], egui::Button::new("Delete")).clicked() {
                    self.tasks.retain(|t| t.id != id);
                    self.selected_id = None;
                }
                if ui.add_sized([80.0, 35.0], egui::Button::new("Back")).clicked() {
                    self.selected_id = None;
                }
            });
        } else {
            ui.label("Task not found");
            if ui.button("Back").clicked() {
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
                        self.edit_mode = false;
                    }
                    if ui.add_sized([80.0, 35.0], egui::Button::new("Cancel")).clicked() {
                        self.edit_mode = false;
                    }
                });
            }
        }
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native("ToDo App", options, Box::new(|_cc| Ok(Box::new(TodoApp::default()))));
}