use crate::Element;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

pub struct Drive;
impl Element for Drive {
    fn name(&self) -> &'static str {
        "drive"
    }
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name()).open(open).show(ctx, |ui| {
            if ui.button("enable").clicked() {
                spawn_local(async {
                    Request::post("/api/enable").send().await.unwrap();
                })
            }
            if ui.button("disable").clicked() {
                spawn_local(async {
                    Request::post("/api/disable").send().await.unwrap();
                })
            }
        });
    }
}
