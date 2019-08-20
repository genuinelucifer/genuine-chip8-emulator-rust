use super::memory;
use super::timer;
use super::display;

pub struct Chip8CPU {
    V: [u8; 16],
    I: u16,
    MEM: memory::Chip8Memory,   // Memory
    PC: u16,
    stack: [u16; 24],           // Stack with 48 bytes, 24 levels of nesting
    SP: usize,                     // Current position of calls in stack
    delay_timer: timer::Chip8Timer,
    sound_timer: timer::Chip8Timer,
    display: display::Chip8Display,
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
            sound_timer: timer::Chip8Timer::new(),
            display: display::Chip8Display::new()
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
        let opcode = (byte0 as u16)<<8 | (byte1 as u16);
        match opcode & 0xF000 {
            0x0000 => {
                match opcode {
                    0x00E0 => {
                        //clear screen
                        self.display.disp_clear();
                    },
                    0x00EE => {
                        //return from subroutine
                        self.PC = self.stack[self.SP];
                        self.SP -= 1;
                    },
                    _ => {
                        //do nothing
                    }
                }
            },
            0x1000 => {
                // address jump
                self.PC = opcode & 0x0FFF;
            },
            0x2000 => {
                // call subroutine
                self.SP += 1;
                self.stack[self.SP] = self.PC;
                self.PC = opcode & 0x0FFF;
            },
            0x3000 => {
                // Skip next instruction if Vx = kk <-(3xkk)
                if self.V[((opcode & 0x0F00)>>8) as usize] == (opcode & 0x00FF) as u8 {
                    self.PC += 2;
                }
            },
            0x4000 => {
                // Skip next instruction if Vx != kk <-(4xkk)
                if self.V[((opcode & 0x0F00)>>8) as usize] != (opcode & 0x00FF) as u8 {
                    self.PC += 2;
                }
            },
            0x5000 => {
                // Skip next instruction if Vx = Vy <-(5xy0)
                if self.V[((opcode & 0x0F00)>>8) as usize] == self.V[((opcode & 0x00F0)>>4) as usize] {
                    self.PC += 2;
                }
            },
            0x6000 => {
                // 6xkk - LD Vx, byte
                // Set Vx = kk.
                self.V[((opcode & 0x0F00)>>8) as usize] = (opcode & 0x00FF) as u8;
            },
            0x7000 => {
                // 7xkk - ADD Vx, byte
                // Set Vx = Vx + kk.
                self.V[((opcode & 0x0F00)>>8) as usize] += (opcode & 0x00FF) as u8
            }

            _ => {
                // do nothing
                // unsupported opcode
            }

        }
    }
}

