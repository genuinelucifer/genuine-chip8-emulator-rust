extern crate piston_window;

mod loader;
mod chip8;

use std::io::stdin;
use piston_window::*;


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
                        println!("{:?}",&roms_with_index[line].1.path().to_str());
                        let rom_data = loader::load_roms(&roms_with_index[line-1].1.path().to_str());
                        match rom_data {
                            Ok(v) => {
                                let mut chip8_cpu = chip8::cpu::Chip8CPU::new();
                                chip8_cpu.load_program(&v);
                                println!("{:?}", v);
                                loop {
                                    chip8_cpu.exec_next_instruction();
                                }
                            },
                            Err(e) => {
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
