use eframe::egui::{CentralPanel, Context};

#[derive(Default)]
struct CalculatorApp {
    display: String,
    current_value: f64,
    stored_value: Option<f64>,
    current_operator: Option<String>,
    reset_display: bool,
}

impl CalculatorApp {
    fn process_input(&mut self, input: &str) {
        match input {
            "C" => {
                self.display.clear();
                self.current_value = 0.0;
                self.stored_value = None;
                self.current_operator = None;
                self.reset_display = false;
            }
            "+" | "-" | "*" | "/" => {
                if self.display.is_empty() {
                    return;
                }

                if let Some(last_char) = self.display.chars().last() {
                    if "+-*/".contains(last_char) {
                        return;
                    }
                }

                if let Some(_) = &self.current_operator {
                    self.calculate();
                }

                self.stored_value = Some(self.current_value);
                self.current_operator = Some(input.to_string());
                self.display.push_str(input);
                self.reset_display = true;
            }
            "=" => {
                self.calculate();
                self.current_operator = None;
                self.reset_display = true;
            }
            _ => {
                if self.reset_display {
                    self.display.clear();
                    self.reset_display = false;
                }

                if input == "." && self.display.contains('.') {
                    return;
                }

                self.display.push_str(input);
                self.current_value = self.display.parse().unwrap_or(0.0);
            }
        }
    }

    fn calculate(&mut self) {
        if let Some(stored) = self.stored_value {
            if let Some(op) = &self.current_operator {
                self.current_value = match op.as_str() {
                    "+" => stored + self.current_value,
                    "-" => stored - self.current_value,
                    "*" => stored * self.current_value,
                    "/" => {
                        if self.current_value != 0.0 {
                            stored / self.current_value
                        } else {
                            self.display = "Помилка: ділення на нуль".to_string();
                            return;
                        }
                    }
                    _ => self.current_value,
                };
                self.display = self.current_value.to_string();
            }
        }
        self.stored_value = None;
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label(&self.display);

            let buttons: [&[&str]; 5] = [
                &["7", "8", "9", "/"],
                &["4", "5", "6", "*"],
                &["1", "2", "3", "-"],
                &["0", ".", "=", "+"],
                &["C"],
            ];

            for row in buttons.iter() {
                ui.horizontal(|ui| {
                    for &button in *row {
                        if ui.button(button).clicked() {
                            self.process_input(button);
                        }
                    }
                });
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Калькулятор",
        options,
        Box::new(|_cc| Box::<CalculatorApp>::default()),
    )
}
