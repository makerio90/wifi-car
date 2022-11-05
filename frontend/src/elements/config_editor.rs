use std::sync::{Arc, RwLock};

use crate::Element;
use wasm_bindgen_futures::spawn_local;
use web_sys::Document;

pub struct Edit {
    config: Arc<RwLock<String>>,
}

impl Default for Edit {
    fn default() -> Self {
        Self {
            config: Arc::new(RwLock::new(String::new())),
        }
    }
}

impl Element for Edit {
    fn name(&self) -> &'static str {
        "config"
    }
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        let Self { config } = self;
        egui::Window::new("config").open(open).show(ctx, |ui| {
            let mut config_string = config.read().unwrap().to_string();
            let response = ui.add(
                egui::TextEdit::multiline(&mut config_string)
                    .font(egui::TextStyle::Monospace) // for cursor height
                    .code_editor()
                    .desired_rows(10)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY),
            );
            if response.changed() {
                *config.write().unwrap() = config_string
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.horizontal(|ui| {
                    if ui.button("save").clicked() {
                        let config = config.clone();
                        spawn_local(async move {
                            let client = reqwest::Client::new();
                            let document = Document::new().unwrap();
                            let loc = document
                                .location()
                                .map(|l| l.origin().unwrap())
                                .unwrap_or(String::from("http://127.0.0.1:8080"));
                            client
                                .post(format!("{}/api/config", loc))
                                .body(config.read().unwrap().to_string())
                                .send()
                                .await
                                .unwrap();
                        })
                    };
                    if ui.button("load").clicked() {
                        let config = config.clone();
                        spawn_local(async move {
                            let document = Document::new().unwrap();
                            let loc = document
                                .location()
                                .map(|l| l.origin().unwrap())
                                .unwrap_or(String::from("http://127.0.0.1:8080"));
                            *config.write().unwrap() = reqwest::get(format!("{}/api/config", loc))
                                .await
                                .unwrap()
                                .text()
                                .await
                                .unwrap();
                        })
                    }
                });
            });
        });
    }
}
