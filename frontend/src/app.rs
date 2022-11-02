use crate::elements;
use crate::Element;
use gloo_net::http::Request;
use sha2::{self, Digest, Sha256};
use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
use wasm_bindgen_futures::spawn_local;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct TemplateApp {
    pass: String,
    elements: Vec<Box<dyn Element>>,
    openElements: BTreeSet<String>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            pass: String::new(),
            elements: vec![Box::new(elements::drive::Drive)],
            openElements: BTreeSet::new(),
        }
    }
}

impl TemplateApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            pass,
            elements,
            openElements,
        } = self;
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.separator();
                ui.label("pass: ");
                ui.add(egui::TextEdit::singleline(&mut *pass).password(true));
                if ui.button("login").clicked() {
                    //let key = self.key.clone();
                    let pass = pass.clone();
                    spawn_local(async move {
                        //let s = key.lock().unwrap();
                        Request::post("/auth/login")
                            .header("Authorization", &format!("{:X}", Sha256::digest(pass)))
                            .send()
                            .await
                            .unwrap();
                    });
                };
            });
        });
        egui::SidePanel::left("left")
            .resizable(false)
            .default_width(150.0)
            .show(ctx, |ui| {
                ui.heading("Elements");
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        for element in elements {
                            let mut is_open = openElements.contains(element.name());
                            element.show(ctx, &mut is_open);
                            ui.toggle_value(&mut is_open, element.name());
                            set_open(&mut *openElements, element.name(), is_open);
                        }
                    })
                });
            });
        egui::CentralPanel::default().show(ctx, |_ui| {
            //    ui.label("Hello World!");
        });
    }
}

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}
