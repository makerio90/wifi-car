use futures_util::StreamExt;
use wasm_bindgen_futures::spawn_local;
use web_sys::Document;

use crate::Element;

pub struct Webcam {
    availible_resolutions: Vec<String>,
    current: usize,
}
impl Default for Webcam {
    fn default() -> Self {
        Self {
            availible_resolutions: vec![
                "test".to_string(),
                "another".to_string(),
                "one".to_string(),
                "More".to_string(),
            ],
            current: 0,
        }
    }
}

impl Element for Webcam {
    fn name(&self) -> &'static str {
        "webcam"
    }
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        let Self {
            availible_resolutions,
            mut current,
        } = self;
        egui::Window::new("webcam").open(open).show(ctx, |ui| {
            let document = Document::new().unwrap();
            let loc = document
                .location()
                .map(|l| l.origin().unwrap())
                .unwrap_or(String::from("http://127.0.0.1:8080"));
            ui.hyperlink_to("open webcam Stream", format!("{}/webcam/stream", loc));
            ui.label("dont forget to stop the stream before changing this!");

            egui::ComboBox::from_label("Select one!").show_index(
                ui,
                &mut current,
                availible_resolutions.len(),
                |i| availible_resolutions[i].to_owned(),
            );
        });
    }
    fn get(&mut self) {}
}
