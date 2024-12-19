pub mod ui;
pub mod visualizers;

// use crate::utils::ScreenDimensions;
use notan::app::Graphics;
use notan::math::{vec2, Vec2};
use ui::{DisplayMetrics, SettingsUi};
use visualizers::EmoVisualizer;

pub trait EmoVisualizerFull: EmoVisualizer + DisplayMetrics + SettingsUi {}

impl<T: EmoVisualizer + DisplayMetrics + SettingsUi> EmoVisualizerFull for T {}

/// In this app, where font scaling is involved, a work size that matches
/// the window size results in nicer looking fonts. This comes at the expense of
/// not being able to use literal values for sizing shapes and such (not being able
/// to work against a known scale). Instead, one can use fractions of the work size
/// values.
pub fn get_work_size(gfx: &Graphics) -> Vec2 {
    // If we don't guard against a minimum like this, the app crashes if the window
    // is shrunk to a small size.
    // if gfx.device.size().0 as f32 > ScreenDimensions::MINIMUM.x {
    //     return vec2(gfx.device.size().0 as f32, gfx.device.size().1 as f32);
    // }
    // ScreenDimensions::MINIMUM

    // I think the safeguard above is not necessary -- I think the issue was caused
    // by some other problem during development. Keeping for quick reference for now,
    // in case the problem shows up again.
    return vec2(gfx.device.size().0 as f32, gfx.device.size().1 as f32);
}
