use std::time::{Duration, Instant};
pub struct FpsCounter {
    start: Instant,
    now: Instant,
    frames: i32,
}

impl FpsCounter {
    pub fn new() -> FpsCounter {
        FpsCounter {
            start: Instant::now(),
            now: Instant::now(),
            frames: 0,
        }
    }

    pub fn tick(&mut self) {
        self.frames += 1;
    }

    pub fn average_frames_per_second(&self) -> f32 {
        let duration: Duration = Instant::now() - self.start;
        self.frames as f32 / duration.as_secs() as f32
    }

    pub fn get_delta_time(&mut self) -> f32 {
        let now = Instant::now();
        let delta_time = now.duration_since(self.now);
        self.now = now;

        delta_time.as_secs_f32()
    }
}
