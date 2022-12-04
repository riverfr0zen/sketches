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


```

# wasm-pack command that doesn't work

Probably doesn't work because it's an example.

```
wasm-pack build --out-name eg_notan --out-dir www/wasms  --target web --release --example eg_notan
```


# Text to emotion visualization models

## Simple Color Model

The color model will select the emotion(s) with the highest score in the paragraph. 

If only one emotion has the high score, the hue of the color corresponding to the emotion in one of the mappings (see "Mappings" below) will be set as `hue`. 

If more than one emotion has the high score:
* Simple:
    * The avg value of the hues of each emotion will be set as `hue`
* Advanced:
    * First, emotions that are adjacent on the color wheel (in the same quadrant) will be grouped, 
    * For each group, the avg hue will be calculated.
    * The `hue` will be set to a list of the averaged hues.

The sentiment score (positive / negative) of the paragraph will affect the `value`. The value will initially be set in the middle of the range. The positive score will add to the value, whereas the negative score will subtract from the value. 

The sentiment scores will also affect the `saturation`. However, only the dominant score will be used, and the `saturation` will be set to the value of that score. Maybe the value of the `hue` can also factor in to `saturation`.

The model should return the HSV properties based on the calculations above. In cases where there are multiple hues returned (advanced case above, which maybe should be extracted to a new model), a list of corresponding HSVs should be returned.


### Mappings 
I want to experiment with two different mappings, based on different theories, so switching the used mapping is an important capability.

The first mapping is based on the theories of psychologist Dr. Robert Plutchik as described in [this article](http://shelleycrick.com/how-color-affects-emotions/). I'll call it the Plutchik mapping:


```
Fear: Yellow
Anger: Red
Anticipation: Orange
Trust:
Surprise:
Sadness:
Disgust:
Joy:

```

The second model will come from the "art therapy" world, as described in [this article](http://www.arttherapyblog.com/online/color-meanings-symbolism/#.Y4t04NLMK0o):

```

```




