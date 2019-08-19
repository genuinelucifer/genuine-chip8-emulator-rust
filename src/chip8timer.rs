use std::time::Instant;

pub struct Chip8Timer {
    val: u16,
    last_time: Instant
}

impl Chip8Timer {
    fn new() -> Chip8Timer {
        Chip8Timer {
            val: 0,
            last_time: Instant::now()
        }
    }

    fn update_val(&mut self) {
        let mili_passed = self.last_time.elapsed().as_millis();
        // Reduce the counter by 60 every second
        let reduction = u16(60 * f32(mili_passed / 1000.00));
        if reduction > 0 {
            self.last_time = Instant::now();
            self.val -= reduction;
            if self.val < 0 {
                self.val = 0;
            }
        }
    }

    fn get_val(&mut self) -> u16 {
        self.update_val();
        self.val
    }

    fn set_val(&mut self, val: u16) {
        self.val = val;
        self.last_time = Instant::now();
    }
}