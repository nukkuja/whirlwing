use std::time::{Instant, Duration};

#[derive(Debug)]
pub(crate) struct Time {
    start: Instant,
    now: Duration,
    delta_time: Duration,
}

#[allow(dead_code)]
impl Time {
    pub(crate) fn start() -> Time {
        let start = Instant::now();
        let now = start.elapsed();
        let delta_time = start.elapsed();
        Time { start, now, delta_time }
    }

    pub(crate) fn reset(&mut self) {
        self.start = Instant::now();
        self.now = self.start.elapsed();
        self.delta_time = Duration::new(0, 0);
    }

    pub(crate) fn tick(&mut self) {
        let elapsed = self.start.elapsed();
        self.delta_time = elapsed - self.now;
        self.now = elapsed;
    }

    pub(crate) fn now(&self) -> Duration {
        self.now
    }

    pub(crate) fn delta_time(&self) -> Duration {
        self.delta_time
    }
}