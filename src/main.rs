#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod chlorium_theme;
mod components;
mod traits;
mod utils;

fn main() -> eframe::Result {
	env_logger::init(); // Log to stderr if you run with RUST_LOG=debug

	let native_options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default()
			//.with_inner_size([400.0, 300.0])
			.with_min_inner_size([300.0, 220.0])
			.with_maximized(true)
			.with_icon(
				eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
					.expect("Failed to load icon"),
			),
		..Default::default()
	};

	eframe::run_native(
		"Chlorium",
		native_options,
		Box::new(|cc| {
			let font_bytes: &'static [u8] =
				include_bytes!("../assets/JetBrainsMono-Regular.ttf");
			chlorium_theme::install_chlorium_theme(&cc.egui_ctx, font_bytes);
			Ok(Box::new(app::ChlorideApp::new(cc)))
		}),
	)
}
