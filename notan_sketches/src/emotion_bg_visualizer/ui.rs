use super::visualizers::{ColorTransitionVisualizer, EmoVisualizer};
use notan::egui::{self, RichText, TextStyle, Ui};


pub trait DisplayMetrics {
    fn egui_metrics(&self, ui: &mut Ui, title_style: &dyn Fn() -> TextStyle);
}


impl DisplayMetrics for ColorTransitionVisualizer {
    fn egui_metrics(&self, ui: &mut Ui, title_style: &dyn Fn() -> TextStyle) {
        if let Some(model) = &self.model {
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
        // ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
        //     ui.label("Something Else:");
        //     ui.label(&self.options["Color Method"]);
        // });
    }
}
