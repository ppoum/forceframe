
use minifb::{Key, Window, WindowOptions};
use crate::engine::circle::Circle;
use crate::engine::world::World;
use crate::utils::Vec2;


const WIDTH: usize = 640;
const HEIGHT: usize = 360;

// Defining src modules
pub mod engine {
    pub mod circle;
    pub mod engine_object;
    pub mod world;
}
pub mod utils;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH*HEIGHT];

    let mut window = Window::new(
        "forceframe",
        WIDTH,
        HEIGHT,
        WindowOptions::default()
    ).unwrap();

    // Limit to 500 FPS
    window.limit_update_rate(Some(std::time::Duration::from_millis(2)));

    let x = Circle::new(Vec2::new(512, 512), 100f64);
    let mut w = World::new(WIDTH as u32, HEIGHT as u32, &mut buffer);
    // w.add_object(x);

    for (i, pixel) in buffer.iter_mut().enumerate() {
            let row = i / WIDTH;
            let col = i % WIDTH;

            let r = (255f64 * (row as f64) / (HEIGHT as f64)) as u32;
            let b = (255f64 * (col as f64) / (WIDTH as f64)) as u32;
            *pixel = (r << 16) + b;
        }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
