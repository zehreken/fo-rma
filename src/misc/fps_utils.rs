use std::time::{Duration, Instant};
pub struct FpsCounter {
    now: Instant,
    frames: i32,
}

impl FpsCounter {
    pub fn new() -> FpsCounter {
        FpsCounter {
            now: Instant::now(),
            frames: 0,
        }
    }

    pub fn tick(&mut self) {
        self.frames += 1;
    }

    pub fn average_frames_per_second(self) -> f32 {
        let duration: Duration = Instant::now() - self.now;
        self.frames as f32 / duration.as_secs() as f32
    }
}
