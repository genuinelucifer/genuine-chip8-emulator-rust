extern crate piston_window;
use piston_window::*;

mod loader;
mod chip8;

use std::{io::stdin, process::exit};


fn start() -> Result<(), std::io::Error> {
    let rom_list = loader::get_roms(&String::from("roms/"));

    rom_list.and_then(|roms|{
        let roms_with_index = (1..roms.len())
            .zip(roms)
            .collect::<Vec<_>>();

        let mut events = Events::new(EventSettings::new());
        let mut window: PistonWindow = WindowSettings::new("Chip8!", [640, 320]).build().unwrap();
        window.window.hide();
        let mut chip8_cpu = chip8::cpu::Chip8CPU::new();

        loop {
            roms_with_index.iter().for_each(
                |(ind, path)| println!("{}: {:?}", ind, path.file_name())
            );
            println!("Select Game 1 to {} (enter 'exit' to quit):", roms_with_index.len());
            let mut line = String::new();
            let _ = stdin().read_line(&mut line).and_then(|_x|
                match line.trim().parse::<usize>(){
                    Ok(line)  => {
                        // TODO:: add line>0 && line<rom_list.len()
                        println!("Ok {}", line);
                        println!("{:?}",&roms_with_index[line - 1].1.path().to_str());
                        let rom_data = loader::load_roms(&roms_with_index[line-1].1.path().to_str());
                        match rom_data {
                            Ok(v) => {
                                chip8_cpu.load_program(&v);
                                println!("{:?}", v);
                                window.window.show();

                                while let Some(e) = events.next(&mut window) {
                                    if let Some(Button::Keyboard(key)) = e.press_args() {
                                        match key {
                                            Key::X => chip8_cpu.set_key(0x0),
                                            Key::D1 => chip8_cpu.set_key(0x1),
                                            Key::D2 => chip8_cpu.set_key(0x2),
                                            Key::D3 => chip8_cpu.set_key(0x3),
                                            Key::Q => chip8_cpu.set_key(0x4),
                                            Key::W => chip8_cpu.set_key(0x5),
                                            Key::E => chip8_cpu.set_key(0x6),
                                            Key::A => chip8_cpu.set_key(0x7),
                                            Key::S => chip8_cpu.set_key(0x8),
                                            Key::D => chip8_cpu.set_key(0x9),
                                            Key::Z => chip8_cpu.set_key(0xA),
                                            Key::C => chip8_cpu.set_key(0xB),
                                            Key::D4 => chip8_cpu.set_key(0xC),
                                            Key::R => chip8_cpu.set_key(0xD),
                                            Key::F => chip8_cpu.set_key(0xE),
                                            Key::V => chip8_cpu.set_key(0xF),
                                            Key::Escape => {
                                                chip8_cpu.clear_display();
                                                window.window.hide();
                                                break;
                                            }
                                            _ => {
                                                println!("Key pressed = {}", key as i64)
                                            }
                                        }
                                    }

                                    chip8_cpu.update_display(&mut window, &e);
                                    chip8_cpu.exec_next_instruction();

                                    if let Some(Button::Keyboard(key)) = e.release_args() {
                                        match key {
                                            Key::X => chip8_cpu.unset_key(0x0),
                                            Key::D1 => chip8_cpu.unset_key(0x1),
                                            Key::D2 => chip8_cpu.unset_key(0x2),
                                            Key::D3 => chip8_cpu.unset_key(0x3),
                                            Key::Q => chip8_cpu.unset_key(0x4),
                                            Key::W => chip8_cpu.unset_key(0x5),
                                            Key::E => chip8_cpu.unset_key(0x6),
                                            Key::A => chip8_cpu.unset_key(0x7),
                                            Key::S => chip8_cpu.unset_key(0x8),
                                            Key::D => chip8_cpu.unset_key(0x9),
                                            Key::Z => chip8_cpu.unset_key(0xA),
                                            Key::C => chip8_cpu.unset_key(0xB),
                                            Key::D4 => chip8_cpu.unset_key(0xC),
                                            Key::R => chip8_cpu.unset_key(0xD),
                                            Key::F => chip8_cpu.unset_key(0xE),
                                            Key::V => chip8_cpu.unset_key(0xF),
                                            _ => {
                                                println!("Key released = {}", key as i64);
                                            }
                                        }
                                    }
                                }

                                if events.next(&mut window) == None {
                                    exit(0)
                                }
                            },
                            Err(_e) => {
                                println!("Error occurred while loading rom!");
                            }
                        }
                        Ok(())
                    },
                    Err(e) => {
                        if line.trim() == "exit" {
                            exit(0)
                        }
                        println!("Not Ok {}", e);
                        Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid input"))
                    }
                });
        }
        //Ok(())
    })
}

fn main() -> Result<(), std::io::Error> {
    println!("Chip-8 Emulator in Rust");
    start()
}
