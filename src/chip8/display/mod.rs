extern crate piston_window;
use piston_window::*;

pub struct Chip8Display {
    pixels: [[bool; 64]; 32],
    window: PistonWindow,
}

impl Chip8Display {

    pub fn new() -> Chip8Display {
        Chip8Display {
            pixels: [[false; 64];32],
            window: WindowSettings::new("Chip8!", [640, 320])

        .exit_on_esc(true).build().unwrap()
        }
    }

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

    pub fn draw_byte(&mut self, sprite: u8, row: &usize, column: &usize) -> bool {
        let mut collision = false;

        for i in 0..8 {
            let _row= row%32 as usize;
            let _column = (column+i)%64 as usize;
            let temp = self.pixels[_row][_column];
            if (sprite & (0x80 >> i)) == 1 {
                if self.pixels[_row][_column] == true {
                    collision = true;
                }
                self.pixels[_row][_column] ^= true;
            }
        }
        collision
    }

    pub fn update(&mut self) {
        if let Some(e) = self.window.next() {
            for (x, row) in self.pixels.iter_mut().enumerate() {
                for (y, col) in row.iter_mut().enumerate() {
                    if *col == true {
                        self.window.draw_2d(&e, |c, g, _device| {
                            //clear([0.0; 4], g);
                            //println!("drawing at x: {}, y: {}", x, y);
                            rectangle([255.0, 255.0, 255.0, 1.0], // white
                                      [0.0+(y*32) as f64, 0.0+(x*64) as f64, 10.0+(y*32) as f64, 10.0+(x*64) as f64],
                                      c.transform, g);
                        });
                    } else {
                        self.window.draw_2d(&e, |c, g, _device| {
                            //clear([0.0; 4], g);
                            //println!("drawing at x: {}, y: {}", x, y);
                            rectangle([0.0, 0.0, 0.0, 1.0], // white
                                      [0.0+(y*32) as f64, 0.0+(x*64) as f64, 10.0+(y*32) as f64, 10.0+(x*64) as f64],
                                      c.transform, g);
                        });
                    }
                }
            }
        }
    }
}
