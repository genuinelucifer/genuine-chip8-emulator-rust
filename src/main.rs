mod loader;

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
                    Ok(line) => {
                        println!("Ok {}", line);
                        println!("{:?}",&roms_with_index[line].1.path().to_str());
                        println!("{:?}", loader::load_roms(&roms_with_index[line].1.path().to_str()));
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

fn main() {
    println!("Chip-8 Emulator in Rust");
    match start() {
        Ok(_e) => {},
        Err(e) => println!("{:?}", e)
    }
}
