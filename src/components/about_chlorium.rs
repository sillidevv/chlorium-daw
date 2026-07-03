const LABEL_SPACING: f32 = 12.0;

pub struct AboutChlorium {
	pub(crate) show: bool,
}

impl AboutChlorium {
	pub fn new() -> Self {
		Self {
			show: false,
		}
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
				built fully in rust with the help of egui."
		);

		ui.add_space(LABEL_SPACING);

		ui.label(
			"Since I started using the 'Renoise' tracker, \
			I've been really interested in trackers, and was inspired to try to \
			build my own. Trackers being (in concept) relatively simple than other \
			types of DAWs, I decided to start making Chlorium."

		);

		ui.add_space(LABEL_SPACING);

		ui.label(
			"Don't expect anything good or serious... this started mostly out of \
			boredom."
		);

		ui.add_space(LABEL_SPACING);

		ui.heading("Links");
		Self::links(ui);
	}

	fn links(ui: &mut egui::Ui) {
		use egui::special_emojis::GITHUB;

		ui.hyperlink_to(
			format!("Github - {GITHUB} github.com/sillidevv/chlorium-daw"),
			"https://github.com/sillidevv/chlorium-daw",
		);
	}

	pub fn draw(&mut self, ctx: &egui::Context) {
		if !self.show {
			return;
		}

		egui::Window::new("About Chlorium")
			.default_width(600.0)
			.show(ctx, |ui| {
				self.draw_content(ui);
			});
	}
}
