extern crate chrono;
use chrono::prelude::*;

pub struct Chip8Timer {
    val: u16,
    last_time: i64
}

fn cur_time() -> i64 {
    Utc::now().timestamp_millis()
}

impl Chip8Timer {

    pub fn new() -> Chip8Timer {
        Chip8Timer {
            val: 0,
            last_time: cur_time()
        }
    }

    pub fn update_val(&mut self) {
        let cur_time = cur_time();
        let mili_passed = cur_time - self.last_time;
        // Reduce the counter by 60 every second
        let reduction = (60.0 * (mili_passed as f32 / 1000.00)) as u16;
        if reduction > 0 {
            self.last_time = cur_time;
            self.val -= reduction;
            if self.val < 0 {
                self.val = 0;
            }
        }
    }

    pub fn get_val(&mut self) -> u16 {
        self.update_val();
        self.val
    }

    pub fn set_val(&mut self, val: u16) {
        self.val = val;
        self.last_time = cur_time();
    }
}