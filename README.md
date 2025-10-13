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

# Updating dependencies

1. Do automatic updates

`cargo update`

Also run cargo update on local dependencies: `../notan_touchy`

2. Test

* test sketches natively
* test sketches on wasm 

3. Check what wasn't updated manually, and try to update

`cargo update --verbose`

4. Test again


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


# FontForge

Since notan's text support doesn't support line-spacing yet, I found an alternative in changing the spacing within the font file itself using the excellent [FontForge](https://fontforge.org/en-US/) software.

[This doc was helpful for line-spacing](http://designwithfontforge.com/en-US/Line_Spacing.html).


# Outstanding Issues

## Hot reloader issue

It seems there is an issue with the shader hot reloader (e.g. try running erratic_wave_shader from the terminal). Not sure when this came up, but the best lead to fixing is probably in `notan_sketches/src/shaderutils.rs`. See the comment above the `create_hot_shape_pipeline` fn.

Known affected sketches (there may be more):

* erratic_wave_shader
* color_points_shader


## Issue with native renderer on Pixel 8a

It seems that Notan's web renderer doesn't play well with the native renderer on Pixel 8a. Notan apps are choppy and unresponsive by default on the device, and in some cases (e.g. when a number of RenderTextures are created) the apps can even crash from low memory.

The issue can be fixed by changing the renderer used by the browser to ANGLE. To do this:

On the phone, go to `Settings > System > Developer Options` (developer options have to be enabled on the phone), and find `ANGLE Preferences` settings. Tap on the setting, and find the browser app (Chrome/Brave, etc.). Tap on the app, and choose the `angle` option.

Unfortunately this is not something that can be programmatically done for the end user, so the solution is not ideal. Whether ANGLE will become default over time is a question: apparently the reason ANGLE is not the default on the Pixel 8a is because the phone's native renderer utilizes some hardware and AI techniques to provide what is considered superior renderering. But it doesn't seem to be working for apps made with Notan.

Will have to monitor this issue over time and see how it pans out, but it should be a consideration when thinking about an application's audience.
