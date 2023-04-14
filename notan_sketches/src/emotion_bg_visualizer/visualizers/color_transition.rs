use super::get_optimal_text_color;
use super::EmoVisualizer;
use super::VisualizerSelection;
use crate::emotion::{EmocatTextAnalysis, TopEmotionsModel};
use notan::draw::*;
// use notan::log;
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


#[derive(Clone)]
pub struct ColorTransition {
    pub target_color: Color,
    pub color: Color,
    pub mix_factor: f32,
    pub transitioning: bool,
}

impl Default for ColorTransition {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            target_color: Color::WHITE,
            mix_factor: STARTING_MIX_FACTOR,
            transitioning: false,
        }
    }
}


impl ColorTransition {
    pub fn immediate(&mut self) {
        self.color = self.target_color.clone();
        self.transitioning = false;
    }

    pub fn step(&mut self) {
        // The mix function used to blend colors below doesn't always end up with the
        // exact floating point numbers of the end color, so comparing with rounded
        // color values instead of comparing the colors directly.
        let precision = COLOR_COMPARISON_PRECISION;
        // log::debug!(
        //     "{}::{}, {}::{}, {}::{}",
        //     round(self.color.r, precision),
        //     round(self.target_color.r, precision),
        //     round(self.color.g, precision),
        //     round(self.target_color.g, precision),
        //     round(self.color.b, precision),
        //     round(self.target_color.b, precision),
        // );
        if round(self.color.r, precision) != round(self.target_color.r, precision)
            || round(self.color.g, precision) != round(self.target_color.g, precision)
            || round(self.color.b, precision) != round(self.target_color.b, precision)
        {
            self.transitioning = true;
            // log::debug!("Mix factor: {}", state.bg_color_mix_factor);
            let bg_color = Srgb::new(self.color.r, self.color.g, self.color.b);
            let target_color = Srgb::new(
                self.target_color.r,
                self.target_color.g,
                self.target_color.b,
            );
            let mut bg_color = LinSrgb::from_color(bg_color);
            let target_color = LinSrgb::from_color(target_color);
            bg_color = bg_color.mix(&target_color, self.mix_factor);
            let bg_color = Srgb::from_color(bg_color);
            self.color = Color::from_rgb(bg_color.red, bg_color.green, bg_color.blue);
            self.mix_factor += MIX_RATE;
        } else {
            self.mix_factor = STARTING_MIX_FACTOR;
            self.transitioning = false;
        }
    }
}


pub struct ColorTransitionVisualizer {
    pub model: Option<TopEmotionsModel>,
    pub color_method: String,
    pub transition: ColorTransition,
    text_color: Color,
    dynamic_text_color: bool,
}

impl ColorTransitionVisualizer {
    pub fn new(bg_color: Color, text_color: Color, enable_dynamic_text_color: bool) -> Self {
        Self {
            model: None,
            color_method: "Simple Color".to_string(),
            transition: ColorTransition {
                target_color: bg_color,
                color: bg_color,
                mix_factor: STARTING_MIX_FACTOR,
                transitioning: false,
            },
            text_color: text_color,
            dynamic_text_color: enable_dynamic_text_color,
        }
    }

    pub fn get_options() -> HashMap<String, Vec<String>> {
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


    pub fn update_text_color(&mut self) {
        if self.dynamic_text_color {
            self.text_color = get_optimal_text_color(&self.transition.color);
        } else {
            self.text_color = get_optimal_text_color(&self.transition.target_color);
        }
    }
}


impl EmoVisualizer for ColorTransitionVisualizer {
    fn get_enum(&self) -> VisualizerSelection {
        VisualizerSelection::ColorTransition
    }


    fn reset(&mut self, bg_color: Color, text_color: Color, enable_dynamic_text_color: bool) {
        self.model = None;
        self.transition.color = bg_color;
        self.transition.target_color = bg_color;
        self.transition.mix_factor = STARTING_MIX_FACTOR;
        self.transition.transitioning = false;
        self.text_color = text_color;
        self.dynamic_text_color = enable_dynamic_text_color;
    }

    fn gracefully_reset(
        &mut self,
        bg_color: Color,
        _text_color: Color,
        _enable_dynamic_text_color: bool,
    ) {
        self.transition.target_color = bg_color;
    }

    fn update_model(&mut self, analysis: &EmocatTextAnalysis) {
        let model = TopEmotionsModel::from_analysis(&analysis);
        match self.color_method.as_str() {
            "Simple Color" => self.transition.target_color = model.get_simple_color(),
            "Black, White, Gray" => self.transition.target_color = model.get_black_or_white(),
            "Grayscale" => self.transition.target_color = model.get_grayscale(),
            _ => {}
        }
        self.model = Some(model);
    }

    fn update_visualization(&mut self) {
        self.transition.step();
        self.update_text_color();
    }


    fn get_text_color(&self) -> Color {
        self.text_color
    }

    fn draw(&mut self, _app: &mut App, _gfx: &mut Graphics, draw: &mut Draw) {
        // The following call to clear() is important when rendering draw & egui output together.
        draw.clear(self.transition.color);
    }
}
