# Rusty Image

[![Build Status](https://travis-ci.org/rudsvar/rusty-image.svg?branch=master)](https://travis-ci.org/rudsvar/rusty-image)

A program written in Rust that converts an image to colored ASCII text.

## Usage

Provide a filename and the scale, where 0.01 would print every hundredth pixel, 0.1 would print every tenth, and so on.
Using the --release flag makes reading the image a lot faster.

Running it with cargo run

```bash
cargo run --release path/to/file.png 0.01
```

Compiling with cargo, then running the binary

```bash
cargo build --release
./target/release/rusty-image path/to/file.png 0.05
```

I recommend starting with values in the range 0.01 to 0.1
