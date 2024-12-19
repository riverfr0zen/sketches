initial

# Local configuration

Local configuration can be added to `notan_sketches/.cargo/config.toml`.

For example, I have added it on my weak laptop to limit the number of jobs.

I could have done this in `$CARGO_HOME/config.toml` too, but I didn't want to forget about it, and this seemed more visible. Might change that in the future.

See: https://doc.rust-lang.org/cargo/reference/config.html

For this repo, `.cargo/config.toml`` is ignored in `sketches/.gitignore`


# IMPORTANT: notes on updating Notan

When updating `notan` to a new version, make sure to upgrade across all notan-using packages (notan_sketches AND notan_touchy)

# Building

## NOTE: wasm-bindgen version mismatch error

If you see an error like this:

```
it looks like the Rust project used to create this wasm file was linked against
version of wasm-bindgen that uses a different bindgen format than this binary:

  rust wasm file schema version: 0.2.95
     this binary schema version: 0.2.92
```

you should update wasm-bindgen-cli to the latest version. Typically you can do this with:

`cargo install wasm-bindgen-cli`


## Build commands

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

cargo build --release --example erratic_wave_shader --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/erratic_wave_shader.wasm


```

## wasm-pack command that doesn't work

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


## Hot reloader issue

It seems there is an issue with the shader hot reloader (e.g. try running erratic_wave_shader from the terminal). Not sure when this came up, but the best lead to fixing is probably in `notan_sketches/src/shaderutils.rs`. See the comment above the `create_hot_shape_pipeline` fn.


## ShaderRenderTexture memory issue on mobile

While working on mobile fixes for emo_bg_visualizer, I found that creating too many ShaderRenderTexture (and hence RenderTexture) instances can cause the program to crash. This will probably happen on any platform when too many are created, but the limitation is especially pronounced on mobile.

For now, I have done a quick workaround that reduces the number of tiles (and hence SRT instances) for mobile.

Need to look into how to address the issue in the long-run, since there may be other sketches that face this issue too. So far I have thought of two possibilities:

1. Reduce the size (dimensions) of the RenderTextures. This is not so desirable, for e.g. in the TiledShader visualizer, where if there is only 1 tile, then you'd want a render texture with the max screen dimensions. Maybe some smarter logic in managing the rts (e.g. only create a limited amount of larger RTs when necessary, and use these larger RTs only when the layout calls for it).

2. Could it be possible to re-use the RTs? So instead of multiple ShaderRenderTexture instances, maybe there could be one that is re-used. I'm not sure if this is possible to due while still having different uniform values for each tile.
