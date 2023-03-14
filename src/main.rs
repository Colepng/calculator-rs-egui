#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

// TODO: In the feuature the best way to do this would to have a string as the input and parse the
// string

struct MyApp<'a> {
    input: String,
    mode_int: bool,
    display: &'a str,
}

impl Default for MyApp<'_> {
    fn default() -> Self {
        Self {
            input: "".to_owned(),
            mode_int: true,
            display: "",
        }
    }
}

// The parser
fn calulate(_input: &str) {}

// Checks if the last input was an operator 
fn check_if_operator(input: &str) -> bool {
    matches!(input.chars().last(), Some('+') | Some('-') | Some('*') | Some('/'))
}

impl eframe::App for MyApp<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Calulator");

            if self.mode_int {
                ui.horizontal(|ui| {
                    if ui.button("7").clicked() {
                        self.input.push('7');
                    } else if ui.button("8").clicked() {
                        self.input.push('8')
                    } else if ui.button("9").clicked() {
                        self.input.push('9');
                    } else if ui.button("*").clicked() && !check_if_operator(&self.input) {
                        self.input.push('*');
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("4").clicked() {
                        self.input.push('4');
                    } else if ui.button("5").clicked() {
                        self.input.push('5');
                    } else if ui.button("6").clicked() {
                        self.input.push('6');
                    } else if ui.button("-").clicked() && !check_if_operator(&self.input) {
                        self.input.push('-');
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("1").clicked() {
                        self.input.push('1');
                    } else if ui.button("2").clicked() {
                        self.input.push('2')
                    } else if ui.button("3").clicked() {
                        self.input.push('3')
                    } else if ui.button("+").clicked() && !check_if_operator(&self.input) {
                        self.input.push('+');
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("0").clicked() {
                        self.input.push('0');
                    } else if ui.button(".").clicked() {
                        self.input.push('.');
                    } else if ui.button("=").clicked() {
                        calulate(&self.input);
                    } else if ui.button("/").clicked() && !check_if_operator(&self.input) {
                            self.input.push('/');
                    }
                });
            }

            self.display = &self.input;
            ui.heading(self.display);
            ui.label(format!(
                "Current input {:#?}, Current Display {:#?}",
                self.input, self.display
            ));
        });
    }
}
