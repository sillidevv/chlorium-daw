use crate::discord_rpc::ChloriumDRPC;
use crate::traits::view::View;
use crate::widgets::about_chlorium::AboutChlorium;
use crate::widgets::tracker_channel::TrackerChannel;
use eframe::glow::Context;
use crate::widgets::about_chlorium::AboutChlorium;
use crate::discord_rpc::ChloriumDRPC;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
///#[derive(serde::Deserialize, serde::Serialize)]
//#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ChloriumApp {
	// Example stuff:
	label: String,

	//#[serde(skip)] // This how you opt-out of serialization of a field
	value: f32,
	collapsed: bool,

	// Subwindows
	about_chlorium: AboutChlorium,

	pub discord_rpc: ChloriumDRPC,
}

impl Default for ChloriumApp {
	fn default() -> Self {
		Self {
			// Example stuff:
			label: "Hello World!".to_owned(),
			value: 2.7,
			collapsed: false,

			// Subwindows
			about_chlorium: AboutChlorium::new(),

			discord_rpc: ChloriumDRPC::new(),
		}
	}
}

impl ChloriumApp {
	/// Called once before the first frame.
	pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
		// This is also where you can customize the look and feel of egui using
		// `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

		// Load previous app state (if any).
		// if let Some(storage) = cc.storage {
		// 	eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
		// } else {
		// 	Self::default()
		// }

		// ^^ Removed for simplication.
		//
		//    Uncomment the block along with the fn save block
		//    inside the impl eframe::App for TemplateApp if you want persistence.
		//
		//    Make sure to also uncomment the "persistence" feature of eframe in Cargo.toml

		let mut app = Self::default(); // <- Remove this if you enable persistenc

		app.discord_rpc.initialize_drpc();

		app
	}
}

impl eframe::App for ChloriumApp {
	// Save state before shutdown.
	// fn save(&mut self, storage: &mut dyn eframe::Storage) {
	// 	eframe::set_value(storage, eframe::APP_KEY, self);
	// }

	// ^^ Removed for simplication.
	//
	//    Uncomment the block along with the commented code in fn new if you want persistence
	//    Make sure to also uncomment the "persistence" feature of eframe in Cargo.toml

	/// Called each frame.
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		// --- Top bar ---------------------------------------------------
		egui::TopBottomPanel::top("top_bar")
			//.exact_height(40.0)
			.show(ctx, |ui| {
				ui.horizontal(|ui| {
					ui.menu_button("File", |ui| {
						if ui.button("hello").clicked() {
							println!("owo")
						}

						if ui.button("Quit").clicked() {
							std::process::exit(0);
						}

						ui.separator();

						if ui.button("About chlorium").clicked() {
							self.about_chlorium.toggle();
						}
					});
					ui.menu_button("Edit", |ui| {});
					ui.separator();
					if ui.button("▶ Play").clicked() {}
					if ui.button("■ Stop").clicked() {}

					ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
						ui.label("+ CHLORIUM +");
					});
				});
			});

		// --- Left panel ---------------------------------------------------
		egui::SidePanel::left("left_panel")
			.resizable(true)
			.default_width(200.0)
			.show(ctx, |ui| {
				ui.heading("Left panel");

				// if self.collapsed {
				// 	if ui.button("▶").clicked() {
				// 		self.collapsed = false;
				// 	}
				// } else {
				// 	if ui.button("◀ Collapse").clicked() {
				// 		self.collapsed = true;
				// 	}
				// 	ui.separator();
				// 	ui.label("Panel content here");
				// }
			});

		// --- Bottom bar, status panel -------------------------------------
		egui::TopBottomPanel::bottom("status_bar")
			.exact_height(24.0)
			.show(ctx, |ui| {
				ui.label("Row: 1/64  |  Pattern: 01");
			});

		// --- Right panel --------------------------------------------------
		egui::SidePanel::right("assets")
			.resizable(true)
			.exact_width(150.0)
			.show(ctx, |ui| {
				ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
					ui.heading("Assets");

					if ui.button("Import").clicked() {

					};
				});
			});

		// --- Center -------------------------------------------------------
		egui::CentralPanel::default().show(ctx, |ui| {
			egui::TopBottomPanel::top("maybe_mixer")
				.resizable(true)
				.default_height(120.0)
				.show_inside(ui, |ui| {
					ui.centered_and_justified(|ui| {
						ui.label("[PLACEHOLDER, MAYBE MIXER]");
					})
				});

			egui::SidePanel::right("sequence")
				.default_width(24.0)
				.show_inside(ui, |ui| {
					ui.vertical(|ui| {
						// once again sample data, not actual stuff
						ui.add(
							egui::DragValue::new(&mut 1)
								.clamp_range(0..=999)
								.custom_formatter(|n, _| format!("{n:03}"))
						);

						ui.add(
							egui::DragValue::new(&mut 2)
								.clamp_range(0..=999)
								.custom_formatter(|n, _| format!("{n:03}"))
						);

						ui.add(
							egui::DragValue::new(&mut 3)
								.clamp_range(0..=999)
								.custom_formatter(|n, _| format!("{n:03}"))
						);

						ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
							ui.add(
								egui::DragValue::new(&mut 1)
									.clamp_range(0..=999)
									.custom_formatter(|n, _| format!("{n:03}"))
							);

							ui.label(egui::RichText::new("Active").size(10.0));

							ui.separator();

							ui.with_layout(egui::Layout::left_to_right(egui::Align::BOTTOM), |ui| {
								if ui.button("+").clicked() {};
								if ui.button("-").clicked() {};
							})
						})
					})
				});

			egui::TopBottomPanel::top("channel_tools")
				.default_height(24.0)
				.show_inside(ui, |ui| {
					if ui.button("+").clicked() {};
				});

			ui.add_space(10.0);

			ui.centered_and_justified(|ui| {
				ui.scope(|ui| {
					//ui.spacing_mut().item_spacing.x = 0.0;
					ui.horizontal(|ui| {
						TrackerChannel::new("Track 1".to_string()).ui(ui);
						TrackerChannel::new("Track 2".to_string()).ui(ui);
						TrackerChannel::new("Track 3".to_string()).ui(ui);
						TrackerChannel::new("Track 4".to_string()).ui(ui);
						TrackerChannel::new("Track 5".to_string()).ui(ui);
					});


				});

				//ui.label("[TRACKER GRID GOES HERE]");
			})
		});

		// Subwindows and shit go here
		self.about_chlorium.draw(ctx);
	}

	fn on_exit(&mut self, _gl: Option<&Context>) {
	}
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
	ui.horizontal(|ui| {
		ui.spacing_mut().item_spacing.x = 0.0;
		ui.label("Powered by ");
		ui.hyperlink_to("egui", "https://github.com/emilk/egui");
		ui.label(" and ");
		ui.hyperlink_to(
			"eframe",
			"https://github.com/emilk/egui/tree/master/crates/eframe",
		);
		ui.label(".");
	});
}
