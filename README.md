pa-visualizer
=============

A collection of pulseaudio visualizers

## Usage ##
Set (using `pavucontrol` or `pactl`) the default source to the monitor of
your speakers and start the visualizer with

```terminal
cargo run --release --bin pa-visualizer-<visualizer-name>
```

## Visualizers ##

### ascii ###
A simple debug visualizer, printing the left and right spectra to the terminal window

### python ###
A wrapper to write visualizers in python.

### sfml-simple ###
![sfml-simple](img/sfml-simple.png)
A basic visualizer made with sfml

### sfml-wireline ###
![sfml-wireline](img/sfml-wireline.png)
A visualizer made using sfml with a wireframe style

### sfml-wirecircle ###
![sfml-wirecircle](img/sfml-wirecircle.png)
Same as wireline, just "wrapping" it around a circle

### noambition ###
![noambition](img/noambition.png)
A 3d visualizer, written using glium based on the [Demo "No Ambition" by Quite & T-Rex](http://www.pouet.net/prod.php?which=69730)

### noambition2 ###
![noambition](img/noambition2.png)
Taking the noambition visualizer to the next level ... Thanks to offdroid, for help with implementing this ...

## Shaders used ###
* [Gaussian Blur shader](https://www.shadertoy.com/view/XdfGDH) by mrhaircot
* [Bokeh disc shader](https://www.shadertoy.com/view/4d2Xzw) by David Hoskins
* [Chromatic Abberation shader](https://github.com/spite/Wagner/blob/master/fragment-shaders/chromatic-aberration-fs.glsl)

## License ##
pa-visualizer licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
