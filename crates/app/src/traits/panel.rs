//! Contains the [`Show`] struct.

use egui::Id;

/// Similar to [`View`], but instead represents anything that takes
/// the egui context instead of the ui
pub trait Show {
	fn id(&self) -> impl Into<Id>;

	/// Shows the panel onto an egui context
	fn show(&mut self, ctx: &mut egui::Context);

	/// Draw's the panels inner ui
	fn ui(&mut self, ctx: &mut egui::Ui);
}
