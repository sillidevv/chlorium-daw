use crate::ui_utils::hyperlinks::{standard_hyperlink, HyperlinkType};

const LABEL_SPACING: f32 = 12.0;

pub struct AboutChlorium {
	pub show: bool,
}

impl AboutChlorium {
	pub fn new() -> Self {
		Self { show: false }
	}

	pub fn toggle(&mut self) {
		self.show = !self.show;
	}
}

impl AboutChlorium {
	fn draw_content(&self, ui: &mut egui::Ui) {
		ui.vertical_centered(|ui| {
			ui.heading("+ chlorium +");
		});

		ui.separator();

		ui.label(
			"Chlorium is a very simple tracker-ish DAW (Digital Audio Workstation) \
				built fully in rust with the help of egui.",
		);

		ui.add_space(LABEL_SPACING);

		ui.label(
			"Since I started using the 'Renoise' tracker, \
			I've been really interested in trackers, and was inspired to try to \
			build my own. Trackers being (in concept) relatively simpler to make \
			than other types of DAWs, I decided to start making Chlorium.",
		);

		ui.add_space(LABEL_SPACING);

		ui.label(
			"Don't expect anything good or serious... this started mostly out of \
			boredom.",
		);

		ui.add_space(LABEL_SPACING);

		ui.heading("Links");
		Self::links(ui);

		ui.add_space(LABEL_SPACING);

		egui::CollapsingHeader::new("Licensing")
			.default_open(false)
			.show(ui, |ui| {
				Self::licensing(ui);
			});
	}

	fn links(ui: &mut egui::Ui) {
		standard_hyperlink(
			ui,
			HyperlinkType::GitHub,
			"https://github.com/sillidevv/chlorium-daw",
			None, //Some("(link)")
		);
	}

	fn licensing(ui: &mut egui::Ui) {
		ui.horizontal_wrapped(|ui| {
			ui.label(
				"Chlorium is licensed under either of\n\
				\x20* Apache License, Version 2.0 -",
			);

			standard_hyperlink(
				ui,
				HyperlinkType::GitHub,
				"https://github.com/sillidevv/chlorium-daw/blob/main/LICENSE-APACHE",
				Some("(link)"),
			);

			ui.label("\n\x20* MIT License -");

			standard_hyperlink(
				ui,
				HyperlinkType::GitHub,
				"https://github.com/sillidevv/chlorium-daw/blob/main/LICENSE-MIT",
				Some("(link)"),
			);

			ui.label("\nat your option.");
		});
	}

	pub fn draw(&mut self, ctx: &egui::Context) {
		let mut is_open = self.show;

		egui::Window::new("About Chlorium")
			.default_width(600.0)
			.open(&mut is_open)
			.show(ctx, |ui| {
				self.draw_content(ui);
			});

		self.show = is_open;
	}
}
