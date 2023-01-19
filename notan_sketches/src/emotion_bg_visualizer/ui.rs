use super::visualizers::ColorTransitionVisualizer;
use notan::egui::{self, RichText, TextStyle, Ui};


pub trait DisplayMetrics {
    fn egui(&self, ui: &mut Ui, title_style: &dyn Fn() -> TextStyle);
}


impl DisplayMetrics for ColorTransitionVisualizer {
    fn egui(&self, ui: &mut Ui, title_style: &dyn Fn() -> TextStyle) {
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
