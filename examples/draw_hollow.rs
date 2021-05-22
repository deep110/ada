extern crate ada;

use minifb::{Key, Window, WindowOptions};

use ada::{shape, Canvas};

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

fn draw_line(canvas: &mut Canvas, buffer: &mut [u8]) {
    shape::draw_line2d(50, 50, 200, 300, canvas, &ada::color::WHITE, buffer);
}

fn draw_rectangle(canvas: &mut Canvas, buffer: &mut [u8]) {
    shape::draw_rect2d(50, 100, 100, 150, canvas, &ada::color::RED, buffer);
}

fn draw_circle(canvas: &mut Canvas, buffer: &mut [u8]) {
    shape::draw_ellipse2d(350, 200, 100, 100, canvas, &ada::color::BLUE, buffer);
}

fn draw_ellipse(canvas: &mut Canvas, buffer: &mut [u8]) {
    shape::draw_ellipse2d(150, 400, 100, 50, canvas, &ada::color::GREEN, buffer);
}

fn draw_polygon(canvas: &mut Canvas, buffer: &mut [u8]) {
    let xi: [i32; 10] = [127, 243, 62, 110, 93, 193, 135, 70, 258, 248];
    let yi: [i32; 10] = [320, 15, 162, 54, 311, 314, 290, 10, 163, 155];

    shape::draw_polygon2d(&xi, &yi, canvas, &ada::Color::rgb(255, 255, 0), buffer);
}

fn draw_quadratic_bezier(canvas: &mut Canvas, buffer: &mut [u8]) {
    shape::draw_quadratic_bezier2d(
        (70, 250),
        (220, 60),
        (20, 110),
        canvas,
        &ada::Color::rgb(0, 255, 255),
        buffer,
    );
}

fn draw_cubic_bezier(canvas: &mut Canvas, buffer: &mut [u8]) {
    shape::draw_cubic_bezier2d(
        (110, 150),
        (210, 30),
        (25, 190),
        (210, 250),
        canvas,
        &ada::Color::rgb(255, 0, 255),
        buffer,
    );
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
    let mut canvas = Canvas::new(WIDTH, HEIGHT).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw_line(&mut canvas, &mut cbuffer[..]);
        draw_rectangle(&mut canvas, &mut cbuffer[..]);
        draw_circle(&mut canvas, &mut cbuffer[..]);
        draw_ellipse(&mut canvas, &mut cbuffer[..]);
        draw_polygon(&mut canvas, &mut cbuffer[..]);
        draw_quadratic_bezier(&mut canvas, &mut cbuffer[..]);
        draw_cubic_bezier(&mut canvas, &mut cbuffer[..]);

        for (i, pix) in buffer.iter_mut().enumerate() {
            let c = canvas.get_color((i % WIDTH) as i32, (i / WIDTH) as i32, &mut cbuffer[..]);
            *pix = ((c[0] as u32) << 16) | ((c[1] as u32) << 8) | (c[2] as u32);
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        canvas.clear(&ada::color::BLACK, &mut cbuffer[..]);
    }
}
