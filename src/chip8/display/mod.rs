extern crate piston_window;
use piston_window::*;

pub struct Chip8Display {
    pixels: [[bool; 64]; 32],
    prev_pixels: [[bool; 64]; 32]
    //window: &'a mut PistonWindow,
}

impl Chip8Display {

    pub fn new() -> Chip8Display {
        Chip8Display {
            pixels: [[false; 64];32],
            prev_pixels: [[false; 64];32],
            //window: WindowSettings::new("Chip8!", [640, 320]).exit_on_esc(true).automatic_close(true).build().unwrap(),
            //window: window
        }
    }

    pub fn disp_clear(&mut self) -> bool {
        // clear the display, return true if any on pixel was switched to off
        let mut collision = false;
        for (x, row) in self.pixels.iter_mut().enumerate() {
            for (y, col) in row.iter_mut().enumerate() {
                self.prev_pixels[x][y] = *col;
                if *col == true {
                    collision = true;
                }
                *col = false;
            }
        }
        collision
    }

    pub fn print_pixel(&mut self) {
        for (x, row) in self.pixels.iter_mut().enumerate() {
            for (y, col) in row.iter_mut().enumerate() {
                print!("{} ", *col);
            }
            println!();
        }
    }

    pub fn draw_byte(&mut self, sprite: u8, row: usize, column: usize) -> bool {
        let mut collision = false;
        for i in 0..8 {
            let _row= row%32 as usize;
            let _column = (column+i)%64 as usize;
            self.prev_pixels[_row][_column] = self.pixels[_row][_column];
            if (sprite & (0x80 >> i)) != 0 {
                if self.pixels[_row][_column] == true {
                    collision = true;
                }
                self.pixels[_row][_column] ^= true;
            }
        }
        collision
    }

    pub fn update(&mut self, window: &mut PistonWindow, e: &Event) {
        let pixel = &self.pixels;
        let prev_pixel = &self.prev_pixels;
        window.draw_2d(e, |c, g, _device| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            for (x, row) in pixel.iter().enumerate() {
                for (y, col) in row.iter().enumerate() {
                    if *col == true {
                        rectangle([255.0, 255.0, 255.0, 1.0], // white
                                  [(y * 10) as f64, (x * 10) as f64, 10.0, 10.0],
                                  c.transform, g);
                    } else if prev_pixel[x][y] {
//                        rectangle([0.0, 0.0, 0.0, 1.0], // black
//                                  [(y * 10) as f64, (x * 10) as f64, 10.0, 10.0],
//                                  c.transform, g);
                    }
                }
            }
        });
    }
}
