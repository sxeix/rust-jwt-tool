#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use project_info::Project;
use strum::IntoEnumIterator;

mod jwt;
mod project_info;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 500.0)),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "jwt-generator",
        options,
        Box::new(|_cc| Box::new(JwtGenerator::default())),
    );
}

struct JwtGenerator {
    selected_option: Project,
    secret: String,
    content: String,
    jwt: String,
    current_content: Project,
    error_message: String,
}

impl Default for JwtGenerator {
    fn default() -> Self {
        Self {
            selected_option: project_info::Project::Project1,
            secret: String::from(""),
            content: String::from(""),
            jwt: String::from(""),
            current_content: project_info::Project::InvalidProject,
            error_message: String::from(""),
        }
    }
}

impl eframe::App for JwtGenerator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("JWT generator");

            ui.label("Please select the app you want a JWT for");

            egui::ComboBox::from_label(format!(
                "Currently selected project: {}",
                self.selected_option
            ))
            .selected_text(self.selected_option.to_string())
            .show_ui(ui, |ui| {
                for project in Project::iter() {
                    if project == Project::InvalidProject {
                        continue;
                    }
                    ui.selectable_value(&mut self.selected_option, project, project.to_string());
                }
            });

            if self.current_content != self.selected_option {
                match self.selected_option {
                    Project::Project1 => self.content.replace_range(.., "{\n\t\"test\":\"test\"\n}"),
                    Project::Project2 => self.content.replace_range(.., "{\n\t\"another\":\"test\"\n}"),
                    _ => self.content.clear(),
                }
                self.current_content = self.selected_option;
            }

            ui.label("Edit payload content");
            ui.text_edit_multiline(&mut self.content);

            if !self.error_message.is_empty() {
                ui.label(format!("Error: {}", self.error_message));
            }

            ui.label("Enter secret");
            ui.text_edit_singleline(&mut self.secret);

            ui.label(format!("Payload is currently:\n{}", self.content));
            ui.label(format!("Secret is currently: {}", self.secret));

            if ui.button("Generate").clicked() {
                self.jwt = jwt::generate_jwt(&self.selected_option, &self.content, &self.secret);
                if self.jwt.is_empty() {
                    self.error_message.replace_range(
                        ..,
                        "Something went wrong with creating the JWT - check json is valid and key is in base64",
                    );
                } else {
                    self.error_message.clear();
                }
            }

            ui.label(format!("JWT: {}", self.jwt));
            if ui.button("📋").on_hover_text("Click to copy").clicked() {
                ui.output_mut(|po| po.copied_text = self.jwt.clone());
            }
        });
    }
}
