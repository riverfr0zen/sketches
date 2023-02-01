use super::color_transition::ColorTransition;
use super::get_optimal_text_color;
use super::EmoVisualizer;
use crate::emotion::{ColorMapping, EmoColor, EmocatTextAnalysis, Sentiment, TopEmotionsModel};
use crate::utils::get_rng;
use notan::draw::*;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use palette::{FromColor, LinSrgb, Mix, Shade, Srgb};
use rapier2d::na::iter::RowIterMut;
use std::collections::HashMap;

/// Slightly increases the sentiment score for use as a value to brighten/darken HSV
const VALUE_MODIFIER: f32 = 3.0;
// const TILE_ALPHA: f32 = 0.2;
// const TILE_ALPHA: f32 = 0.8;
const TILE_ALPHA: f32 = 1.0;
const MAX_COLS: usize = 10;
const MAX_ROWS: usize = 10;


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
    rows: usize,
    cols: usize,
    reprs: Vec<Vec<ColorTransition>>,
}

impl TilesLayout {
    fn none() -> Self {
        Self {
            tile_size: vec2(0.0, 0.0),
            rows: 0,
            cols: 0,
            reprs: vec![],
        }
    }
}

pub struct TileVisualizer {
    rng: Random,
    pub model: Option<TopEmotionsModel>,
    bg_color: Color,
    /// As I experiment, I want a separate property to base the optimal text color on,
    /// because `bgcolor` above may not change per analysis (i.e. it might just remain
    /// white or whatever color the visualizer was initialized with).
    ///
    /// See `update_model()` in impl for EmoVisualizer
    bg_color_for_text: Color,
    text_color: Color,
    dynamic_text_color: bool,
    tiles: Vec<Tile>,
    layout: TilesLayout,
    refresh_layout: bool,
}

fn get_sentiment_enhanced_color(
    emocolor: &EmoColor,
    rng: &mut Random,
    positive_sentiment: f32,
    negative_sentiment: f32,
) -> Color {
    let mut hsv_color = emocolor.hsv.clone();
    hsv_color = match emocolor.sentiment {
        Sentiment::POSITIVE => {
            if positive_sentiment > 0.0 {
                hsv_color.lighten(rng.gen_range(0.0..(positive_sentiment * VALUE_MODIFIER)))
            } else {
                hsv_color
            }
        }
        Sentiment::NEGATIVE => {
            if negative_sentiment > 0.0 {
                hsv_color.darken(rng.gen_range(0.0..(negative_sentiment * VALUE_MODIFIER)))
            } else {
                hsv_color
            }
        }
        _ => {
            if positive_sentiment > negative_sentiment {
                hsv_color.lighten(rng.gen_range(0.0..(positive_sentiment * VALUE_MODIFIER)))
            } else if negative_sentiment > positive_sentiment {
                hsv_color.darken(rng.gen_range(0.0..(negative_sentiment * VALUE_MODIFIER)))
            } else {
                hsv_color
            }
        }
    };
    let srgb = Srgb::from_color(hsv_color);
    Color::from_rgb(srgb.red, srgb.green, srgb.blue)
}


impl TileVisualizer {
    pub fn new(bg_color: Color, text_color: Color, enable_dynamic_text_color: bool) -> Self {
        let (rng, _) = get_rng(None);
        Self {
            rng: rng,
            model: None,
            bg_color: bg_color,
            bg_color_for_text: bg_color,
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


    pub fn update_text_color(&mut self) {
        // self.text_color = get_optimal_text_color(&self.bg_color);
        self.text_color = get_optimal_text_color(&self.bg_color_for_text);
    }


    fn prepare_layout(&mut self, draw: &mut Draw) {
        if self.refresh_layout {
            if self.tiles.len() > MAX_COLS {
                self.layout.cols = self.tiles.len();
            } else {
                self.layout.cols = self.rng.gen_range(self.tiles.len()..=MAX_COLS);
            }
            self.layout.rows = self.rng.gen_range(1..=MAX_ROWS);
            self.layout.tile_size = vec2(
                draw.width() / self.layout.cols as f32,
                draw.height() / self.layout.rows as f32,
            );
            log::debug!(
                "refreshed: rows {}, cols {}",
                self.layout.rows,
                self.layout.cols
            );

            self.layout.reprs = vec![];
            // Should look at this more closely, but unless I use inclusive ranges `..=`
            // here I get index out of bounds errors.
            for row in 0..self.layout.rows {
                self.layout.reprs.push(vec![]);
                for _ in 0..self.layout.cols {
                    self.layout.reprs[row].push(ColorTransition::default());
                }
            }


            self.refresh_layout = false;
        } else {
            self.layout.cols = (draw.width() / self.layout.tile_size.x).ceil() as usize;
            self.layout.rows = (draw.height() / self.layout.tile_size.y).ceil() as usize;
        }
    }

    fn draw_flurry(&mut self, draw: &mut Draw) {
        if self.tiles.len() < 1 {
            return;
        }
        self.prepare_layout(draw);

        for (row_index, row) in self.layout.reprs.iter_mut().enumerate() {
            for (col_index, col) in row.iter_mut().enumerate() {
                if col.transitioning {
                    // col.immediate();
                    col.step();
                } else {
                    let lucky_tile;
                    if self.tiles.len() > 1 {
                        lucky_tile = self.rng.gen_range(0..self.tiles.len());
                    } else {
                        lucky_tile = 0;
                    }
                    let fill_color = get_sentiment_enhanced_color(
                        &self.tiles[lucky_tile].emocolor,
                        &mut self.rng,
                        self.model.as_ref().unwrap().positive,
                        self.model.as_ref().unwrap().negative,
                    );
                    col.target_color = fill_color;
                    col.transitioning = true;
                }

                draw.rect(
                    (
                        col_index as f32 * self.layout.tile_size.x,
                        row_index as f32 * self.layout.tile_size.y,
                    ),
                    (self.layout.tile_size.x, self.layout.tile_size.y),
                )
                .alpha_mode(BlendMode::OVER)
                .alpha(TILE_ALPHA)
                .fill_color(col.color)
                .fill();
            }
        }
    }
}


impl EmoVisualizer for TileVisualizer {
    fn reset(&mut self, bg_color: Color, text_color: Color, enable_dynamic_text_color: bool) {
        self.model = None;
        self.bg_color = bg_color;
        self.bg_color_for_text = bg_color;
        self.text_color = text_color;
        self.dynamic_text_color = enable_dynamic_text_color;
        self.tiles = vec![];
        self.layout = TilesLayout::none();
        self.refresh_layout = false;
    }

    fn gracefully_reset(
        &mut self,
        bg_color: Color,
        text_color: Color,
        enable_dynamic_text_color: bool,
    ) {
        self.bg_color = bg_color;
        self.text_color = text_color;
        self.dynamic_text_color = enable_dynamic_text_color;
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
        self.bg_color = model.get_simple_color();
        self.bg_color_for_text = self.bg_color;
        // self.bg_color_for_text = model.get_simple_color();
        self.refresh_layout = true;
        self.model = Some(model);
    }

    fn update_visualization(&mut self) {
        // self.update_bg_color();
        self.update_text_color();
    }


    fn draw(&mut self, draw: &mut Draw) {
        // The following call to clear() is important when rendering draw & egui output together.
        draw.clear(self.bg_color);
        self.draw_flurry(draw);
    }
}
