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
            let temp = self.pixels[_row][_column];
            if (sprite & (0x80 >> i)) != 0 {
                if self.pixels[_row][_column] == true {
                    collision = true;
                }
                self.pixels[_row][_column] ^= true;
            }
        }
        collision
    }

    pub fn update(&mut self) {
        let mut flag = false;
        while let Some(e) = self.window.next() {
            if flag {
                break;
            }
            if let Some(r) = e.render_args() {
                for (x, row) in self.pixels.iter().enumerate() {
                    for (y, col) in row.iter().enumerate() {
                        if *col == true {
                            self.window.draw_2d(&e, |c, g, _device| {
                                rectangle([255.0, 255.0, 255.0, 1.0], // white
                                          [(y*10) as f64, (x*10) as f64, 10.0, 10.0],
                                          c.transform, g);
                            });
                        } else {
                            self.window.draw_2d(&e, |c, g, _device| {
                                rectangle([0.0, 0.0, 0.0, 1.0], // black
                                          [(y*10) as f64, (x*10) as f64, 10.0, 10.0],
                                          c.transform, g);

                            });
                        }
                    }
                }
                flag = true;
            }
        }
    }

    pub fn get_pressed_key(&mut self) -> Option<Key> {
        let mut events = Events::new(EventSettings::new().lazy(true));
        let mut key = None;
        if let Some(e) = events.next(&mut self.window) {
            if let Some(Button::Keyboard(k)) = e.press_args() {
                println!("Pressed keyboard key '{:?}'", k);
                key = Some(k);
            }
        }
        key
    }
}
