A program to generate a cool images by randomly zooming into the Mandelbrot set or a random Julia set.

Originally intended as a wallpaper generator.

# Features

- Configure the color scheme, i. e.:
    - Fill color of the set itself
    - Gradient used to color the surrounding points (colors)
    - Gradient interpolation (linear, cubic smooth step)
    - Color space to interpotate in (RGB, CIELAB)

- Replicate and tweak already generated images

# Usage

Typical [clap](https://docs.rs/clap/latest/clap/) CLI.

`mandelbrot-wp help [COMMAND]` - get help - general or for a specific command

`mandelbrot-wp generate [OPTIONS] <FILE>` to generate an image

`mandelbrot-wp gradient [OPTIONS] <FILE>` to preview a gradient in with different interpolations and color spaces

`mandelbrot-wp log` to log previous generated images

`mandelbrot-wp [OPTIONS] <FILE> <HASH>` to replicate an image, specified with its hash (or a prefix of the hash)

# Installation
Git clone, cd, `cargo build --release`, move `target/release/mandelbrot-wp` to PATH.
