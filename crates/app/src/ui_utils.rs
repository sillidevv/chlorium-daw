/* -----------------------------------------------------------------------------
 * | If you're reading this... dont ask why this is so much more documented
 * | than literally any other file in this project, it just so happened
 * | that I decided that I should probably focus on documenting the api
 * | as i was making this...
 * -----------------------------------------------------------------------------
 */

//! Contains small egui ui utils

/// Contains methods for drawing egui hyperlinks.
/// This should be used whenever there is a hyperlink in the ui
/// to keep consistent styling.
pub mod hyperlinks {
	/// Defines the 'type" of a hyperlink, usually the site it's pointing to.
	///
	/// This is mostly used to decide what icon to use in [`standard_hyperlink()`].
	#[derive(Clone)]
	pub enum HyperlinkType {
		GitHub,

		/// Represents any hyperlink that hasn't been given a separate [`HyperlinkType`] variant.
		///
		/// The `Option<&str>`
		Other(Option<String>),
	}

	use HyperlinkType::*;

	/// Draws a normal hyperlink.
	///
	/// # Arguments
	/// - `ui` - Self explanatory.
	/// - `to` - The actual url of the hyperlink. The `https://` prefix gets removed.
	pub fn standard_hyperlink(ui: &mut egui::Ui, hyperlink_type: HyperlinkType, to: &str, text_override: Option<&str>) {
		let mut hyperlink_text = String::new();
		let icon = get_hyperlink_icon(hyperlink_type.clone());

		if let Some(icon_char) = icon {
			hyperlink_text.push_str(&icon_char.to_string());
			hyperlink_text.push_str(" ");
		}

		if let Some(text_override) = text_override {
			hyperlink_text.push_str(&text_override.to_string());
		}
		else {
			// this decides how much of the prefix to remove
			match hyperlink_type {
				GitHub =>
					hyperlink_text.push_str(to.strip_prefix("https://github.com/").unwrap_or(to)),

				_ => hyperlink_text.push_str(to.strip_prefix("https://").unwrap()),
			}
		}

		ui.hyperlink_to(
			hyperlink_text,
			to,
		);
	}

	/// Maps a [`HyperlinkType`] to an actual `char` icon.
	///
	/// In the case of [`Other`] it just returns `c?.chars.next`,
	/// so if it's `None` or the string is empty it returns `None`.
	fn get_hyperlink_icon(typ: HyperlinkType) -> Option<char> {
		use egui::special_emojis;

		match typ {
			GitHub => Some(special_emojis::GITHUB),
			Other(c) => c?.chars().next(),
		}
	}
}