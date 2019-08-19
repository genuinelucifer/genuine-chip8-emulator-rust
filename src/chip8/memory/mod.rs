pub struct Chip8Memory {
    data: [u8; 4096]
}

impl Chip8Memory {
    pub fn new() -> Chip8Memory {
        Chip8Memory {
            data: [0; 4096]
        }
    }

    pub fn load_memory(&mut self, data: &Vec<u8>, start_pos: usize) {
        let mut cur_pos = start_pos;
        for byte in data.iter() {
            self.data[cur_pos] = *byte;
            cur_pos += 1;
        }
    }

    pub fn get_word(&self, pos: usize) -> (u8, u8) {
        (self.data[pos], self.data[pos+1])
    }
}