use rand::random;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
const START_ADDR: u16 = 0x200;

const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

// Core Emulator Structure
pub struct Emu {
    pc: u16,                                      // one byte program counter
    ram: [u8; RAM_SIZE],                          // 4 kilobytes of ram
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT], // screen of "single bits"
    v_reg: [u8; NUM_REGS],                        // V REgisters
    i_reg: u16,                                   // indexing register
    sp: u16,                                      // 2byte stack pointer
    stack: [u16; STACK_SIZE], // CPU LIFO stack (could be a Deque but CHIP would've used this sp system)
    keys: [bool; NUM_KEYS],   // input handling
    dt: u8,                   // Delay Timer
    st: u8,                   //Sound Timer
}

impl Emu {
    pub fn init() -> Self {
        let mut new_emu = Self {
            pc: START_ADDR, // CHIP-8 standard for program start
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        };
        new_emu.ram[0..FONTSET_SIZE].copy_from_slice(&FONTSET);

        new_emu
    }

    pub fn reset(&mut self) -> () {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[0..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    // Main Intepreter Cycle => Fetch -> Decode -> Execute
    pub fn cycle(&mut self) -> () {
        // Fetch
        let op = self.fetch();

        // Decode and Execute
        self.execute(op);
    }

    pub fn tick_timers(&mut self) -> () {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // TRIGGER SOUND
            }
            self.st -= 1;
        }
    }

    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }

    pub fn key_down(&mut self, idx: usize) -> () {
        self.keys[idx] = true;
    }

    pub fn key_up(&mut self, idx: usize) -> () {
        self.keys[idx] = false;
    }

    pub fn load(&mut self, data: &[u8]) -> () {
        self.ram[(START_ADDR as usize)..((START_ADDR as usize) + data.len())].copy_from_slice(data);
    }
}

impl Emu {
    // Push value to CPU stack
    fn push(&mut self, val: u16) -> () {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    // Pop value from CPU stack
    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    fn fetch(&mut self) -> u16 {
        // retrieve the two byte instruction from the ram
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;

        //bitshift + or to combine the bytes
        let op = (higher_byte << 8) | lower_byte;

        //move the pc
        self.pc += 2;

        op
    }

    fn execute(&mut self, op: u16) -> () {
        //unpack the digits so we can pattern match them
        let dig1 = (op & 0xF000) >> 12;
        let dig2 = (op & 0x0F00) >> 8;
        let dig3 = (op & 0x00F0) >> 4;
        let dig4 = op & 0x000F;

        // Process and execute op code
        match (dig1, dig2, dig3, dig4) {
            // 0x0000 NOP - do nothing
            (0, 0, 0, 0) => return,

            // 0x00E0 - Clear screen
            (0, 0, 0xE, 0) => self.screen = [false; SCREEN_HEIGHT * SCREEN_WIDTH],

            // 0x00EE - Return from Sub routine
            (0, 0, 0xE, 0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            },

            // 0x1NNN - Jump to address NNN
            (1, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = nnn;
            },

            // 0x2NNN - Call subroutine at NNN
            (2, _, _, _) => {
                let nnn = op & 0xFFF;
                self.push(self.pc);
                self.pc = nnn;
            },

            // 0x2XNN - Skip Next if VX == NN
            (3, _, _, _) => {
                let x = dig2 as usize;
                let nn = (op & 0xFF) as u8;

                if self.v_reg[x] == nn {
                    self.pc += 2;
                }
            },

            // 0x4XNN - Skip Next if VX !== NN
            (4, _, _, _) => {
                let x = dig2 as usize;
                let nn = (op & 0xFF) as u8;

                if self.v_reg[x] != nn {
                    self.pc += 2;
                }
            },

            // 0x5XY0 - Skip Next if VX == VY
            (5, _, _, 0) => {
                let x = dig2 as usize;
                let y = dig3 as usize;

                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            },

            // 0x6XNN - VX == NN
            (6, _, _, _) => {
                let x = dig2 as usize;
                let nn = (op & 0xFF) as u8;

                self.v_reg[x] = nn;
            },

            // 0x7XNN - VX += NN, wrapping avoid panic at overflow
            (7, _, _, _) => {
                let x = dig2 as usize;
                let nn = (op & 0xFF) as u8;

                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            },

            // 0x8XY0 - VX = VY
            (8, _, _, 0) => {
                let x = dig2 as usize;
                let y = dig3 as usize;

                self.v_reg[x] = self.v_reg[y];
            },

            // 0x8XY1 - Bitwise OP VX |= VY
            (8, _, _, 1) => {
                let x = dig2 as usize;
                let y = dig3 as usize;

                self.v_reg[x] |= self.v_reg[y];
            },

            // 0x8XY2 - Bitwise OP VX &= VY
            (8, _, _, 2) => {
                let x = dig2 as usize;
                let y = dig3 as usize;

                self.v_reg[x] &= self.v_reg[y];
            },

            // 0x8XY3 - Bitwise OP VX ^= VY
            (8, _, _, 3) => {
                let x = dig2 as usize;
                let y = dig3 as usize;

                self.v_reg[x] ^= self.v_reg[y];
            },

            // 0x8XY4 - VX += VY, set VF to 1 in case of overload
            (8, _, _, 4) => {
                let x = dig2 as usize;
                let y = dig3 as usize;

                let (new_vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let new_vf = if carry { 1 } else { 0 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            },

            // 0x8XY5 - VX -= VY, set VF to 0 in case of underflow
            (8, _, _, 5) => {
                let x = dig2 as usize;
                let y = dig3 as usize;

                let (new_vx, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                let new_vf = if borrow { 0 } else { 1 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            },

            // 0x8XY6 - VX >>= 1, Right Shift VX by 1, least significant bit is stored in VF
            (8, _, _, 6) => {
                let x = dig2 as usize;
                let lsb = self.v_reg[x] & 1;

                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb;
            },

            // 0x8XY7 - VX = VY - VX, set VF to 0 in case of underflow
            (8, _, _, 7) => {
                let x = dig2 as usize;
                let y = dig3 as usize;

                let (new_vx, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                let new_vf = if borrow { 0 } else { 1 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            },

            // 0x8XYE - VX <<= 1, Most significant bit is sotred in VF
            (8, _, _, 0xE) => {
                let x = dig2 as usize;
                let msb = (self.v_reg[x] >> 1) & 1;

                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;
            },

            // 0x9XY0 - Skip Next if VX != VY
            (9, _, _, 0) => {
                let x = dig2 as usize;
                let y = dig3 as usize;

                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }
            },

            // ANNN - I = NNN
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;

                self.i_reg = nnn;
            },

            // BNNN - Jump to V0 + NNN
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;

                self.pc = (self.v_reg[0] as u16) + nnn;
            },

            // CXNN - VX = rand() and NN
            (0xC, _, _, _) => {
                let x = dig2 as usize;
                let nn = (op & 0xFF) as u8;
                let rng: u8 = random();

                self.v_reg[x] = rng & nn;
            },

            // DXYN - Draw SPrite, XY are VX/VY coord of sprite, N is the height of sprite
            (0xD, _, _, _) => {
                let x_coord = self.v_reg[dig2 as usize] as u16;
                let y_coord = self.v_reg[dig3 as usize] as u16;

                let num_rows = dig4;

                let mut flipped = false;
                for y_line in 0..num_rows {
                    let addr = self.i_reg + y_line as u16;
                    let pixels = self.ram[addr as usize];

                    for x_line in 0..8 {
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                            let idx = x + SCREEN_WIDTH * y;

                            flipped |= self.screen[idx];
                            self.screen[idx] ^= true;
                        }
                    }
                }

                if flipped {
                    self.v_reg[0xF] = 1;
                } else {
                    self.v_reg[0xF] = 0;
                }
            },

            // EX0E - Skip if key in VX pressed
            (0xE, _, 9, 0xE) => {
                let x = dig2 as usize;
                let vx = self.v_reg[x];
                let key = self.keys[vx as usize];

                if key {
                    self.pc += 2;
                }
            },

            // EXA1 - Skip if key in VX not pressed
            (0xE, _, 0xA, 1) => {
                let x = dig2 as usize;
                let vx = self.v_reg[x];
                let key = self.keys[vx as usize];

                if !key {
                    self.pc += 2;
                }
            },

            // FX07 - VX = Delay Timer
            (0xF, _, 0, 7) => {
                let x = dig2 as usize;
                self.v_reg[x] = self.dt;
            },

            // EX0A - Wait for Key press
            (0xF, _, 0, 0xA) => {
                let x = dig2 as usize;
                let mut pressed = false;
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_reg[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    // redo opcode until key presesd
                    self.pc -= 2;
                }
            },

            // FX15 - DT = VX
            (0xF, _, 1, 5) => {
                let x = dig2 as usize;
                self.dt = self.v_reg[x];
            },
            
            // FX18 - ST = VX
            (0xF, _, 1, 8) => {
                let x = dig2 as usize;
                self.st = self.v_reg[x];
            },
            
            // FX1E - I += VX
            (0xF, _, 1, 0xE) => {
                let x = dig2 as usize;
                let vx = self.v_reg[x] as u16;
                self.i_reg = self.i_reg.wrapping_add(vx);
            },
            
            // FX29 - Set I to Font Address
            (0xF, _, 2, 9) => {

                let x = dig2 as usize;
                let c = self.v_reg[x] as u16;
                self.i_reg = c * 5;
            },

            // FX33 - RAM[I] = Binary Coded Decimal of VX
            (0xF, _, 3, 3) => {
                // since VX is a u8 it ranges from 0 to 255, it will always be three digits
                let x = dig2 as usize;
                let vx = self.v_reg[x];

                let bcd = Self::double_dabble(&vx);

                self.ram[self.i_reg as usize] = bcd[0];
                self.ram[(self.i_reg + 1) as usize] = bcd[1];
                self.ram[(self.i_reg + 2) as usize] = bcd[2];
            },

            // FX55 - Store V0 - VX into I
            (0xF, _, 5, 5) => {

                let x = dig2 as usize;
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.ram[i + idx] = self.v_reg[idx];
                }
            },
            
            // FX65 - Load I into V0 - VX
            (0xF, _, 6, 5) => {

                let x = dig2 as usize;
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.v_reg[idx] = self.ram[i + idx];
                }
            },
            
            (_, _, _, _) => unimplemented!("Unimplemented opcode {:#02x}", op),
        }
    }

    pub fn double_dabble(num: &u8) -> [u8; 3] {
        // implementation of the double dable algorithm for finding BCD
        // 7ns per cycle compared to the book's 25ns
        
        let mut res: u32 = (0 | num) as u32; // convert to 32-bit num for padding, need 4bits for every digit (20 in this case)
        
        // masks to extract relevant digit from number
        let ones_mask = 0b1111_0000_0000; 
        let tens_mask = 0b1111_0000_0000_0000;
        let huns_mask = 0b1111_0000_0000_0000_0000;
        
        for _ in 0..8 { // algo will only shift length of num times (8 times)
            
            if res & ones_mask >= 0b0101_0000_0000 {
                
                res += 0b0011_0000_0000; // if ones digit is greater than or equal to 5 add 3 to THAT digit
            }
            if res & tens_mask >= 0b0101_0000_0000_0000 {
                
                res += 0b0011_0000_0000_0000;
            }
            if res & huns_mask >= 0b0101_0000_0000_0000_0000 {
                
                res += 0b0011_0000_0000_0000_0000;
            }
            
            res <<= 1;
        }
        
        let huns = (res & huns_mask) >> 16;
        let tens = (res & tens_mask) >> 12;
        let ones = (res & ones_mask) >> 8;

        [huns as u8, tens as u8, ones as u8]
    }
}

