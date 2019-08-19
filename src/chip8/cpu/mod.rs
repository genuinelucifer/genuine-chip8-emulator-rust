use super::memory;
use super::timer;

pub struct Chip8CPU {
    V: [u8; 16],
    I: u16,
    MEM: memory::Chip8Memory,   // Memory
    PC: u16,
    stack: [u16; 24],           // Stack with 48 bytes, 24 levels of nesting
    SP: u8,                     // Current position of calls in stack
    delay_timer: timer::Chip8Timer,
    sound_timer: timer::Chip8Timer,
}

impl Chip8CPU {
    pub fn new() -> Chip8CPU {
        Chip8CPU {
            V: [0; 16],
            I: 0,
            MEM: memory::Chip8Memory::new(),
            PC: 0x200,
            stack: [0; 24],
            SP: 0,
            delay_timer: timer::Chip8Timer::new(),
            sound_timer: timer::Chip8Timer::new()
        }
    }

    pub fn load_program(&mut self, program: &Vec<u8>) {
        self.MEM.load_memory(program, 0x200);
        self.PC = 0x200;
    }

    pub fn exec_next_instruction(&mut self) {
        // read the instruction at PC and execute it
        let (byte0, byte1) = self.MEM.get_word(self.PC as usize);
        println!("Bytes received: {}, {}", byte0, byte1);
        self.PC += 2;
        if byte0 == 0 && byte1 == 0 {
            // exit
            std::process::exit(0);
        }
        // TODO: Handle all other op codes here
    }
}

