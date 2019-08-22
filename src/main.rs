extern crate piston_window;
use piston_window::*;

mod loader;
mod chip8;

use std::io::stdin;


fn start() -> Result<(), std::io::Error> {
    let rom_list = loader::get_roms(&String::from("roms/"));

    rom_list.and_then(|roms|{
        let roms_with_index = (1..roms.len())
            .zip(roms)
            .collect::<Vec<_>>();

        loop {
            roms_with_index.iter().for_each(
                |(ind, path)| println!("{}: {:?}", ind, path.file_name())
            );
            println!("Select Game 1 to {}:", roms_with_index.len());
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
                                let mut window: PistonWindow = WindowSettings::new("Chip8!", [640, 320]).exit_on_esc(true).automatic_close(true).build().unwrap();
                                let mut chip8_cpu = chip8::cpu::Chip8CPU::new();
                                chip8_cpu.load_program(&v);
                                println!("{:?}", v);

                                let mut events = Events::new(EventSettings::new());

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
                                            _ => {}
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
                                            _ => {}
                                        }
                                    }
                                }

//                                loop {
//                                    chip8_cpu.exec_next_instruction();
//                                }
                            },
                            Err(_e) => {
                                println!("Error occurred while loading rom!");
                            }
                        }
                        Ok(())
                    },
                    Err(e) => {
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
