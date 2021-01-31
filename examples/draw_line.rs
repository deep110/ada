extern crate ada;

use minifb::{Key, Window, WindowOptions};

use ada::Canvas;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            topmost: true,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~30 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(33333)));

    // create canvas
    let mut cbuffer: Vec<u8> = vec![0; 4 * WIDTH * HEIGHT];
    let mut canvas = Canvas::new(WIDTH, HEIGHT, &mut cbuffer[..]);

    let white = ada::color::WHITE;
    let line = ada::shape::Line2D::new(50, 50, 60, 60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        canvas.draw(&line, &white, false);

        for (i, pix) in buffer.iter_mut().enumerate() {
            let c = canvas.get_color(i % WIDTH, i / WIDTH);
            *pix = ((c[0] as u32) << 16) | ((c[1] as u32) << 8) | (c[2] as u32);
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        canvas.clear(&ada::color::BLACK);
    }
}
