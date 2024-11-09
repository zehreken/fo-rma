use std::sync::atomic::{AtomicU32, Ordering};

pub struct AudioClock {
    sample_count: AtomicU32,
}

impl AudioClock {
    pub fn new() -> Self {
        Self {
            sample_count: AtomicU32::new(0),
        }
    }

    pub fn update(&self) {
        self.sample_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn sample_count(&self) -> u32 {
        self.sample_count.load(Ordering::Relaxed)
    }
}
