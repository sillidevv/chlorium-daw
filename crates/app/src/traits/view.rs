//! Contains the [`View`] trait.

/// Trait representing something that can be drawn onto a UI
pub trait View {
	fn ui(&mut self, ui: &mut egui::Ui);
}
