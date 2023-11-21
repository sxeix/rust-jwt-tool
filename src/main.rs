#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

const STRING_OPTIONS: [&str; 2] = [
    "project1", "project2"
];

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "jwt-generator",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    string_option: String,
    secret: String,
    content: String,
    jwt: String,
    current_content: String
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            string_option: String::from(""),
            secret: String::from(""),
            content: String::from(""),
            jwt: String::from(""),
            current_content: String::from("")
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("JWT generator");

            ui.label("Please select the app you want a JWT for");


            egui::ComboBox::from_label(format!(
                "Currently selected project: {}",
                self.string_option
            ))
            .selected_text(self.string_option.clone())
            .show_ui(ui, |ui| {
                for option in STRING_OPTIONS {
                    // Selectable values can be anything: enums, strings or integers - as long as they can be compared and have a text repersentation
                    ui.selectable_value(&mut self.string_option, option.into(), option);
                }
            });
            
            if self.current_content != self.string_option {
                match self.string_option.as_str() {
                    "project1" => self.content.replace_range(.., "helloworld"),
                    "project2" => self.content.replace_range(.., "helloworl2"),
                    _ => self.content.clear()
                }
                self.current_content.replace_range(.., &self.string_option)
            }
                
            ui.text_edit_multiline(&mut self.content);

            ui.text_edit_singleline(&mut self.secret);

            ui.label(format!("Payload is currently: {}", self.content));

            if ui.button("Generate").clicked() {
                self.jwt = String::from("my jwt result");
            }

            ui.label(format!("JWT: {}", self.jwt));
        });
    }
}
