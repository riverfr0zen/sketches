initial


# Build commands

```
cargo build --release --example eg_notan --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/eg_notan.wasm

cargo build --release --example eg_stretch_resize --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/eg_stretch_resize.wasm

cargo build --release --example eg_aspect_fit --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/eg_aspect_fit.wasm

cargo build --release --example sierpinski_gasket --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/sierpinski_gasket.wasm

cargo build --release --example sierpinski_gasket_bushy --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/sierpinski_gasket_bushy.wasm

cargo build --release --example emo_simple_color --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/emo_simple_color.wasm

cargo build --release --example emo_bg_visualizer --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/emo_bg_visualizer.wasm

cargo build --release --example schotter --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/schotter.wasm

cargo build --release --example schotter_solid_v1 --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/schotter_solid_v1.wasm

cargo build --release --example schotter_solid_v2 --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/schotter_solid_v2.wasm

cargo build --release --example schotter_anim --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/schotter_anim.wasm

cargo build --release --example radial_pointillist --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/radial_pointillist.wasm

cargo build --release --example eg_frag_shader --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/eg_frag_shader.wasm


```

# wasm-pack command that doesn't work

Probably doesn't work because it's an example.

```
wasm-pack build --out-name eg_notan --out-dir www/wasms  --target web --release --example eg_notan
```


# Text to emotion visualization models

## Top Emotions Model (previously called "SimpleColor")

The model lists "top emotions": emotions sharing the highest score in the text analysis. The sentiment (positive/negative) scores from the analysis are also available.

### Model methods

#### Simple color method

This method will return a color calculated as below:

If no emotion has a score, a neutral gray color will be returned.

If only one emotion has the high score, the color corresponding to the emotion in one of the mappings (see "Emotion to Color Mappings" below) will be returned. 

If there are multiple emotions in the high score, the top emotions will be further grouped into "positive",  "negative", and "neutral" emotions. Following this, starting with a neutral gray HSV value, each emotion's color (based on the color mappings) will be "mixed" in. The factor by which one color mixes into the total will be based on the analysis value of the sentiment (positive/negative) that the color corresponds to. For example, if the emotion is "joy", which is a positive emotion, then the value of "positive" from the analysis will be the factor. If the emotion is neutral, the factor will be the dominant sentiment value.

#### Black or white method

This simple toy method will return black if the dominant sentiment of the text is positive, or white if it is negative. If the sentiments are equal, gray will be returned.


## Multi Hue Color Model
(TODO: update name above and contents below to match learnings from implementing Top Emotions Model. This is still a separate model, but its main difference will be how the top emotions are grouped up. The model should be named after said grouping. The hues calculations and returns described will be part of some new method of this model which is yet to be articulated).

Similar to Top Emotions Model above, except that if more than one emotion has the high score:

* First, top emotions that are adjacent on the color wheel (in the same quadrant) will be grouped. 
===probably the separator between model & method===
* For each group, the avg hue will be calculated.
* The model will return a list of the averaged hues.

This model may also do the same with a "second ranked color" if only one emotion is at the top.


## Emotion to Color Mappings 
I want to experiment with two different mappings, based on different theories, so switching the used mapping is an important capability.

The first mapping is based on the theories of psychologist Dr. Robert Plutchik as described in [this article](http://shelleycrick.com/how-color-affects-emotions/). I'll call it the Plutchik mapping.


The second mapping will come from the "art therapy" world, as described in [this article](http://www.arttherapyblog.com/online/color-meanings-symbolism):


## FontForge

Since notan's text support doesn't support line-spacing yet, I found an alternative in changing the spacing within the font file itself using the excellent [FontForge](https://fontforge.org/en-US/) software. 

[This doc was helpful for line-spacing](http://designwithfontforge.com/en-US/Line_Spacing.html).