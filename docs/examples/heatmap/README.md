# Creating a heatmap

This example shows how to create a heatmap from positions where players fired their weapons from.

:information_source: Uses radar images from `../_assets/radar` directory.

See [`examples/heatmap.rs`](../../examples/heatmap.rs) for the source code.

## Running the example

`cargo run --example heatmap -- -demo /path/to/demo > out.jpg`

This will create a JPEG of a radar overview with dots on all the locations where shots were fired from.

![Resulting heatmap](https://raw.githubusercontent.com/markus-wa/demoinfocs-golang/master/examples/heatmap/heatmap.jpg)
