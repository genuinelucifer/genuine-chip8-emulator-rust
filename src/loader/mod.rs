use std::fs;
use std::io;

pub fn get_roms(path: &String) -> Result<Vec<fs::DirEntry>, io::Error> {
    fs::read_dir(path).and_then(
        |paths|
            Ok(
                paths
                    .filter(|path|path.is_ok())
                    .map(|path|path.unwrap())
                    .collect::<Vec<fs::DirEntry>>()
            )

    )
}

pub fn load_roms(path: &Option<&str>) -> Result<Vec<u8>, io::Error> {
    fs::read(String::from(path.unwrap()).clone())
}