use super::visualizers::color_transition::ColorTransitionVisualizer;
use super::visualizers::tile::TilesVisualizer;
use super::visualizers::tiled_shaders::TiledShadersVisualizer;
use crate::emotion::TopEmotionsModel;
use notan::egui::{self, RichText, TextStyle, Ui};


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


impl DisplayMetrics for TiledShadersVisualizer {
    fn egui_metrics(&self, ui: &mut Ui, title_style: &dyn Fn() -> TextStyle) {
        if let Some(model) = &self.model {
            top_emotions_egui_metrics_ui(model, ui, title_style);
        } else {
            ui.small("The emotion analysis metrics will appear here when you start reading.");
        }
    }
}


pub trait SettingsUi {
    fn egui_settings(&mut self, ui: &mut Ui);
}

impl SettingsUi for ColorTransitionVisualizer {
    fn egui_settings(&mut self, ui: &mut Ui) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            let viz_options = &mut Self::get_options();

            ui.label("Color Method");
            egui::ComboBox::new("color-method", "")
                .selected_text(&self.color_method)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                    for option in viz_options.get_mut("Color Method").unwrap().iter() {
                        ui.selectable_value(&mut self.color_method, option.to_string(), option);
                    }
                });
        });
    }
}


impl SettingsUi for TilesVisualizer {
    fn egui_settings(&mut self, ui: &mut Ui) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            let viz_options = &mut Self::get_options();

            ui.label("Text Shadow");
            egui::ComboBox::new("shadow-style", "")
                .selected_text(&self.text_shadow_style)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                    for option in viz_options.get_mut("Shadow Style").unwrap().iter() {
                        ui.selectable_value(
                            &mut self.text_shadow_style,
                            option.to_string(),
                            option,
                        );
                    }
                });
        });
    }
}


impl SettingsUi for TiledShadersVisualizer {
    fn egui_settings(&mut self, ui: &mut Ui) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            // ui.label("There are no options for the Tiles visualizer.");
            // ui.label(&self.options["Color Method"]);

            let viz_options = &mut Self::get_options();

            ui.label("Text Shadow");
            egui::ComboBox::new("shadow-style", "")
                .selected_text(&self.text_shadow_style)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                    for option in viz_options.get_mut("Shadow Style").unwrap().iter() {
                        ui.selectable_value(
                            &mut self.text_shadow_style,
                            option.to_string(),
                            option,
                        );
                    }
                });
        });
    }
}
