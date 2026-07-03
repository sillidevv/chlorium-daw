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
	pub fn draw_content(&self, ui: &mut egui::Ui) {
		ui.vertical_centered(|ui| {
			ui.heading("Chlorium");
		});

		ui.separator();

		ui.label(
		"Chlorium is a tracker-ish daw made mostly out of boredom.\
			This is not a serious project, like at all, so dont expect anything good"
		);

		ui.add_space(LABEL_SPACING);

		ui.label(
			"This is a solo project developed by @sillidev - 'Max' on github"
		);
	}

	pub fn draw(&mut self, ctx: &egui::Context) {
		if !self.show {
			return;
		}

		egui::Window::new("About Chlorium")
			.show(ctx, |ui| {
				self.draw_content(ui);
			});
	}
}
