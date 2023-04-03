pub mod color_transition;
pub mod tile;

use crate::emotion::EmocatTextAnalysis;
use crate::utils::{scale_font, ScreenDimensions};
use notan::draw::*;
use notan::log;
use notan::math::Vec2;
use notan::prelude::*;

const TITLE_COLOR: Color = Color::BLACK;
const META_COLOR: Color = Color::GRAY;
const FONT_RESIZE_STEP: f32 = 2.0;


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
        let max_width = work_size.x * 0.75;
        let max_height = work_size.y * 0.75;

        let font_size =
            get_optimal_text_size(draw, font, text, work_size, 32.0, max_width, max_height);
        draw.text(&font, &text)
            .alpha_mode(BlendMode::OVER)
            .color(self.get_text_color())
            // NOTE: These draw.text fonts size differently than font sizes in egui
            .size(scale_font(font_size, work_size))
            .max_width(max_width)
            .position(work_size.x * 0.5 - max_width * 0.5, work_size.y * 0.5)
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
        let help_font_size = 18.0;
        draw.text(&font, &text)
            .alpha_mode(BlendMode::OVER)
            .color(Color::TRANSPARENT)
            // NOTE: These draw.text fonts size differently than font sizes in egui
            .size(scale_font(help_font_size, work_size))
            .max_width(textbox_width)
            .position(work_size.x * 0.5 - textbox_width * 0.5, work_size.y * 0.8)
            .v_align_middle()
            .h_align_left();
        let help_bounds = draw.last_text_bounds();
        let rect_ypos = help_bounds.y - work_size.y * 0.02;
        draw.rect((0.0, rect_ypos), (work_size.x, work_size.y - rect_ypos))
            .color(bgcolor);
        draw.text(&font, &text)
            .alpha_mode(BlendMode::OVER)
            .color(txtcolor)
            .size(scale_font(help_font_size, work_size))
            .max_width(textbox_width)
            .position(work_size.x * 0.5 - textbox_width * 0.5, work_size.y * 0.8)
            .v_align_middle()
            .h_align_left();
    }
}


/// Draw text and check bounds recursively to find nearest fit for oversized text
pub fn get_optimal_text_size(
    draw: &mut Draw,
    font: &Font,
    text: &str,
    work_size: Vec2,
    font_size: f32,
    max_width: f32,
    max_height: f32,
) -> f32 {
    draw.text(&font, &text)
        .alpha_mode(BlendMode::OVER)
        .color(Color::TRANSPARENT)
        .size(scale_font(font_size, work_size))
        .max_width(max_width)
        .position(work_size.x * 0.5 - max_width * 0.5, work_size.y * 0.5)
        .v_align_middle()
        .h_align_left();

    let para_bounds = draw.last_text_bounds();
    // log::debug!("{} > {}?", para_bounds.height, max_height);
    if para_bounds.height > max_height {
        let font_size = font_size - FONT_RESIZE_STEP;
        // log::debug!("Font resized to {}", font_size);
        return get_optimal_text_size(
            draw, font, text, work_size, font_size, max_width, max_height,
        );
    }
    font_size
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
