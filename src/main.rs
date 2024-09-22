use minifb::{Window, WindowOptions};
use minifb_geometry::GeometryDrawer;
use rust_clock::Clock;

const WIDTH: usize = 800;
const HEIGHT: usize = 400;

const CLOCK_SIZE_PADDING: usize = 7;
const CLOCK_SIZE: usize = HEIGHT / 2 - CLOCK_SIZE_PADDING;
const CLOCK_L_ORIGIN: (usize, usize) = (WIDTH / 4, HEIGHT / 2);
const CLOCK_R_ORIGIN: (usize, usize) = (WIDTH / 4 * 3, HEIGHT / 2);

fn draw_clock(
    buffer: &mut Vec<u32>,
    geometry: &GeometryDrawer,
    clock: &Clock,
    origin_x: usize,
    origin_y: usize,
    size: usize,
) {
    // Draw clock border.
    let _ = geometry.draw_circle(buffer, origin_x, origin_y, size, 0);

    // hour
    let (hour_x, hour_y) = clock.hour_coordinates(origin_x, origin_y, size);
    let _ = geometry.draw_line(buffer, origin_x, origin_y, hour_x, hour_y, 0xff0000);

    // minute
    let (minute_x, minute_y) = clock.minute_coordinates(origin_x, origin_y, size);
    let _ = geometry.draw_line(buffer, origin_x, origin_y, minute_x, minute_y, 0x00ff00);

    // second
    let (second_x, second_y) = clock.second_coordinates(origin_x, origin_y, size);
    let _ = geometry.draw_line(buffer, origin_x, origin_y, second_x, second_y, 0x0000ff);
}

/// Draws two clocks side-by-side in a minifb-based GUI.
fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let geometry = GeometryDrawer::new(WIDTH);

    let mut window = Window::new(
        "Digital Analog Wall Clock - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Failed to create window by minifb: {:#?}", e);
    });

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        // clear screen
        let _ = geometry.draw_box(&mut buffer, 0, 0, WIDTH, HEIGHT, 0xffffff);

        let clock_left = Clock::new(true);
        draw_clock(
            &mut buffer,
            &geometry,
            &clock_left,
            CLOCK_L_ORIGIN.0,
            CLOCK_L_ORIGIN.1,
            CLOCK_SIZE,
        );

        let clock_right = Clock::new(false);
        draw_clock(
            &mut buffer,
            &geometry,
            &clock_right,
            CLOCK_R_ORIGIN.0,
            CLOCK_R_ORIGIN.1,
            CLOCK_SIZE,
        );

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
