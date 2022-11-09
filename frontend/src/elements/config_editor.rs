use std::sync::{Arc, RwLock};

use crate::Element;
use wasm_bindgen_futures::spawn_local;
use web_sys::Document;

pub struct Edit {
    config: Arc<RwLock<Option<String>>>,
}

impl Default for Edit {
    fn default() -> Self {
        Self {
            config: Arc::new(RwLock::new(None)),
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
            match &config.read().unwrap().clone() {
                Some(ref string) => {
                    let mut config_string: String = string.to_string();
                    let response = ui.add(
                        egui::TextEdit::multiline(&mut config_string)
                            .font(egui::TextStyle::Monospace) // for cursor height
                            .code_editor()
                            .desired_rows(10)
                            .lock_focus(true)
                            .desired_width(f32::INFINITY),
                    );
                    if response.changed() {
                        *config.write().unwrap() = Some(config_string)
                    };
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("save").clicked() {
                                let config = config.clone();
                                spawn_local(async move {
                                    let client = reqwest::Client::new();
                                    let document = Document::new().unwrap();
                                    let window = web_sys::window().unwrap();
                                    let loc = window.location();
                                    let loc = loc.origin().unwrap();
                                    client
                                        .post(format!("{}/api/config", loc))
                                        .body((*config.read().unwrap()).as_ref().unwrap().clone())
                                        .send()
                                        .await
                                        .unwrap();
                                })
                            };
                        });
                    });
                }
                None => {
                    ui.spinner();
                }
            }
        });
    }

    fn get(&mut self) {
        let config = self.config.clone();
        spawn_local(async move {
            let window = web_sys::window().unwrap();
            let loc = window.location();
            let loc = loc.origin().unwrap();
            *config.write().unwrap() = /*Arc::new(*/Some(
                reqwest::get(format!("{}/api/config", loc))
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap(),
            );
        })
    }
}
