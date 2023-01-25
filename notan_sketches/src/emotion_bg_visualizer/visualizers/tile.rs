use super::EmoVisualizer;
use crate::emotion::{ColorMapping, EmoColor, EmocatTextAnalysis, Sentiment, TopEmotionsModel};
use crate::utils::get_rng;
use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use palette::{FromColor, LinSrgb, Mix, Shade, Srgb};
use std::collections::HashMap;

/// For modifies the sentiment score to be used as a value for HSV
const VALUE_MODIFIER: f32 = 3.0;


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


pub struct TilesLayout {
    tile_size: Vec2,
    rows: i32,
    cols: i32,
}

impl TilesLayout {
    fn none() -> Self {
        Self {
            tile_size: vec2(0.0, 0.0),
            rows: 0,
            cols: 0,
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
    layout: TilesLayout,
    refresh_layout: bool,
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
            layout: TilesLayout::none(),
            refresh_layout: false,
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

    fn draw_flurry(&mut self, draw: &mut Draw) {
        if self.tiles.len() < 1 {
            return;
        }

        let num_cols: f32;
        let num_rows: f32;
        if self.refresh_layout {
            num_cols = self.rng.gen_range(self.tiles.len()..10) as f32;
            num_rows = self.rng.gen_range(1..10) as f32;
            self.layout.tile_size = vec2(draw.width() / num_cols, draw.height() / num_rows);
            self.refresh_layout = false;
        } else {
            num_cols = draw.width() / self.layout.tile_size.x; // Not sure why this doesn't seem to need a ceil()
            num_rows = (draw.height() / self.layout.tile_size.y).ceil();
        }
        log::debug!(
            "cols: {}, rows: {}, rowsc: {}",
            num_cols,
            num_rows,
            num_rows.ceil()
        );

        for row in 0..num_rows as i32 {
            for col in 0..num_cols as i32 {
                let lucky_tile;
                if self.tiles.len() > 1 {
                    lucky_tile = self.rng.gen_range(0..self.tiles.len());
                } else {
                    lucky_tile = 0;
                }
                let mut hsv_color = self.tiles[lucky_tile].emocolor.hsv.clone();
                hsv_color =
                    match self.tiles[lucky_tile].emocolor.sentiment {
                        Sentiment::POSITIVE => hsv_color.lighten(self.rng.gen_range(
                            0.0..(self.model.as_ref().unwrap().positive * VALUE_MODIFIER),
                        )),
                        Sentiment::NEGATIVE => hsv_color.darken(self.rng.gen_range(
                            0.0..(self.model.as_ref().unwrap().negative * VALUE_MODIFIER),
                        )),
                        _ => hsv_color,
                    };
                let srgb = Srgb::from_color(hsv_color);
                // let fill_color = Color::from_rgb(srgb.red, srgb.green, srgb.blue);
                let fill_color = Color::from_rgb(srgb.red, srgb.green, srgb.blue);

                draw.rect(
                    (
                        col as f32 * self.layout.tile_size.x,
                        row as f32 * self.layout.tile_size.y,
                    ),
                    (self.layout.tile_size.x, self.layout.tile_size.y),
                )
                // .alpha_mode(BlendMode::OVER)
                // .alpha(0.8)
                .fill_color(fill_color)
                .fill();
            }
        }
    }
}


impl EmoVisualizer for TileVisualizer {
    fn reset(&mut self, bg_color: Color, text_color: Color, enable_dynamic_text_color: bool) {
        self.model = None;
        self.bg_color = bg_color;
        self.text_color = text_color;
        self.dynamic_text_color = enable_dynamic_text_color;
        self.tiles = vec![];
        self.layout = TilesLayout::none();
        self.refresh_layout = false;
    }

    fn gracefully_reset(
        &mut self,
        _bg_color: Color,
        _text_color: Color,
        _enable_dynamic_text_color: bool,
    ) {
        self.tiles = vec![];
        self.layout = TilesLayout::none();
        self.refresh_layout = false;
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
        self.refresh_layout = true;
        self.model = Some(model);
    }

    fn update_visualization(&mut self) {
        // self.update_bg_color();
        // self.update_text_color();
    }


    fn draw(&mut self, draw: &mut Draw) {
        // The following call to clear() is important when rendering draw & egui output together.
        draw.clear(self.bg_color);
        self.draw_flurry(draw);
    }
}
