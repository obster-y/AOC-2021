use std::time::{SystemTime};

pub struct Timer {
    timer: SystemTime
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            timer: SystemTime::now()
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let elsp = 0.001 * (
            self.timer
            .elapsed()
            .unwrap()
            .as_millis() as f32
        );
        println!("Used: {} seconds", elsp);
    }
}
