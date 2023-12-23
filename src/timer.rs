use std::time::Instant;

pub struct Timer {
    instant: Instant,
    delta: f32,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            instant: Instant::now(),
            delta: 0.0,
        }
    }

    pub fn tick(&mut self) {
        let last_instant = self.instant;
        self.instant = Instant::now();
        self.delta = (self.instant - last_instant).as_secs_f32();
    }

    pub fn get_delta(&self) -> f32 {
        self.delta
    }
}
