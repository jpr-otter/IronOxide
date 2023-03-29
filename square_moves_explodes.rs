use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::time::{Duration, Instant};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

impl Particle {
    fn new(x: f32, y: f32) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(1.0..5.0);
        Self {
            x,
            y,
            vx: angle.cos() * speed,
            vy: angle.sin() * speed,
        }
    }

    fn update(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }

    fn draw(&self, buffer: &mut Vec<u32>) {
        let x = self.x as usize;
        let y = self.y as usize;
        if x < WIDTH && y < HEIGHT {
            buffer[y * WIDTH + x] = 0xffffffff;
        }
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut x = WIDTH as f32 / 2.0;
    let mut y = HEIGHT as f32 / 2.0;
    let square_size = 20.0;
    let mut particles = Vec::new();
    let mut respawn_time: Option<Instant> = None;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer = vec![0; WIDTH * HEIGHT];

        if window.is_key_down(Key::W) {
            y -= 1.0;
        }
        if window.is_key_down(Key::S) {
            y += 1.0;
        }
        if window.is_key_down(Key::A) {
            x -= 1.0;
        }
        if window.is_key_down(Key::D) {
            x += 1.0;
        }

        if particles.is_empty() {
            if let Some(time) = respawn_time {
                if time.elapsed().as_secs_f32() >= 5.0 {
                    respawn_time = None;
                    x = WIDTH as f32 / 2.0;
                    y = HEIGHT as f32 / 2.0;
                }
            } else if x < square_size / 2.0
                || x > WIDTH as f32 - square_size / 2.0
                || y < square_size / 2.0
                || y > HEIGHT as f32 - square_size / 2.0
            {
                for i in 0..square_size as usize {
                    for j in 0..square_size as usize {
                        particles.push(Particle::new(
                            x + i as f32 - square_size / 2.0,
                            y + j as f32 - square_size / 2.0,
                        ));
                    }
                }
                respawn_time = Some(Instant::now());
            } else {
                for i in 0..square_size as usize {
                    for j in 0..square_size as usize {
                        let px = (x + i as f32 - square_size / 2.0) as usize;
                        let py = (y + j as f32 - square_size / 2.0) as usize;
                        if px < WIDTH && py < HEIGHT {
                            buffer[py * WIDTH + px] = 0xffffffff;
                        }
                    }
                }
            }
        } else {
            particles.retain(|p| p.x >= 0.0 && p.x < WIDTH as f32 && p.y >= 0.0 && p.y < HEIGHT as f32);
            for particle in &mut particles {
                particle.update();
                particle.draw(&mut buffer);
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
