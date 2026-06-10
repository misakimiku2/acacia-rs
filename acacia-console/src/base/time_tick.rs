use std::time::{Duration, Instant};

pub struct TimeTick {
    interval: Duration,
    last_time: Instant,
}

impl TimeTick {
    pub fn new(interval: Duration) -> TimeTick {
        Self {
            interval: interval,
            last_time: Instant::now(),
        }
    }

    pub fn tick(&mut self) -> bool {
        if self.last_time.elapsed() >= self.interval {
            self.last_time = Instant::now();
            return true;
        }
        false
    }

    pub fn reset(&mut self) {
        self.last_time = Instant::now();
    }
}
