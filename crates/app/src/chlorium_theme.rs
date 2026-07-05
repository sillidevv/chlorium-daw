use catppuccin_egui::MOCHA;
use eframe::epaint::text::{FontData, FontDefinitions};
use eframe::epaint::{FontFamily, FontId};
use egui::{Color32, Context, Style, Visuals};

pub mod color_palette {
	use egui::Color32;

	pub const FULL_ACCENT: Color32 = Color32::from_rgb(43, 140, 72);

	pub const GRAY_BG_1: Color32 = Color32::from_rgb(20, 20, 22);
	pub const GRAY_BG_2: Color32 = Color32::from_rgb(26, 27, 29);
	pub const COLOR_BG_1: Color32 = Color32::from_rgb(14, 23, 15);
	pub const COLOR_BG_2: Color32 = Color32::from_rgb(18, 27, 20);

	pub const GRAY_INTERACTIVE_1: Color32 = Color32::from_rgb(35, 35, 38);
	pub const GRAY_INTERACTIVE_2: Color32 = Color32::from_rgb(41, 42, 46);
	pub const GRAY_INTERACTIVE_3: Color32 = Color32::from_rgb(48, 49, 54);
	pub const COLOR_INTERACTIVE_1: Color32 = Color32::from_rgb(21, 44, 26);
	pub const COLOR_INTERACTIVE_2: Color32 = Color32::from_rgb(15, 61, 28);
	pub const COLOR_INTERACTIVE_3: Color32 = Color32::from_rgb(19, 75, 36);

	pub const GRAY_BORDER_1: Color32 = Color32::from_rgb(57, 58, 64);
	pub const GRAY_BORDER_2: Color32 = Color32::from_rgb(70, 72, 79);
	pub const GRAY_BORDER_3: Color32 = Color32::from_rgb(95, 96, 106);
	pub const COLOR_BORDER_1: Color32 = Color32::from_rgb(23, 91, 44);
	pub const COLOR_BORDER_2: Color32 = Color32::from_rgb(26, 108, 52);
	pub const COLOR_BORDER_3: Color32 = Color32::from_rgb(24, 127, 60);

	pub const GRAY_SOLID_1: Color32 = Color32::from_rgb(108, 110, 121);
	pub const GRAY_SOLID_2: Color32 = Color32::from_rgb(121, 123, 134);
	pub const COLOR_SOLID_1: Color32 = Color32::from_rgb(43, 140, 72);
	pub const COLOR_SOLID_2: Color32 = Color32::from_rgb(23, 126, 59);

	pub const GRAY_TEXT_1: Color32 = Color32::from_rgb(178, 179, 189);
	pub const GRAY_TEXT_2: Color32 = Color32::from_rgb(238, 238, 240);
	pub const COLOR_TEXT_1: Color32 = Color32::from_rgb(115, 207, 135);
	pub const COLOR_TEXT_2: Color32 = Color32::from_rgb(172, 246, 186);
}

pub fn install_fonts(ctx: &Context, mono_ttf_bytes: &'static [u8]) {
	let mut fonts = FontDefinitions::default();

	fonts.font_data.insert(
		"tracker_mono".to_owned(),
		FontData::from_static(mono_ttf_bytes).into(),
	);

	fonts
		.families
		.entry(FontFamily::Proportional)
		.or_default()
		.insert(0, "tracker_mono".to_owned());
	fonts
		.families
		.entry(FontFamily::Monospace)
		.or_default()
		.insert(0, "tracker_mono".to_owned());

	ctx.set_fonts(fonts);
}
pub fn install_chlorium_theme(ctx: &Context, font_bytes: &'static [u8]) {
	install_fonts(ctx, font_bytes);
	ctx.set_style(build_style());
}

fn build_style() -> Style {
	let mut style = Style::default();
	let v = &mut style.visuals;
	*v = Visuals::dark();

	v.override_text_color = Some(color_palette::COLOR_TEXT_2);
	v.window_fill = MOCHA.base;
	v.panel_fill = color_palette::GRAY_BG_1;
	v.widgets.noninteractive.bg_fill = MOCHA.surface2;
	v.window_fill = color_palette::GRAY_BG_2;
	v.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, color_palette::GRAY_INTERACTIVE_1);
	v.widgets.inactive.bg_fill = Color32::from_hex("#172448").unwrap();
	v.widgets.inactive.weak_bg_fill = color_palette::COLOR_INTERACTIVE_1;
	v.widgets.hovered.bg_fill = MOCHA.surface2;
	v.widgets.active.bg_fill = MOCHA.overlay0;
	v.selection.bg_fill = MOCHA.mauve;
	v.hyperlink_color = MOCHA.blue;
	v.error_fg_color = MOCHA.red;
	v.warn_fg_color = MOCHA.peach;

	style.text_styles.insert(
		egui::TextStyle::Small,
		FontId::new(10.0, FontFamily::Monospace),
	);
	style.text_styles.insert(
		egui::TextStyle::Body,
		FontId::new(12.5, FontFamily::Monospace),
	);
	style.text_styles.insert(
		egui::TextStyle::Button,
		FontId::new(12.5, FontFamily::Monospace),
	);
	style.text_styles.insert(
		egui::TextStyle::Heading,
		FontId::new(15.0, FontFamily::Monospace),
	);
	style.text_styles.insert(
		egui::TextStyle::Monospace,
		FontId::new(12.5, FontFamily::Monospace),
	);

	return style;
}
