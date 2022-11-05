use crate::Element;

pub struct Webcam;

impl Element for Webcam {
    fn name(&self) -> &'static str {
        "webcam"
    }
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .show(ctx, |ui| ui.label("tbd"));
    }
}
