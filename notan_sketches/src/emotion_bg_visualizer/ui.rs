use super::visualizers::color_transition::ColorTransitionVisualizer;
use super::visualizers::tile::TilesVisualizer;
use crate::emotion::TopEmotionsModel;
use crate::utils::ScreenDimensions;
use notan::egui::{self, RichText, TextStyle, Ui};
use notan::math::Vec2;


/// Scale the font according to the current work size. Quite simple right now,
/// probably lots of room for improving this.
///
/// These return values were decided by comparing sizes on my own setup. Needs testing
/// across devices.
///
/// @TODO: What about portrait dimensions?
pub fn scale_font(default_size: f32, work_size: Vec2) -> f32 {
    if work_size.x >= ScreenDimensions::RES_QHD.x && work_size.x < ScreenDimensions::RES_720p.x {
        // log::debug!("720p, x:{} y:{}", work_size.x, work_size.y);
        return default_size * 1.5;
    }
    if work_size.x >= ScreenDimensions::RES_720p.x && work_size.x < ScreenDimensions::RES_HDPLUS.x {
        // log::debug!("720p, x:{} y:{}", work_size.x, work_size.y);
        return default_size * 1.75;
    }
    if work_size.x >= ScreenDimensions::RES_HDPLUS.x && work_size.x < ScreenDimensions::RES_1080P.x
    {
        // log::debug!("HDPLus, x:{} y:{}", work_size.x, work_size.y);
        return default_size * 2.2;
    }
    if work_size.x >= ScreenDimensions::RES_1080P.x && work_size.x < ScreenDimensions::RES_1440P.x {
        // log::debug!("1080p, x:{} y:{}", work_size.x, work_size.y);
        return default_size * 2.5;
    }
    if work_size.x >= ScreenDimensions::RES_1440P.x && work_size.x < ScreenDimensions::RES_4K.x {
        // log::debug!("1440p, x:{} y:{}", work_size.x, work_size.y);
        return default_size * 3.0;
    }
    if work_size.x >= ScreenDimensions::RES_4K.x {
        // log::debug!("4k, x:{} y:{}", work_size.x, work_size.y);
        return default_size * 4.5;
    }
    // log::debug!("Default, x:{} y:{}", work_size.x, work_size.y);
    return default_size;
}


fn top_emotions_egui_metrics_ui(
    model: &TopEmotionsModel,
    ui: &mut Ui,
    title_style: &dyn Fn() -> TextStyle,
) {
    ui.label("");
    let header = RichText::new("Sentiment scores:")
        .color(egui::Color32::BLACK)
        .text_style(title_style());
    ui.label(header);
    ui.small(format!("positive: {}", model.positive));
    ui.small(format!("negative: {}", model.negative));
    ui.label("");
    let header = RichText::new("Top emotions:")
        .color(egui::Color32::BLACK)
        .text_style(title_style());
    ui.label(header);
    if model.top_emotions.len() > 0 && model.top_emotions[0].score > 0.0 {
        for top_emo in model.top_emotions.iter() {
            ui.small(format!("{}: {}", top_emo.marker, top_emo.score));
        }
    } else {
        ui.small("None");
    }
}

pub trait DisplayMetrics {
    fn egui_metrics(&self, ui: &mut Ui, title_style: &dyn Fn() -> TextStyle);
}


impl DisplayMetrics for ColorTransitionVisualizer {
    fn egui_metrics(&self, ui: &mut Ui, title_style: &dyn Fn() -> TextStyle) {
        if let Some(model) = &self.model {
            top_emotions_egui_metrics_ui(model, ui, title_style);
        } else {
            ui.small("The emotion analysis metrics will appear here when you start reading.");
        }
    }
}


impl DisplayMetrics for TilesVisualizer {
    fn egui_metrics(&self, ui: &mut Ui, title_style: &dyn Fn() -> TextStyle) {
        if let Some(model) = &self.model {
            top_emotions_egui_metrics_ui(model, ui, title_style);
        } else {
            ui.small("The emotion analysis metrics will appear here when you start reading.");
        }
    }
}


pub trait SettingsUi {
    fn egui_settings(&mut self, ui: &mut Ui, option_style: &dyn Fn() -> TextStyle);
}

impl SettingsUi for ColorTransitionVisualizer {
    fn egui_settings(&mut self, ui: &mut Ui, option_style: &dyn Fn() -> TextStyle) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            let viz_options = &mut Self::get_options();

            ui.label("Color Method");
            let option_text = RichText::new(&self.color_method).text_style(option_style());
            egui::ComboBox::from_id_source("color-method")
                .selected_text(option_text)
                .show_ui(ui, |ui| {
                    for option in viz_options.get_mut("Color Method").unwrap().iter() {
                        let option_text = RichText::new(option).text_style(option_style());
                        ui.selectable_value(
                            &mut self.color_method,
                            option.to_string(),
                            option_text,
                        );
                    }
                });
        });
    }
}


impl SettingsUi for TilesVisualizer {
    fn egui_settings(&mut self, ui: &mut Ui, _option_style: &dyn Fn() -> TextStyle) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.label("There are no options for the Tiles visualizer.");
            // ui.label(&self.options["Color Method"]);
        });
    }
}
