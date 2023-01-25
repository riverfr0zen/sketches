use super::EmoVisualizer;
use crate::emotion::{ColorMapping, EmoColor, EmocatTextAnalysis, TopEmotionsModel};
use notan::draw::*;
use notan::prelude::*;
use palette::{FromColor, LinSrgb, Mix, Srgb};
use std::collections::HashMap;


/// Represents different Tile "baselines/archetypes"
///
/// For now just contains an EmoColor, but anticipating expansion
struct Tile {
    emocolor: EmoColor,
}


impl Tile {
    fn new(emocolor: &EmoColor) -> Self {
        Self {
            emocolor: EmoColor {
                emotion: emocolor.emotion.clone(),
                hsv: emocolor.hsv,
                sentiment: emocolor.sentiment.clone(),
            },
        }
    }
}


pub struct TileVisualizer {
    pub model: Option<TopEmotionsModel>,
    bg_color: Color,
    text_color: Color,
    dynamic_text_color: bool,
    tiles: Vec<Tile>,
}


impl TileVisualizer {
    pub fn new(bg_color: Color, text_color: Color, enable_dynamic_text_color: bool) -> Self {
        Self {
            model: None,
            bg_color: bg_color,
            text_color: text_color,
            dynamic_text_color: enable_dynamic_text_color,
            tiles: vec![],
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


impl EmoVisualizer for TileVisualizer {
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

    fn update_model(&mut self, analysis: &EmocatTextAnalysis) {
        let model = TopEmotionsModel::from_analysis(&analysis);
        let top_emocolors = model.get_top_emocolors(&ColorMapping::PLUTCHIK);
        self.tiles = top_emocolors
            .iter()
            .map(|emocolor| Tile::new(emocolor))
            .collect();
        self.model = Some(model);
    }

    fn update_visualization(&mut self) {
        // self.update_bg_color();
        // self.update_text_color();
    }


    fn draw(&self, draw: &mut Draw) {
        // The following call to clear() is important when rendering draw & egui output together.
        draw.clear(self.bg_color);

        for (index, tile) in self.tiles.iter().enumerate() {
            // let mut fill_color = tile.emocolor.hsv.clone();
            let srgb = Srgb::from_color(tile.emocolor.hsv);
            let fill_color = Color::from_rgb(srgb.red, srgb.green, srgb.blue);
            draw.rect((100.0 * (index + 1) as f32, 100.0), (300.0, 200.0))
                .fill_color(fill_color);
        }
    }
}
