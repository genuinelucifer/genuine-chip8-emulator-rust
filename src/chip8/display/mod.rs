struct Chip8Display {
    pixels: [[bool; 64]; 32]
}

impl Chip8Display {
    pub fn disp_clear(&mut self) -> bool {
        // clear the display, return true if any on pixel was switched to off
        let mut collision = false;
        for (x, row) in self.pixels.iter_mut().enumerate() {
            for (y, col) in row.iter_mut().enumerate() {
                if *col == true {
                    collision = true;
                }
                *col = false;
            }
        }
        collision
    }
}
