use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::time::Duration;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut rng = rand::thread_rng();

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.get_mouse_pos(minifb::MouseMode::Discard).map(|(x, y)| {
            let x = x as usize;
            let y = y as usize;
            let tile_size = 10;
            let color: u32 = rng.gen();
            for tile_x in 0..tile_size {
                for tile_y in 0..tile_size {
                    let pixel_x = x + tile_x;
                    let pixel_y = y + tile_y;
                    if pixel_x < WIDTH && pixel_y < HEIGHT {
                        buffer[pixel_y * WIDTH + pixel_x] = color;
                    }
                }
            }
        });

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
        std::thread::sleep(Duration::from_millis(10));
    }
}
