use super::EmoVisualizer;
use crate::emotion::{ColorMapping, EmoColor, EmocatTextAnalysis, TopEmotionsModel};
use notan::draw::*;
use notan::prelude::*;
use palette::{FromColor, LinSrgb, Mix, Srgb};
use std::collections::HashMap;


pub struct SquareCollageVisualizer {
    pub model: Option<TopEmotionsModel>,
    bg_color: Color,
    text_color: Color,
    dynamic_text_color: bool,
    emocolors: Vec<EmoColor>,
}


impl SquareCollageVisualizer {
    pub fn new(bg_color: Color, text_color: Color, enable_dynamic_text_color: bool) -> Self {
        Self {
            model: None,
            bg_color: bg_color,
            text_color: text_color,
            dynamic_text_color: enable_dynamic_text_color,
            emocolors: vec![],
        }
    }

    // pub fn get_options() -> HashMap<String, Vec<String>> {
    //     let mut options = HashMap::new();
    //     options.insert(
    //         "Color Method".to_string(),
    //         vec![
    //             "Simple Color".to_string(),
    //             "Black, White, Gray".to_string(),
    //             "Grayscale".to_string(),
    //         ],
    //     );
    //     options
    // }
}


impl EmoVisualizer for SquareCollageVisualizer {
    fn reset(&mut self, bg_color: Color, text_color: Color, enable_dynamic_text_color: bool) {
        self.model = None;
        self.bg_color = bg_color;
        self.text_color = text_color;
        self.dynamic_text_color = enable_dynamic_text_color;
    }

    fn gracefully_reset(
        &mut self,
        _bg_color: Color,
        _text_color: Color,
        _enable_dynamic_text_color: bool,
    ) {
        // self.target_color = bg_color;
    }


    fn get_text_color(&self) -> Color {
        self.text_color
    }


    fn draw(&self, draw: &mut Draw) {
        // The following call to clear() is important when rendering draw & egui output together.
        draw.clear(self.bg_color);
        draw.rect((100.0, 100.0), (300.0, 200.0))
            .fill_color(Color::RED);
    }

    fn update_model(&mut self, analysis: &EmocatTextAnalysis) {
        let model = TopEmotionsModel::from_analysis(&analysis);
        self.emocolors = model.get_top_emocolors(&ColorMapping::PLUTCHIK);
        // self.bg_color = model.get_simple_color();
        self.model = Some(model);
    }

    fn update_visualization(&mut self) {
        // self.update_bg_color();
        // self.update_text_color();
    }
}
