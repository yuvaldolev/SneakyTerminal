use std::time::Duration;

use spin_sleep::SpinSleeper;

use crate::timer::Timer;

pub struct VsyncAwaiter {
    timer: Timer,
    sleeper: SpinSleeper,
    target_seconds_per_frame: f32,
}

impl VsyncAwaiter {
    pub fn new(game_update_hz: u32) -> Self {
        Self {
            timer: Timer::new(),
            sleeper: SpinSleeper::new(Duration::from_secs(1).as_nanos() as u32),
            target_seconds_per_frame: 1.0 / (game_update_hz as f32),
        }
    }

    pub fn wait(&mut self) {
        self.timer.tick();

        if self.timer.get_delta() < self.target_seconds_per_frame {
            self.sleeper.sleep(Duration::from_secs_f32(
                self.target_seconds_per_frame - self.timer.get_delta(),
            ));
        }

        self.timer.tick();
    }
}
