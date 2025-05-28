pub struct Timer {
    period: usize,
    delta: usize,
        
    time_left: usize,
}

impl Timer {
    
    pub fn new(period: usize, delta: usize) -> Self {
        Self {
            period,
            delta,
            time_left: 0
        }
    }
    
    pub fn wind_up(&mut self) {
        self.time_left = self.period;
    }
    
    pub fn tick(&mut self) {
        println!("Working on it: {}%", ((self.period as f64 - self.time_left as f64) / self.period as f64 * 100.0).trunc());
        
        self.time_left -= self.delta;
        
        if self.time_left == 0 {
            println!("Done!");
        }
        
    } 
}