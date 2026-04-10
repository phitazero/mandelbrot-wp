A program to generate a cool images by randomly zooming into the Mandelbrot set or a random Julia set.

Originally intended as a wallpaper generator.

# Feature~s~

- Allows to configure the color scheme, i. e.:
    - Fill color of the set itself
    - Gradient used to color the surrounding points (colors)
    - Gradient interpolation (linear, cubic smooth step)
    - Color space to interpotate in (RGB, CIELAB)

# Usage

Typical [clap](https://docs.rs/clap/latest/clap/) CLI.

`mandelbrot-wp help [COMMAND]` - get help - general or for a specific command

`mandelbrot-wp <FILE> generate [OPTIONS]` to generate an image

`mandelbrot-wp <FILE> gradient [OPTIONS]` to preview a gradient in with different interpolations and color spaces

# Installation
Git clone, cd, `cargo build --release`, move `target/release/mandelbrot-wp` to PATH.
