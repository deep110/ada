extern crate ada;

use minifb::{Key, Window, WindowOptions};

use ada::{shape, Canvas};

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

fn draw_line(canvas: &mut Canvas) {
    shape::draw_line2d(50, 50, 200, 300, canvas, &ada::color::WHITE);
}

fn draw_rectangle(canvas: &mut Canvas) {
    shape::draw_rect2d_filled(50, 100, 100, 150, canvas, &ada::color::RED);
}

fn draw_circle(canvas: &mut Canvas) {
    shape::draw_ellipse2d_filled(350, 200, 100, 100, canvas, &ada::color::BLUE);
}

fn draw_ellipse(canvas: &mut Canvas) {
    shape::draw_ellipse2d_filled(150, 400, 100, 50, canvas, &ada::color::GREEN);
}

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
    let mut canvas = Canvas::new(WIDTH, HEIGHT, &mut cbuffer[..]).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw_line(&mut canvas);
        draw_rectangle(&mut canvas);
        draw_circle(&mut canvas);
        draw_ellipse(&mut canvas);

        for (i, pix) in buffer.iter_mut().enumerate() {
            let c = canvas.get_color((i % WIDTH) as i32, (i / WIDTH) as i32);
            *pix = ((c[0] as u32) << 16) | ((c[1] as u32) << 8) | (c[2] as u32);
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        canvas.clear(&ada::color::BLACK);
    }
}
