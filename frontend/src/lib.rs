#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod elements;

pub use app::TemplateApp;

/// Something to view
pub trait Element {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);

    /// get data (onclick)
    fn get(&mut self);
}
