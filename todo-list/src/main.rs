use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Task {
    title: String,
    completed: bool,
}

impl Task {
    fn new(title: String) -> Self {
        Self {
            title,
            completed: false,
        }
    }
}

struct TodoApp {
    tasks: Vec<Task>,
    new_task_title: String,
    editing_task_index: Option<usize>,
    save_file: String,
}

impl TodoApp {
    fn new(save_file: &str) -> Self {
        let tasks = if let Ok(mut file) = File::open(save_file) {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Vec::new()
        };

        Self {
            tasks,
            new_task_title: String::new(),
            editing_task_index: None,
            save_file: save_file.to_string(),
        }
    }

    fn save_tasks(&self) {
        if let Ok(mut file) = File::create(&self.save_file) {
            let data = serde_json::to_string(&self.tasks).unwrap();
            file.write_all(data.as_bytes()).unwrap();
        }
    }

    fn render_task(&mut self, ui: &mut egui::Ui, index: usize, editing: bool, tasks_to_remove: &mut Vec<usize>) {
        let task = &self.tasks[index].clone();
        let mut completed = task.completed;

        ui.horizontal(|ui| {
            if !editing {
                if ui.checkbox(&mut completed, "").clicked() {
                    self.tasks[index].completed = completed;
                    self.save_tasks();
                }
            }

            let title_color = if task.completed {
                egui::Color32::GRAY
            } else {
                egui::Color32::BLACK
            };

            ui.label(egui::RichText::new(&task.title).color(title_color));

            if !task.completed && !editing {
                if ui.button("Редагувати").clicked() {
                    self.editing_task_index = Some(index);
                    self.new_task_title = task.title.clone();
                }

                if ui.button("Видалити").clicked() {
                    tasks_to_remove.push(index);
                }
            }
        });
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Список справ");

            ui.horizontal(|ui| {
                ui.label("Назва завдання:");
                ui.text_edit_singleline(&mut self.new_task_title);
            });

            if let Some(editing_index) = self.editing_task_index {
                if ui.button("Зберегти зміни").clicked() {
                    if let Some(task) = self.tasks.get_mut(editing_index) {
                        task.title = self.new_task_title.clone();
                        self.editing_task_index = None;
                        self.new_task_title.clear();
                        self.save_tasks();
                    }
                }

                if ui.button("Скасувати редагування").clicked() {
                    self.editing_task_index = None;
                    self.new_task_title.clear();
                }
            } else {
                if ui.button("Додати завдання").clicked() {
                    if !self.new_task_title.is_empty() {
                        self.tasks.push(Task::new(self.new_task_title.clone()));
                        self.new_task_title.clear();
                        self.save_tasks();
                    }
                }
            }

            ui.separator();

            let editing = self.editing_task_index.is_some();

            let mut tasks_to_remove: Vec<usize> = Vec::new();

            for index in 0..self.tasks.len() {
                self.render_task(ui, index, editing, &mut tasks_to_remove);
            }

            for index in tasks_to_remove.iter().rev() {
                self.tasks.remove(*index);
            }

            self.save_tasks();
        });
    }
}

fn main() {
    let save_file = "tasks.json";
    let app = TodoApp::new(save_file);
    let options = eframe::NativeOptions::default();
    eframe::run_native("Список справ", options, Box::new(|_| Box::new(app))).unwrap();
}
