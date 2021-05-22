# Ada

[![Build Status](https://travis-ci.com/deep110/ada.svg?branch=master)](https://travis-ci.com/deep110/ada)
[![Crates.io](https://img.shields.io/crates/v/ada.svg)](https://crates.io/crates/ada)
[![docs.rs](https://docs.rs/ada/badge.svg)](https://docs.rs/ada)

A 2D Shapes pixel rendering library in rust. Supported shapes are:
* Line2D
* Rectangle2D
* Ellipse2D
* Polygon2D
* Bezier2D [Both quadratic and cubic]

No use of unsafe blocks. `#![forbid(unsafe_code)]` is also declared at crate level.

## Usage

Add this to `Cargo.toml` file:
```toml
[dependencies]
ada = "0.2.0"
```

Example code:

```rust
use ada::{shape, Canvas};

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

// create a pixel buffer for RGBA values
let mut buffer = vec![0u8; 4 * WIDTH * HEIGHT];

// create canvas
let mut canvas = Canvas::new(WIDTH, HEIGHT).unwrap();

// draw line
shape::draw_line2d(50, 50, 200, 300, canvas, &ada::color::WHITE, &mut buffer[..]);

// draw rectangle
shape::draw_rect2d(50, 100, 100, 150, canvas, &ada::color::RED, &mut buffer[..]); // hollow
shape::draw_rect2d_filled(50, 100, 90, 120, canvas, &ada::color::GREEN, &mut buffer[..]); // filled
```

You can find more examples for all shapes in `examples` folder. To run an example:
```shell
cargo run --example draw_hollow
```

## Contributing
Please feel free to open any issues or pull requests.

## License

This is under Apache 2.0 License.
