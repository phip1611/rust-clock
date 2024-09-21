use minifb::{Window, WindowOptions};
use minifb_geometry::GeometryDrawer;
use rust_clock::Clock;

const WIDTH: usize = 400;
const HEIGHT: usize = WIDTH;

const CLOCK_SIZE: usize = HEIGHT / 2 - 10;
const CLOCK_ORIGIN: (usize, usize) = (HEIGHT / 2, HEIGHT / 2);

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
        let clock = Clock::new();

        // clear screen
        let _ = geometry.draw_box(&mut buffer, 0, 0, WIDTH, HEIGHT, 0);

        let _ = geometry.draw_circle(
            &mut buffer,
            CLOCK_ORIGIN.0,
            CLOCK_ORIGIN.1,
            CLOCK_SIZE,
            0xffffff,
        );

        // hour
        let (hour_x, hour_y) = clock.hour_coordinates(CLOCK_ORIGIN.0, CLOCK_ORIGIN.1, CLOCK_SIZE);
        let _ = geometry.draw_line(
            &mut buffer,
            CLOCK_ORIGIN.0,
            CLOCK_ORIGIN.1,
            hour_x,
            hour_y,
            0xff0000,
        );
        // minute
        let (minute_x, minute_y) =
            clock.minute_coordinates(CLOCK_ORIGIN.0, CLOCK_ORIGIN.1, CLOCK_SIZE);
        let _ = geometry.draw_line(
            &mut buffer,
            CLOCK_ORIGIN.0,
            CLOCK_ORIGIN.1,
            minute_x,
            minute_y,
            0x00ff00,
        );
        // second
        let (second_x, second_y) =
            clock.second_coordinates(CLOCK_ORIGIN.0, CLOCK_ORIGIN.1, CLOCK_SIZE);
        let _ = geometry.draw_line(
            &mut buffer,
            CLOCK_ORIGIN.0,
            CLOCK_ORIGIN.1,
            second_x,
            second_y,
            0x0000ff,
        );

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        //sleep(Duration::from_secs(1));
    }
}
