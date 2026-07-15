# beg2dither

A TUI for image dithering with live preview.<br>

![Screenshot](https://raw.githubusercontent.com/northernmarshes/beg2dither/7e0862ca65b4ad5d98e76ca0f3e26477806db679/screenshot.png)

If you do not have Rust installed follow the [instructions](https://rust-lang.org/tools/install/).<br> Once installed, enter the directory and run in release mode.

``` bash
cd beg2dither
cargo run --release
```

## Algorithms

- Floyd Steinberg
- Stucki
- Jarvis
- Atkinson
- None

## Requirements

For the list of compatible terminal emulators see [ratatui-image](https://github.com/ratatui/ratatui-image).
You are probably going to need to install [chafa](https://hpjansson.org/chafa/download/) to display the preview.

## Disclaimer

Coded by a human - all bugs carefully crafted by hand.
