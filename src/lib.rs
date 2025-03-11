use std::f32::consts::PI;
use std::time::Instant;
pub mod render;

pub struct State {
    pub window_size: (u32, u32),
    pub start_time: Instant,
}

impl State {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            window_size: (width, height),
            start_time: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        // Update logic can be added here if needed
    }

    pub fn get_dot_position(&self) -> (f32, f32) {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let radius = 100.0;
        let angle = elapsed * PI / 2.0; // Adjust speed by changing the multiplier

        let x = (self.window_size.0 as f32 / 2.0) + radius * angle.cos();
        let y = (self.window_size.1 as f32 / 2.0) + radius * angle.sin();

        (x, y)
    }
}
