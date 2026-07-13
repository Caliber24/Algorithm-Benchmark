use std::time::Instant;

pub struct Timer {
    start: Option<Instant>,
    end: Option<Instant>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
        self.end = None;
    }

    pub fn stop(&mut self) {
        self.end = Some(Instant::now());
    }

    pub fn elapsed_ms(&self) -> f64 {
        match (self.start, self.end) {
            (Some(start), Some(end)) => {
                end.duration_since(start).as_secs_f64() * 1000.0
            }
            _ => 0.0,
        }
    }
}