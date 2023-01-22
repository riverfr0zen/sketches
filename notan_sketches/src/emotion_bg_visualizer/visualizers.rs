use crate::emotion::{EmocatTextAnalysis, TopEmotionsModel};
use notan::draw::*;
use notan::prelude::*;
use palette::{FromColor, LinSrgb, Mix, Srgb};
use std::collections::HashMap;


const COLOR_COMPARISON_PRECISION: f32 = 3.0;
const STARTING_MIX_FACTOR: f32 = 0.0;
/// Because vsync in wasm seems to be non-negotiable (I think), we need a faster mix rate
/// to match what it looks like in native
#[cfg(target_arch = "wasm32")]
const MIX_RATE: f32 = 0.0001;
#[cfg(not(target_arch = "wasm32"))]
const MIX_RATE: f32 = 0.00001;
// const MIX_RATE: f32 = 0.000001;


fn round(val: f32, digits: f32) -> f32 {
    // log::debug!("{}, {}", val, (val * 100.0).round() / 100.0);
    // (val * 100.0).round() / 100.0

    let mut multiplier: f32 = 10.0;
    multiplier = multiplier.powf(digits);
    // log::debug!("{}, {}", val, (val * multiplier).round() / multiplier);
    (val * multiplier).round() / multiplier
}


pub struct ColorTransitionVisualizer {
    pub model: Option<TopEmotionsModel>,
    pub color_method: String,
    pub target_color: Color,
    pub bg_color: Color,
    pub bg_color_mix_factor: f32,
    pub text_color: Color,
    pub dynamic_text_color: bool,
}

impl ColorTransitionVisualizer {
    pub fn new(bg_color: Color, text_color: Color, enable_dynamic_text_color: bool) -> Self {
        Self {
            model: None,
            color_method: "Simple Color".to_string(),
            target_color: bg_color,
            bg_color: bg_color,
            bg_color_mix_factor: STARTING_MIX_FACTOR,
            text_color: text_color,
            dynamic_text_color: enable_dynamic_text_color,
        }
    }

    /// Return black or white depending on the current background color
    ///
    /// Based on this algorithm:
    /// https://stackoverflow.com/a/1855903/4655636
    ///
    pub fn get_text_color(&self) -> Color {
        let luminance: f32;
        if self.dynamic_text_color {
            luminance =
                0.299 * self.bg_color.r + 0.587 * self.bg_color.g + 0.114 * self.bg_color.b / 255.0;
        } else {
            luminance = 0.299 * self.target_color.r
                + 0.587 * self.target_color.g
                + 0.114 * self.target_color.b / 255.0;
        }

        // log::debug!("Luminance {}", luminance);
        if luminance < 0.5 {
            return Color::WHITE;
        }
        Color::BLACK
    }

    pub fn update_text_color(&mut self) {
        self.text_color = self.get_text_color();
    }

    // pub fn update_bg_color(app: &App, state: &mut State) {
    pub fn update_bg_color(&mut self) {
        // The mix function used to blend colors below doesn't always end up with the
        // exact floating point numbers of the end color, so comparing with rounded
        // color values instead of comparing the colors directly.
        let precision = COLOR_COMPARISON_PRECISION;
        // log::debug!(
        //     "{}::{}, {}::{}, {}::{}",
        //     round(state.bg_color.r, precision),
        //     round(state.target_color.r, precision),
        //     round(state.bg_color.g, precision),
        //     round(state.target_color.g, precision),
        //     round(state.bg_color.b, precision),
        //     round(state.target_color.b, precision),
        // );
        if round(self.bg_color.r, precision) != round(self.target_color.r, precision)
            || round(self.bg_color.g, precision) != round(self.target_color.g, precision)
            || round(self.bg_color.b, precision) != round(self.target_color.b, precision)
        {
            // log::debug!("Mix factor: {}", state.bg_color_mix_factor);
            let bg_color = Srgb::new(self.bg_color.r, self.bg_color.g, self.bg_color.b);
            let target_color = Srgb::new(
                self.target_color.r,
                self.target_color.g,
                self.target_color.b,
            );
            let mut bg_color = LinSrgb::from_color(bg_color);
            let target_color = LinSrgb::from_color(target_color);
            bg_color = bg_color.mix(&target_color, self.bg_color_mix_factor);
            let bg_color = Srgb::from_color(bg_color);
            self.bg_color = Color::from_rgb(bg_color.red, bg_color.green, bg_color.blue);
            self.bg_color_mix_factor += MIX_RATE;
        } else {
            self.bg_color_mix_factor = STARTING_MIX_FACTOR;
        }
    }


    pub fn update_bg_color_simple(&mut self) {
        self.bg_color = self.target_color.clone();
    }
}


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


impl EmoVisualizer for ColorTransitionVisualizer {
    fn new(&mut self, bg_color: Color, text_color: Color, enable_dynamic_text_color: bool) -> Self {
        return Self::new(bg_color, text_color, enable_dynamic_text_color);
    }

    fn reset(&mut self, bg_color: Color, text_color: Color, enable_dynamic_text_color: bool) {
        self.bg_color = bg_color;
        self.target_color = bg_color;
        self.bg_color_mix_factor = STARTING_MIX_FACTOR;
        self.text_color = text_color;
        self.dynamic_text_color = enable_dynamic_text_color;
    }

    fn gracefully_reset(
        &mut self,
        bg_color: Color,
        _text_color: Color,
        _enable_dynamic_text_color: bool,
    ) {
        self.target_color = bg_color;
    }

    fn get_options() -> HashMap<String, Vec<String>> {
        let mut options = HashMap::new();
        options.insert(
            "Color Method".to_string(),
            vec![
                "Simple Color".to_string(),
                "Black, White, Gray".to_string(),
                "Grayscale".to_string(),
            ],
        );
        options
    }

    fn draw(&self, draw: &mut Draw) {
        // The following call to clear() is important when rendering draw & egui output together.
        draw.clear(self.bg_color);
    }

    fn update_model(&mut self, analysis: &EmocatTextAnalysis) {
        let model = TopEmotionsModel::from_analysis(&analysis);
        match self.color_method.as_str() {
            "Simple Color" => self.target_color = model.get_simple_color(),
            "Black, White, Gray" => self.target_color = model.get_black_or_white(),
            "Grayscale" => self.target_color = model.get_grayscale(),
            _ => {}
        }
        self.model = Some(model);
    }

    fn update_visualization(&mut self) {
        self.update_bg_color();
        self.update_text_color();
    }
}
