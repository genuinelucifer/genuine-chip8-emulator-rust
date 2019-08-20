extern crate rand;

use rand::Rng;

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
        println!("Bytes received: 0x{:x}, 0x{:x}", byte0, byte1);
        self.PC += 2;
        if byte0 == 0 && byte1 == 0 {
            // exit
            std::process::exit(0);
        }
        let opcode = (byte0 as u16)<<8 | (byte1 as u16);
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
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
                if self.V[x] == (opcode & 0x00FF) as u8 {
                    self.PC += 2;
                }
            },
            0x4000 => {
                // Skip next instruction if Vx != kk <-(4xkk)
                if self.V[x] != (opcode & 0x00FF) as u8 {
                    self.PC += 2;
                }
            },
            0x5000 => {
                // Skip next instruction if Vx = Vy <-(5xy0)
                if self.V[x] == self.V[y] {
                    self.PC += 2;
                }
            },
            0x6000 => {
                // 6xkk - LD Vx, byte
                // Set Vx = kk.
                self.V[x] = (opcode & 0x00FF) as u8;
            },
            0x7000 => {
                // 7xkk - ADD Vx, byte
                // Set Vx = Vx + kk.
                self.V[x] += (opcode & 0x00FF) as u8
            },
            0x8000 => {
                // 8XYZ

                match opcode & 0x000F {
                    0x0000 => {
                        //8XY0 	Assign 	Vx=Vy 	Sets VX to the value of VY.
                        self.V[x] = self.V[y];
                    },
                    0x0001 => {
                        //8XY1 	BitOp 	Vx=Vx|Vy 	Sets VX to VX or VY. (Bitwise OR operation)
                        self.V[x] |= self.V[y];
                    },
                    0x0002 => {
                        //8XY2 	BitOp 	Vx=Vx&Vy 	Sets VX to VX and VY. (Bitwise AND operation)
                        self.V[x] &= self.V[y];
                    },
                    0x0003 => {
                        //8XY3 	BitOp 	Vx=Vx^Vy 	Sets VX to VX xor VY.
                        self.V[x] ^= self.V[y];
                    },
                    0x0004 => {
                        //8XY4 	Math 	Vx += Vy 	Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
                        let u16sum = (self.V[x] as u16) + (self.V[y] as u16);
                        self.V[x] += self.V[y];
                        // set the carry flag
                        if u16sum > (self.V[x] as u16) {
                            self.V[0x000F] = 1;
                        }
                        else {
                            self.V[0x000F] = 0;
                        }
                    },
                    0x0005 => {
                        //8XY5  Math 	Vx -= Vy 	VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
                        self.V[x] -= self.V[y];
                        // A borrow when self.V[x] < self.V[y]
                        if self.V[x] < self.V[y] {
                            self.V[0x000F] = 0;
                        }
                        else {
                            self.V[0x000F] = 1;
                        }
                    },
                    0x0006 => {
                        //8XY6 	BitOp 	Vx>>=1 	Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
                        self.V[x] >>= 1;
                    },
                    0x0007 => {
                        //8XY7 	Math 	Vx=Vy-Vx 	Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
                        self.V[x] = self.V[y] - self.V[x];
                        // A borrow when self.V[y] < self.V[x]
                        if self.V[y] < self.V[x] {
                            self.V[0x000F] = 0;
                        }
                        else {
                            self.V[0x000F] = 1;
                        }
                    },
                    0x000E => {
                        //8XYE 	BitOp 	Vx<<=1 	Stores the most significant bit of VX in VF and then shifts VX to the left by 1.
                        self.V[x] <<= 1;
                    },
                    _ => {
                        // do nothing
                        // unsupported opcode
                    }
                }
            },
            0x9000 => {
                if (opcode & 0x000F) == 0 {
                    //9XY0 	Cond 	if(Vx!=Vy) 	Skips the next instruction if VX doesn't equal VY. (Usually the next instruction is a jump to skip a code block)
                    if self.V[x] != self.V[y] {
                        self.PC += 2;
                    }
                }
                else {
                        // do nothing
                        // unsupported opcode
                }
            },
            0xA000 => {
                //ANNN 	MEM 	I = NNN 	Sets I to the address NNN.
                self.I = opcode & 0x0FFF;
            },
            0xB000 => {
                //BNNN 	Flow 	PC=V0+NNN 	Jumps to the address NNN plus V0.
                self.PC = (self.V[0] as u16) + (opcode & 0x0FFF);
            },
            0xC000 => {
                //CXNN 	Rand 	Vx=rand()&NN 	Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let NN = (opcode & 0x00FF) as u8;
                let mut rng = rand::thread_rng();
                let rand_u8: u8 = rng.gen();
                self.V[x] = rand_u8 &  NN;
            },
            0xD000 => {
                //Dxyn - DRW Vx, Vy, nibble
                //Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                let n = opcode & 0x000F;
                let row = self.V[x];
                let column = self.V[y];
                self.V[0xF] = 0;
                for i in 0..n {
                    if self.display.draw_byte(self.MEM.get_word((self.I + i) as usize).0, &x, &y) == true {
                        self.V[0xF] = 1;
                    }
                }
                self.display.update();
            }

             // TODO: Handle unhandled op codes here
            _ => {
                // do nothing
                // unsupported opcode
            }

        }
    }
}

