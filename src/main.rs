#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{isize, result};

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
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

// TODO: In the feuature the best way to do this would to have a string as the input and parse the
// string

struct MyApp {
    input: String,
    display: String,
    nums: Vec<isize>,
    operators: String,
    last_operator_pos: usize,
    operator_pos: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input: "".to_owned(),
            display: "0".to_owned(),
            nums: vec![],
            operators: "".to_owned(),
            last_operator_pos: 0,
            operator_pos: 0,
        }
    }
}

impl MyApp {
    // fn clear(&mut self) {
    //
    //
    //
    // }
    //
    fn append_num(&mut self) {
        // if self.input.len() !=
        if let Ok(num) = self.input[self.last_operator_pos..self.operator_pos - 1].parse::<isize>()
        {
            println!("{num}");
            self.nums.push(num);
        }
    }

    fn calulate(&mut self) -> isize {
        // let mut nums: Vec<isize> = self.nums;
        // turn into a complex number
        // let mut result_float: f64 = 0.0;
        let mut result: isize;
        for (index, operator) in self.operators.clone().chars().enumerate() {
            let pos_1: usize;
            let pos_2: usize;
            match operator {
                '*' => {
                    println!("index {index}");
                    if index == self.nums.len()-1 {
                        pos_1 = index - 1;
                        pos_2 = index; 
                    } else if index == 0 {
                        pos_1 = index;
                        pos_2 = index + 1;
                    } else {
                        pos_1 = index - 1;
                        pos_2 = index + 1;
                    }
                        result = self.nums[pos_1] * self.nums[pos_2];
                        self.nums.remove(pos_1);
                        self.nums[pos_1] = result;
                        if index == self.operators.len() {
                            self.operators.pop();
                        } else {
                            self.operators.remove(index);
                        }
                }
                // '/' => {
                //     let num_1: isize = self.nums[0];
                //     let num_2: isize = self.nums[1];
                //     result_float = num_1 as f64 / num_2 as f64;
                //
                // }
                _ => {
                    todo!()
                }
            }
        }
        // if result != 0 {
        //     ValueCalculated::Number(result)
        // } else {
        //     ValueCalculated::Float(result_float)
        // }
        self.nums[0]
    }
}

// fn find_operator(input: &str, starting_pos: usize) {
//     for i in ['*', '/'] {
//         input[starting_pos..].find(i)
//     }
// }
// #[derive(Debug)]
// enum ValueCalculated {
//     Number(isize),
//     Float(f64),
// }
//
// Checks if the last input was an operator
fn check_if_operator(input: &str) -> bool {
    matches!(
        input.chars().last(),
        Some('+') | Some('-') | Some('*') | Some('/')
    )
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Calulator");

            ui.horizontal(|ui| {
                if ui.button("AC").clicked() {
                    self.input = "".to_owned();
                    self.display = "0".to_owned();
                    self.operators = "".to_owned();
                    self.nums = vec![];
                }
            });

            ui.horizontal(|ui| {
                if ui.button("7").clicked() {
                    self.input.push('7');
                } else if ui.button("8").clicked() {
                    self.input.push('8')
                } else if ui.button("9").clicked() {
                    self.input.push('9');
                } else if ui.button("*").clicked() && !check_if_operator(&self.input) {
                    self.input.push('*');
                    // self.num_of_opeators += 1;
                    self.operators.push('*');
                    self.operator_pos = self.input.len();
                    self.append_num();
                    self.last_operator_pos = self.input.len();
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
                    self.operators.push('-');
                    self.operator_pos = self.input.len();
                    self.append_num();
                    self.last_operator_pos = self.input.len();
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
                    self.operators.push('+');
                    self.operator_pos = self.input.len();
                    self.append_num();
                    self.last_operator_pos = self.input.len();
                }
            });

            ui.horizontal(|ui| {
                if ui.button("0").clicked() {
                    self.input.push('0');
                } else if ui.button(".").clicked() {
                    self.input.push('.');
                } else if ui.button("=").clicked() {
                    self.operator_pos = self.input.len() + 1;
                    self.append_num();
                    self.display = format!("{}", self.calulate());
                    println!("{:#?}\n{:#?}", self.nums, self.operators);
                    self.input = "".to_owned();
                    self.operators = "".to_owned();
                    self.nums = vec![];
                    self.operator_pos = 0;
                } else if ui.button("/").clicked() && !check_if_operator(&self.input) {
                    self.input.push('/');
                    self.operators.push('/');
                    self.operator_pos = self.input.len();
                    self.append_num();
                    self.last_operator_pos = self.input.len();
                }
            });
            if !self.input.is_empty() {
                self.display = self.input.clone();
            }
            ui.heading(&self.display);
            // ui.label(format!(
            //     "Current input {:#?}, Current Display {:#?}",
            //     self.input, self.display
            // ));
            println!("{:#?}\n{:#?}", self.nums, self.operators);
        });
    }
}
