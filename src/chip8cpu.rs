mod chip8mem
mod chip8timer

pub struct Chip8CPU {
    V: [u8, 16],
    I: u16,
    M: chip8mem::Chip8Memory,   // Memory
    PC: u16,
    stack: [u16, 24],           // Stack with 48 bytes, 24 levels of nesting
    SP: u8,                     // Current position of calls in stack
    delay_timer: chip8timer::Chip8Timer,
    sound_timer: chip8timer::Chip8Timer,
}

impl Chip8CPU {

}

