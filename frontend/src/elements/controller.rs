use std::sync::{Arc, Mutex};

use futures::{stream::SplitSink, SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;

use crate::Element;

pub struct Controller {
	controllers: Option<Vec<(usize, String)>>,
	selected: Option<usize>,
	config: Option<SelectedOptions>,
	ws_write: Arc<Mutex<Option<SplitSink<WebSocket, Message>>>>,
	stream: bool,
}

#[derive(Clone)]
pub struct SelectedOptions {
	drive_axis: usize,
	drive_rev: bool,
	steer_axis: usize,
	steer_rev: bool,
}

impl Default for SelectedOptions {
	fn default() -> Self {
		Self {
			drive_axis: 1,
			drive_rev: false,
			steer_axis: 0,
			steer_rev: true,
		}
	}
}

#[derive(Serialize, Clone)]
pub struct DriveQuery {
	accelerate: f64,
	steer: f64,
}

impl Default for Controller {
	fn default() -> Self {
		Self {
			controllers: None,
			selected: None,
			config: None,
			ws_write: Arc::new(Mutex::new(None)),
			stream: false,
		}
	}
}

impl Element for Controller {
	fn get(&mut self) {
		let window = web_sys::window().unwrap();
		let navigator = window.navigator();
		let gamepads = match navigator.get_gamepads() {
			Ok(a) => Some(a),
			Err(_) => None,
		};
		self.controllers = gamepads
			.map(|g| {
				g.iter()
					.filter(|gp| !gp.is_null() && !gp.is_undefined())
					.map(|gp| {
						let gp: web_sys::Gamepad = gp.into();
						gp.id()
					})
					.enumerate()
					.collect()
			})
			.and_then(|v: Vec<(usize, String)>| if v.is_empty() { None } else { Some(v) });
		let loc = window.location().host().unwrap();
		let ws = WebSocket::open(&format!("ws://{}/ws", loc)).unwrap();
		let (write, _) = ws.split();
		*self.ws_write.lock().unwrap() = Some(write);
	}
	fn name(&self) -> &'static str {
		"controller"
	}
	fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
		egui::Window::new(self.name())
			.open(open)
			.show(ctx, |ui| match &self.controllers {
				Some(d) => {
					//if we get the gamepad, aways repaint
					ctx.request_repaint();
					egui::ComboBox::from_label("Gamepad")
						.selected_text(
							self.selected
								.as_ref()
								.map(|s| d[*s].1.clone())
								.unwrap_or_else(|| "None".to_string()),
						)
						.show_ui(ui, |ui| {
							for gp in d {
								ui.selectable_value(&mut self.selected, Some(gp.0), &gp.1);
							}
						});

					ui.separator();
					if self.selected.is_some() {
						let gamepad = self.get_gamepad().unwrap();
						let axes: Vec<f64> =
							gamepad.axes().iter().map(|d| d.as_f64().unwrap()).collect();
						#[allow(clippy::needless_collect)]
						let buttons: Vec<web_sys::GamepadButton> = gamepad.buttons().iter().map(|x| x.into()).collect();

						ui.label(format!(
							"gamepad has {} axes and {} buttons",
							axes.len(),
							buttons.len()
						));
						ui.checkbox(&mut self.stream, "start data flow");

						let mut config;
						if let Some(c) = self.config.clone() {
							config = c;
						} else {
							config = SelectedOptions::default()
						}

						let query = DriveQuery {
							accelerate: axes[config.drive_axis]
								* if config.drive_rev { 1.0 } else { -1.0 },
							steer: axes[config.steer_axis]
								* if config.steer_rev { 1.0 } else { -1.0 },
						};

						//TODO: custom widget?
						ui.add(
							egui::ProgressBar::new((query.accelerate + 1.0 / 2.0) as f32)
								.text("drive"),
						);
						ui.add(
							egui::ProgressBar::new((query.steer + 1.0 / 2.0) as f32).text("steer"),
						);

						if self.stream {
							self.send_ws(query);
						}

						ui.horizontal(|ui| {
							egui::ComboBox::from_label("throttle")
								.selected_text(
									self.config
										.as_ref()
										.ok_or("None")
										.map(|s| format!("axis {}", s.drive_axis))
										.unwrap_or_else(|_| "None".to_string()),
								)
								.show_ui(ui, |ui| {
									for i in 0..axes.len() {
										ui.selectable_value(
											&mut config.drive_axis,
											i,
											format!("axis {}", i),
										);
									}
								});
							ui.checkbox(&mut config.drive_rev, "reverse")
						});

						ui.horizontal(|ui| {
							egui::ComboBox::from_label("steer")
								.selected_text(
									self.config
										.as_ref()
										.ok_or("None")
										.map(|s| format!("axis {}", s.steer_axis))
										.unwrap_or_else(|_| "None".to_string()),
								)
								.show_ui(ui, |ui| {
									for i in 0..axes.len() {
										ui.selectable_value(
											&mut config.steer_axis,
											i,
											format!("axis {}", i),
										);
									}
								});
							ui.checkbox(&mut config.steer_rev, "reverse")
						});
						self.config = Some(config)
					}
				}
				None => {
					ui.label("no gamepads detected; try pressing a button on the controller to wake it up");
					if ui.button("try agiain").clicked() {
						self.get()
					}
				}
			});
	}
}

impl Controller {
	fn send_ws(&mut self, query: DriveQuery) {
		let ws_write = self.ws_write.clone();
		spawn_local(async move {
			(*ws_write.lock().unwrap())
				.as_mut()
				.unwrap()
				.send(Message::Text(serde_json::to_string(&query).unwrap()))
				.await
				.unwrap();
		});
	}
	fn get_gamepad(&self) -> Option<web_sys::Gamepad> {
		self.selected.and_then(|id| {
			let window = web_sys::window().unwrap();
			let navigator = window.navigator();
			let gamepads = match navigator.get_gamepads() {
				Ok(a) => Some(a),
				Err(_) => None,
			};

			// only return Some if value is not Null or Undefined
			let gamepad = gamepads?.at(id as i32);
			if gamepad.is_null() || gamepad.is_undefined() {
				None
			} else {
				Some(gamepad.into())
			}
		})
	}
}
