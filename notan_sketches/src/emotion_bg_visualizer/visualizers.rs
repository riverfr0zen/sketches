pub mod color_transition;
pub mod tile;

use crate::emotion::EmocatTextAnalysis;
use crate::utils::{scale_font, ScreenDimensions};
use notan::draw::*;
use notan::math::Vec2;
use notan::prelude::*;

const TITLE_COLOR: Color = Color::BLACK;
const META_COLOR: Color = Color::GRAY;


#[derive(PartialEq)]
pub enum VisualizerSelection {
    Tiles,
    ColorTransition,
}

pub trait EmoVisualizer {
    fn get_enum(&self) -> VisualizerSelection;

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

    /// The model would be updated whenever a new text analysis is requested.
    /// Also a good place to update any state props that only change alongside
    /// the analysis.
    fn update_model(&mut self, analysis: &EmocatTextAnalysis);

    fn update_visualization(&mut self);

    fn get_text_color(&self) -> Color;

    fn draw(&mut self, graphics: &mut Graphics, draw: &mut Draw);


    fn draw_title(
        &mut self,
        draw: &mut Draw,
        title_font: &Font,
        title: &str,
        author: &str,
        work_size: Vec2,
    ) {
        let mut textbox_width = work_size.x * 0.75;

        draw.text(&title_font, &title)
            .alpha_mode(BlendMode::OVER) // Fixes some artifacting -- gonna be default in future Notan
            .color(TITLE_COLOR)
            // NOTE: These draw.text fonts size differently than font sizes in egui
            .size(scale_font(60.0, work_size))
            .max_width(textbox_width)
            .position(work_size.x * 0.5 - textbox_width * 0.5, work_size.y * 0.4)
            .h_align_left()
            .v_align_middle();

        let title_bounds = draw.last_text_bounds();

        textbox_width = textbox_width * 0.9;
        draw.text(&title_font, &format!("by {}", author))
            .alpha_mode(BlendMode::OVER)
            .color(META_COLOR)
            .size(scale_font(30.0, work_size))
            .max_width(textbox_width)
            .position(
                work_size.x * 0.5 - textbox_width * 0.5,
                title_bounds.y + title_bounds.height + work_size.y * 0.1,
            )
            .h_align_left()
            .v_align_middle();
    }

    fn draw_paragraph(&mut self, draw: &mut Draw, font: &Font, text: &str, work_size: Vec2) {
        let textbox_width = work_size.x * 0.75;

        draw.text(&font, &text)
            .alpha_mode(BlendMode::OVER)
            .color(self.get_text_color())
            // NOTE: These draw.text fonts size differently than font sizes in egui
            .size(scale_font(32.0, work_size))
            .max_width(textbox_width)
            .position(work_size.x * 0.5 - textbox_width * 0.5, work_size.y * 0.5)
            .v_align_middle()
            .h_align_left();
    }

    fn draw_read_help(
        &mut self,
        draw: &mut Draw,
        font: &Font,
        text: &str,
        work_size: Vec2,
        txtcolor: Color,
        bgcolor: Color,
    ) {
        let textbox_width = work_size.x * 0.9;
        draw.rect((0.0, work_size.y * 0.7), (work_size.x, work_size.y * 0.3))
            .color(bgcolor);
        draw.text(&font, &text)
            .alpha_mode(BlendMode::OVER)
            .color(txtcolor)
            // NOTE: These draw.text fonts size differently than font sizes in egui
            .size(scale_font(24.0, work_size))
            .max_width(textbox_width)
            .position(work_size.x * 0.5 - textbox_width * 0.5, work_size.y * 0.8)
            .v_align_middle()
            .h_align_left();
    }
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
