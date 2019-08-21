extern crate chrono;
use chrono::prelude::*;

pub struct Chip8Timer {
    value: u8,
    last_time: i64
}

fn cur_time() -> i64 {
    Utc::now().timestamp_millis()
}

impl Chip8Timer {

    pub fn new() -> Chip8Timer {
        Chip8Timer {
            value: 0,
            last_time: cur_time()
        }
    }

    pub fn update_value(&mut self) {
        let cur_time = cur_time();
        let mili_passed = cur_time - self.last_time;
        // Reduce the counter by 60 every second
        let reduction = (60.0 * (mili_passed as f32 / 1000.00)) as u8;
        if reduction > 0 {
            self.last_time = cur_time;
            self.value -= reduction;
            if self.value < 0 {
                self.value = 0;
            }
        }
    }

    pub fn get_value(&mut self) -> u8 {
        self.update_value();
        self.value
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
        self.last_time = cur_time();
    }
}