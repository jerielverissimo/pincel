use chrono::{Local, Timelike};

#[derive(Debug, Clone, Copy)]
pub struct Clock {
    pub start_time: f64,
    pub elapsed: f64,
}

impl Clock {
    /// Updates the clock. Should be called just checking elapsed time.
    /// Has no effect on non-started clocks.
    pub fn update(&mut self) {
        if self.start_time != 0.0 {
            self.elapsed = Self::get_absolute_time() - self.start_time;
        }
    }

    /// Starts the provided clock. Resets elapsed time.
    pub fn start(&mut self) {
        self.start_time = Self::get_absolute_time();
        self.elapsed = 0.0;
    }

    /// Stops the provided clock. Does not reset elapsed time.
    pub fn stop(&mut self) {
        self.start_time = 0.0;
    }

    pub fn get_absolute_time() -> f64 {
        let now = Local::now();

        let time = now.time();

        (time.second() + time.nanosecond()) as f64 * 0.000000001
    }
}
