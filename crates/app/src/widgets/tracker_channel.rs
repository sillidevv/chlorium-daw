use crate::chlorium_theme::color_palette;
use crate::traits::view::View;
use egui::Margin;

const TRACK_WIDTH: f32 = 75.0;
const CONTROLS_MARGIN: i8 = 3;
const HEADER_SPACING: f32 = 10.0;

pub struct TrackerChannel {
	name: String,
	test_value: i32,
}

impl TrackerChannel {
	pub fn new(name: String) -> Self {
		Self {
			name,
			test_value: 0,
		}
	}

	fn draw_header(&self, ui: &mut egui::Ui) {
		ui.vertical_centered_justified(|ui| {
			ui.label(&self.name);
		});

		ui.separator();
		ui.add_space(HEADER_SPACING);
	}

	fn draw_canvas(ui: &mut egui::Ui) {
		egui::Frame::canvas(ui.style())
			.fill(color_palette::GRAY_BG_1)
			.outer_margin(Margin::ZERO)
			.show(ui, |ui| {
				let desired_size = ui.available_size();

				ui.allocate_exact_size(desired_size, egui::Sense::hover());
			});
	}

	fn draw_controls(&mut self, ui: &mut egui::Ui) {
		egui::Frame::default()
			.inner_margin(Margin::same(CONTROLS_MARGIN))
			.show(ui, |ui| {
				ui.vertical_centered(|ui| Self::draw_canvas(ui));

				// ui.add(egui::DragValue::new(&mut self.test_value));
			});
	}
}

impl View for TrackerChannel {
	fn ui(&mut self, ui: &mut egui::Ui) {
		ui.allocate_ui_with_layout(
			egui::vec2(TRACK_WIDTH, ui.available_height()),
			egui::Layout::top_down(egui::Align::LEFT),
			|ui| {
				egui::Frame::new()
					.fill(color_palette::GRAY_BG_2)
					.corner_radius(0)
					.show(ui, |ui| {
						ui.vertical(|ui| {
							self.draw_header(ui);

							ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
								self.draw_controls(ui)
							});
						});
					});
			},
		);
	}
}
