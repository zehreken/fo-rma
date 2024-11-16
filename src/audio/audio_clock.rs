use std::sync::atomic::{AtomicU32, Ordering};

pub struct AudioClock {
    elapsed_samples: AtomicU32,
}

impl AudioClock {
    pub fn new() -> Self {
        Self {
            elapsed_samples: AtomicU32::new(0),
        }
    }

    pub fn update(&self) {
        self.elapsed_samples.fetch_add(1, Ordering::Release);
    }

    pub fn get_elapsed_samples(&self) -> u32 {
        self.elapsed_samples.load(Ordering::Acquire)
    }
}
