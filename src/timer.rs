use std::time::{Duration, Instant};

pub struct Timer {
    last_instant: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            last_instant: Instant::now(),
        }
    }

    pub fn measure_delta(&mut self) -> Duration {
        let last_instant = self.last_instant;
        self.last_instant = Instant::now();
        self.last_instant - last_instant
    }
}
