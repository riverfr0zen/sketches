pub mod color_transition;
pub mod tile;

use crate::emotion::EmocatTextAnalysis;
use notan::draw::*;
use notan::prelude::*;


pub trait EmoVisualizer {
    /// Similar to new(), but does not reset user-configurable properties
    fn reset(&mut self, bg_color: Color, text_color: Color, enable_dynamic_text_color: bool);

    /// Used for a less abrupt transition between reading a paragraph and the title screen of a
    /// work
    fn gracefully_reset(
        &mut self,
        bg_color: Color,
        text_color: Color,
        enable_dynamic_text_color: bool,
    );

    fn get_text_color(&self) -> Color;

    /// The model would be updated whenever a new text analysis is requested.
    /// Also a good place to update any state props that only change alongside
    /// the analysis.
    fn update_model(&mut self, analysis: &EmocatTextAnalysis);

    fn update_visualization(&mut self);

    fn draw(&mut self, draw: &mut Draw);
}


/// Return black or white depending on provided background color
///
/// Based on this algorithm:
/// https://stackoverflow.com/a/1855903/4655636
///
pub fn get_optimal_text_color(bgcolor: &Color) -> Color {
    let luminance: f32;
    luminance = 0.299 * bgcolor.r + 0.587 * bgcolor.g + 0.114 * bgcolor.b / 255.0;

    // log::debug!("Luminance {}", luminance);
    if luminance < 0.5 {
        return Color::WHITE;
    }
    Color::BLACK
}
