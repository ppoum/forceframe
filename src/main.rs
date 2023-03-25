use std::time::{Duration, Instant};
use rand::{Rng, SeedableRng};
use sfml::window::{Event, Style};
use crate::engine::circle::Circle;
use crate::engine::engine_object::EngineObject;
use crate::engine::world::World;
use crate::utils::Vec2f;


const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;
const TIME_STEP: f64 = 1.0 / 60.0;
const SUB_STEP_CNT: u32 = 64;
const RND_SEED: u64 = 141414;

// Defining src modules
pub mod engine {
    pub mod circle;
    pub mod engine_object;
    pub mod world;
}

pub mod utils;

fn main() {

    let mut window = sfml::graphics::RenderWindow::new(
        (WIDTH as u32, HEIGHT as u32), "forceframe",
        Style::CLOSE, &Default::default());
    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(false);

    let c1 = Circle::new(Vec2f::new(100.0, 100.0), 10.0);
    let mut c2 = Circle::new(Vec2f::new(105.0, 200.0), 10.0);
    c2.add_vel(&Vec2f::new(2.0, 0.0), 0.0068);
    let world_center = Vec2f {
        x: WIDTH as f64 / 2.0,
        y: HEIGHT as f64 / 2.0,
    };

    let mut w = World::new(world_center, 400.0);
    w.set_sub_step_cnt(SUB_STEP_CNT);
    w.add_object(Box::new(c1));
    w.add_object(Box::new(c2));

    let mut last = Instant::now();

    let mut last_spawn = Instant::now();
    let max_spawn_cnt = 1000;
    let mut spawn_cnt = 0;
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(RND_SEED);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
        }

        // Update time
        let now = Instant::now();
        let delta = now - last;
        last = now;

        if spawn_cnt < max_spawn_cnt
            && (now - last_spawn) > Duration::from_millis((100.0) as u64) {
            last_spawn = Instant::now();
            spawn_cnt += 1;
            let mut new_circle = Circle::new(Vec2f::new(WIDTH as f64 / 2.0, 200.0),
                                             rng.gen_range(5..15) as f64);
            new_circle.add_vel(&Vec2f::new(400.0, 0.0), TIME_STEP / SUB_STEP_CNT as f64);
            w.add_object(Box::new(new_circle));
        }

        println!("FPS: {}", 1000000 / (delta.as_micros() + 1));  // +1 to avoid div by zero
        w.tick(TIME_STEP);
        w.sfml_render(&mut window);

        window.set_active(true);
        window.display();
    }
}
