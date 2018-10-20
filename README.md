# rusty-image
A program written in Rust that converts an image to colored ASCII text.

## Building
To build it, simply run the following commands.
The --release flag is recommended, as it makes the program a lot faster.

```bash
cargo build --release
```

The binary should then be found in target/debug/rusty-image or target/release/rusty-image depending on the flag used to build it.

## Usage

Either run the binary as mentioned earlier, or use cargo run.
Provide a filename and the scale, where 0.01 would mean print every hundredth pixel.

Example:
```bash
cargo run --release path/to/file.png 0.01
```

Or:
```bash
cargo build --release
./target/release/rusty-image path/to/file.png 0.05
```

I recommend starting with values in the range 0.01 to 0.1
