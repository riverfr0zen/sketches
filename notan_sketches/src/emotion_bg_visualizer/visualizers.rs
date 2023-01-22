use crate::emotion::EmocatTextAnalysis;
use notan::draw::*;
use notan::prelude::*;
use std::collections::HashMap;


pub mod color_transition;

pub trait EmoVisualizer {
    fn new(&mut self, bg_color: Color, text_color: Color, enable_dynamic_text_color: bool) -> Self;

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

    fn get_options() -> HashMap<String, Vec<String>>;

    fn draw(&self, draw: &mut Draw);

    fn update_model(&mut self, analysis: &EmocatTextAnalysis);

    fn update_visualization(&mut self);
}
