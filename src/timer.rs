pub struct Timer<A, B> {
    period: usize,
    delta: usize,
    time_left: usize,
    on_tick: A,
    on_done: B,
}

impl<A: FnMut(f64), B: FnMut()> Timer<A, B> {
    pub fn new(period: usize, delta: usize, on_tick: A, on_done: B) -> Self {
        Self {
            period,
            delta,
            time_left: 0,
            on_tick,
            on_done,
        }
    }

    pub fn wind_up(&mut self) {
        self.time_left = self.period;
    }

    pub fn tick(&mut self) {
        (self.on_tick)(
            ((self.period as f64 - self.time_left as f64) / self.period as f64 * 100.0).trunc(),
        );

        self.time_left -= self.delta;

        if self.time_left == 0 {
            (self.on_done)();
        }
    }
}
