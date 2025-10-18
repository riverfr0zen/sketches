# Emotion Visualization System

The emotion system (`src/emotion.rs`) processes text emotion analysis data and maps emotions to colors:

- **EmocatTextAnalysis**: Deserializes emotion analysis JSON from external tools
- **TopEmotionsModel**: Extracts dominant emotions from analysis data
- **Color Mappings**: Two mapping systems available:
  - Plutchik (default): Based on Plutchik's wheel of emotions
  - Therapy: Based on art therapy color associations
- **Methods**: `get_simple_color()`, `get_black_or_white()`, `get_grayscale()`

Used in examples like `emo_bg_visualizer.rs` and `emo_proto_visualizer.rs`.
