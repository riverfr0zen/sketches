use notan::log;
use notan::prelude::*;
use palette::{FromColor, Hsl, Hsv, LinSrgb, Mix, RgbHue, Srgb};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct EmocatMeta {
    pub paragraph: u8,
    pub from_line: u8,
    pub to_line: u8,
    pub file: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmocatAnalyzerScore {
    pub marker: String,
    pub score: f32,
}


// #[derive(Serialize, Deserialize, Debug)]
// struct EmocatAnalyzerResult {
//     fear: f32,
//     anger: f32,
//     anticipation: f32,
//     trust: f32,
//     surprise: f32,
//     positive: f32,
//     negative: f32,
//     sadness: f32,
//     disgust: f32,
//     joy: f32,
// }


#[derive(Serialize, Deserialize, Debug)]
pub struct EmocatAnalyzerResults {
    pub nrclex: Vec<EmocatAnalyzerScore>,
    pub t2e_repo: Vec<EmocatAnalyzerScore>,
    pub t2e_demo: Vec<EmocatAnalyzerScore>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct EmocatTextAnalysis {
    pub text: String,
    pub meta: EmocatMeta,
    pub results: EmocatAnalyzerResults,
}


#[derive(Serialize, Deserialize, Debug)]
/// Represents an `emocat` output document
pub struct EmocatOutputDoc {
    pub title: String,
    pub author: String,
    pub analyses: Vec<EmocatTextAnalysis>,
}


pub struct EmoColor {
    pub emotion: String,
    pub sentiment: Sentiment,
    pub hsv: Hsv,
}


pub enum Sentiment {
    POSITIVE,
    NEGATIVE,
    NEUTRAL,
}


pub fn get_emotion_sentiment(emotion: &str) -> Sentiment {
    match emotion {
        "fear" => Sentiment::NEGATIVE,
        "anger" => Sentiment::NEGATIVE,
        "anticipation" => Sentiment::NEUTRAL,
        "trust" => Sentiment::POSITIVE,
        "surprise" => Sentiment::NEUTRAL,
        "sadness" => Sentiment::NEGATIVE,
        "disgust" => Sentiment::NEGATIVE,
        "joy" => Sentiment::POSITIVE,
        _ => Sentiment::NEUTRAL,
    }
}


/// Returns color mapped to the emotion provided based on Plutchik color wheel here:
/// http://shelleycrick.com/how-color-affects-emotions/
pub fn get_mapped_color_plutchik(emotion: &str) -> Hsv {
    match emotion {
        "fear" => Hsv::new(RgbHue::from_degrees(88.0), 1.0, 0.59),
        "anger" => Hsv::new(RgbHue::from_degrees(350.0), 1.0, 0.72),
        "anticipation" => Hsv::new(RgbHue::from_degrees(21.0), 1.0, 0.96),
        "trust" => Hsv::new(RgbHue::from_degrees(69.0), 1.0, 0.72),
        "surprise" => Hsv::new(RgbHue::from_degrees(136.0), 0.98, 0.50),
        "sadness" => Hsv::new(RgbHue::from_degrees(206.0), 1.0, 0.85),
        "disgust" => Hsv::new(RgbHue::from_degrees(300.0), 1.0, 0.24),
        "joy" => Hsv::new(RgbHue::from_degrees(55.0), 1.0, 0.91),
        _ => Hsv::new(RgbHue::from_degrees(180.0), 0.0, 0.50),
        // _ => Hsv::new(RgbHue::from_degrees(0.0), 0.0, 0.0),
        // _ => Hsv::new(RgbHue::from_degrees(0.0), 0.0, 1.0),
    }
}


/// Returns color mapped to the emotion provided based on the art therapy color
/// associations here:
///
/// Tbh this mapping is iffy because the source is very wide-ranging and it was not
/// straightforward to map the particular emotions to the colors. Just implemented
/// "for kicks" and to solidify the design of these mappings.
///
/// http://www.arttherapyblog.com/online/color-meanings-symbolism
pub fn get_mapped_color_therapy(emotion: &str) -> Hsv {
    match emotion {
        "fear" => Hsv::new(RgbHue::from_degrees(60.0), 0.8, 1.0),
        "anger" => Hsv::new(RgbHue::from_degrees(5.0), 0.93, 1.0),
        // Loosely interpreting anticipation to be green
        "anticipation" => Hsv::new(RgbHue::from_degrees(95.0), 0.72, 0.69),
        "trust" => Hsv::new(RgbHue::from_degrees(224.0), 0.99, 1.0),
        // Loosely interpreting surprise as violet
        "surprise" => Hsv::new(RgbHue::from_degrees(286.0), 0.99, 0.69),
        "sadness" => Hsv::new(RgbHue::from_degrees(224.0), 0.99, 1.0),
        // Cannot find an equivalent, so just going to return gray
        "disgust" => Hsv::new(RgbHue::from_degrees(180.0), 0.0, 0.50),
        "joy" => Hsv::new(RgbHue::from_degrees(36.0), 0.99, 0.98),
        _ => Hsv::new(RgbHue::from_degrees(180.0), 0.0, 0.50),
        // _ => Hsv::new(RgbHue::from_degrees(0.0), 0.0, 0.0),
        // _ => Hsv::new(RgbHue::from_degrees(0.0), 0.0, 1.0),
    }
}


/// Returns colors & sentiment mapped to the emotion provided
pub fn get_mapped_emocolor(emotion: &str, mapping_func: &dyn Fn(&str) -> Hsv) -> EmoColor {
    match emotion {
        "fear" => EmoColor {
            emotion: emotion.to_string(),
            sentiment: Sentiment::NEGATIVE,
            hsv: mapping_func(emotion),
        },
        "anger" => EmoColor {
            emotion: emotion.to_string(),
            sentiment: Sentiment::NEGATIVE,
            hsv: mapping_func(emotion),
        },
        "anticipation" => EmoColor {
            emotion: emotion.to_string(),
            sentiment: Sentiment::NEUTRAL,
            hsv: mapping_func(emotion),
        },
        "trust" => EmoColor {
            emotion: emotion.to_string(),
            sentiment: Sentiment::POSITIVE,
            hsv: mapping_func(emotion),
        },
        "surprise" => EmoColor {
            emotion: emotion.to_string(),
            sentiment: Sentiment::NEUTRAL,
            hsv: mapping_func(emotion),
        },
        "sadness" => EmoColor {
            emotion: emotion.to_string(),
            sentiment: Sentiment::NEGATIVE,
            hsv: mapping_func(emotion),
        },
        "disgust" => EmoColor {
            emotion: emotion.to_string(),
            sentiment: Sentiment::NEGATIVE,
            hsv: mapping_func(emotion),
        },
        "joy" => EmoColor {
            emotion: emotion.to_string(),
            sentiment: Sentiment::POSITIVE,
            hsv: mapping_func(emotion),
        },
        _ => EmoColor {
            emotion: emotion.to_string(),
            sentiment: Sentiment::NEUTRAL,
            hsv: mapping_func(emotion),
        },
    }
}


pub struct SimpleColorModel {
    pub positive: f32,
    pub negative: f32,
    pub top_emotions: Vec<EmocatAnalyzerScore>,
}

impl SimpleColorModel {
    pub fn from_analysis(analysis: &EmocatTextAnalysis) -> Self {
        let mut scores = analysis.results.nrclex.clone();
        // log::debug!("Scores before {:?}", scores);

        let positive_pos = scores.iter().position(|s| s.marker == "positive").unwrap();
        let positive_sentiment = scores.remove(positive_pos);
        let negative_pos = scores.iter().position(|s| s.marker == "negative").unwrap();
        let negative_sentiment = scores.remove(negative_pos);
        log::debug!(
            "positive: {}, negative: {}",
            positive_sentiment.score,
            negative_sentiment.score
        );

        scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        // log::debug!("Score after {:?}", scores);

        let mut top_emotions: Vec<EmocatAnalyzerScore> = Vec::new();
        top_emotions.push(scores[0].clone());
        for score in scores.iter().skip(1) {
            if score.score == top_emotions[0].score {
                top_emotions.push(score.clone());
            }
        }
        Self {
            positive: positive_sentiment.score,
            negative: negative_sentiment.score,
            top_emotions: top_emotions,
        }
    }

    /// Simple Color Model. See README for description.
    pub fn get_simple_color(&self) -> Color {
        let top_emotions = &self.top_emotions;
        if top_emotions[0].score > 0.0 {
            log::debug!("Top emotions: {:?}:", top_emotions);
            let mapping_func = get_mapped_color_plutchik;
            let emocolors: Vec<EmoColor> = top_emotions
                .iter()
                .map(|s| get_mapped_emocolor(&s.marker, &mapping_func))
                .collect();
            // Start with a neutral gray
            if emocolors.len() > 1 {
                let mut final_color = get_mapped_emocolor("", &mapping_func).hsv;
                for emocolor in emocolors.iter() {
                    log::debug!("Before mix: {:?}", final_color);
                    let sentiment_value: f32 = match &emocolor.sentiment {
                        Sentiment::POSITIVE => self.positive,
                        Sentiment::NEGATIVE => self.negative,
                        Sentiment::NEUTRAL => self.positive.max(self.negative),
                    };
                    // The sentiment values don't often seem to go beyond 0.5, so I'm modifying the
                    // mix factor a little. Must test later with more examples of text.
                    let mix_factor = sentiment_value * 2.0;
                    log::debug!(
                        "Emotion: {}, Sentiment value: {}, Mix_factor: {}",
                        emocolor.emotion,
                        sentiment_value,
                        mix_factor
                    );
                    // final_color = final_color.mix(&emocolor.hsv, 0.5);
                    final_color = final_color.mix(&emocolor.hsv, mix_factor);
                    log::debug!("After mix: {:?}", final_color);
                }
                let color = Srgb::from_color(final_color);
                return Color::from_rgb(color.red, color.green, color.blue);
            } else {
                let color = Srgb::from_color(emocolors[0].hsv);
                return Color::from_rgb(color.red, color.green, color.blue);
            }
        }
        Color::GRAY
    }
}
