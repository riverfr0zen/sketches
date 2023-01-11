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


```

# wasm-pack command that doesn't work

Probably doesn't work because it's an example.

```
wasm-pack build --out-name eg_notan --out-dir www/wasms  --target web --release --example eg_notan
```


# Text to emotion visualization models

## Simple Color Model

The model will return a color calculated as below:

The color model will select the emotion(s) with the highest score in the text analysis.

If no emotion has a score, a neutral gray color will be returned.

If only one emotion has the high score, the color corresponding to the emotion in one of the mappings (see "Mappings" below) will be returned. 

If there are multiple emotions in the high score, the top emotions will be further grouped into "positive",  "negative", and "neutral" emotions. Following this, starting with a neutral gray HSV value, each emotion's color (based on the color mappings) will be "mixed" in. The factor by which one color mixes into the total will be based on the analysis value of the sentiment (positive/negative) that the color corresponds to. For example, if the emotion is "joy", which is a positive emotion, then the value of "positive" from the analysis will be the factor. If the color is neutral, the factor will be dominant sentiment value.


### Mappings 
I want to experiment with two different mappings, based on different theories, so switching the used mapping is an important capability.

The first mapping is based on the theories of psychologist Dr. Robert Plutchik as described in [this article](http://shelleycrick.com/how-color-affects-emotions/). I'll call it the Plutchik mapping.


The second mapping will come from the "art therapy" world, as described in [this article](http://www.arttherapyblog.com/online/color-meanings-symbolism):


## Multi Hue Color Model

Similar to Simple Color Model above, except that if more than one emotion has the high score:

* First, top emotions that are adjacent on the color wheel (in the same quadrant) will be grouped. 
* For each group, the avg hue will be calculated.
* The model will return a list of the averaged hues.

This model may also do the same with a "second ranked color" if only one emotion is at the top.


## FontForge

Since notan's text support doesn't support line-spacing yet, I found an alternative in changing the spacing within the font file itself using the excellent [FontForge](https://fontforge.org/en-US/) software. 

[This doc was helpful for line-spacing](http://designwithfontforge.com/en-US/Line_Spacing.html).