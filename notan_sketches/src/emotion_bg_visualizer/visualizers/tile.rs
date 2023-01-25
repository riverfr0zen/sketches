use super::EmoVisualizer;
use crate::emotion::{ColorMapping, EmoColor, EmocatTextAnalysis, TopEmotionsModel};
use crate::utils::get_rng;
use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
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
    rng: Random,
    pub model: Option<TopEmotionsModel>,
    bg_color: Color,
    text_color: Color,
    dynamic_text_color: bool,
    tiles: Vec<Tile>,
    tile_size: Vec2,
    refresh_tile_size: bool,
}


impl TileVisualizer {
    pub fn new(bg_color: Color, text_color: Color, enable_dynamic_text_color: bool) -> Self {
        let (rng, _) = get_rng(None);
        Self {
            rng: rng,
            model: None,
            bg_color: bg_color,
            text_color: text_color,
            dynamic_text_color: enable_dynamic_text_color,
            tiles: vec![],
            tile_size: vec2(0.0, 0.0),
            refresh_tile_size: false,
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
        self.tiles = vec![];
        self.tile_size = vec2(0.0, 0.0);
        self.refresh_tile_size = false;
    }

    fn gracefully_reset(
        &mut self,
        _bg_color: Color,
        _text_color: Color,
        _enable_dynamic_text_color: bool,
    ) {
        self.tiles = vec![];
        self.tile_size = vec2(0.0, 0.0);
        self.refresh_tile_size = false;
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
        self.refresh_tile_size = true;
        self.model = Some(model);
    }

    fn update_visualization(&mut self) {
        // self.update_bg_color();
        // self.update_text_color();
    }

    fn draw(&mut self, draw: &mut Draw) {
        // The following call to clear() is important when rendering draw & egui output together.
        draw.clear(self.bg_color);

        if self.tiles.len() < 1 {
            return;
        }

        let num_cols: f32;
        let num_rows: f32;
        if self.refresh_tile_size {
            num_cols = self.rng.gen_range(self.tiles.len()..10) as f32;
            num_rows = self.rng.gen_range(1..10) as f32;
            self.tile_size = vec2(draw.width() / num_cols, draw.height() / num_rows);
            self.refresh_tile_size = false;
        } else {
            num_cols = draw.width() / self.tile_size.x;
            num_rows = draw.height() / self.tile_size.y;
        }

        for row in 0..num_rows as i32 {
            for col in 0..num_cols as i32 {
                let lucky_tile;
                if self.tiles.len() > 1 {
                    lucky_tile = self.rng.gen_range(0..self.tiles.len());
                } else {
                    lucky_tile = 0;
                }
                // let mut fill_color = tile.emocolor.hsv.clone();
                let srgb = Srgb::from_color(self.tiles[lucky_tile].emocolor.hsv);
                let fill_color = Color::from_rgb(srgb.red, srgb.green, srgb.blue);

                draw.rect(
                    (col as f32 * self.tile_size.x, row as f32 * self.tile_size.y),
                    (self.tile_size.x, self.tile_size.y),
                )
                .fill_color(fill_color)
                .fill()
                .stroke_color(Color::BLACK)
                .stroke(1.0);
            }
        }
        // for (index, tile) in self.tiles.iter().enumerate() {
        //     // let mut fill_color = tile.emocolor.hsv.clone();
        //     let srgb = Srgb::from_color(tile.emocolor.hsv);
        //     let fill_color = Color::from_rgb(srgb.red, srgb.green, srgb.blue);
        //     draw.rect((100.0 * (index + 1) as f32, 100.0), (300.0, 200.0))
        //         .fill_color(fill_color);
        // }
    }
}
