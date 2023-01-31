use crate::Element;
use wasm_bindgen_futures::spawn_local;

pub struct Drive;

impl Drive {
	fn drive(&self, accelerate: f64, steer: f64) {
		let parems = [
			("accelerate", accelerate.to_string()),
			("steer", steer.to_string()),
		];
		spawn_local(async move {
			let client = reqwest::Client::new();
			let window = web_sys::window().unwrap();
			let loc = window.location();
			let loc = loc.origin().unwrap();
			client
				.post(format!("{}/api/drive", loc))
				.query(&parems)
				.send()
				.await
				.unwrap();
		})
	}
}

impl Element for Drive {
	fn get(&mut self) {}
	fn name(&self) -> &'static str {
		"drive"
	}
	fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
		egui::Window::new(self.name()).open(open).show(ctx, |ui| {
			ui.horizontal(|ui| {
				if ui.button("enable").clicked() {
					spawn_local(async {
						let client = reqwest::Client::new();
						let window = web_sys::window().unwrap();
						let loc = window.location();
						let loc = loc.origin().unwrap();
						client
							.post(format!("{}/api/enable", loc))
							.send()
							.await
							.unwrap();
					})
				}
				if ui.button("disable").clicked() {
					spawn_local(async {
						let client = reqwest::Client::new();
						let window = web_sys::window().unwrap();
						let loc = window.location();
						let loc = loc.origin().unwrap();
						client
							.post(format!("{}/api/disable", loc))
							.send()
							.await
							.unwrap();
					})
				}
			});
			ui.separator();
			egui::Grid::new("some_unique_id").show(ui, |ui| {
				if ui.button("⬉").clicked() {
					self.drive(1.0, -1.0)
				}
				if ui.button("⬆").clicked() {
					self.drive(1.0, 0.0)
				}
				if ui.button("⬈").clicked() {
					self.drive(1.0, -1.0)
				}

				ui.end_row();
				if ui.button("⬅").clicked() {
					self.drive(0.0, -1.0)
				}
				if ui.button("■").clicked() {
					self.drive(0.0, 0.0)
				}
				if ui.button("➡").clicked() {
					self.drive(0.0, 1.0)
				}
				ui.end_row();

				if ui.button("⬋").clicked() {
					self.drive(-1.0, -1.0)
				}
				if ui.button("⬇").clicked() {
					self.drive(-1.0, 0.0)
				}
				if ui.button("⬊").clicked() {
					self.drive(-1.0, 1.0)
				}
				ui.end_row();
			});
		});
	}
}
