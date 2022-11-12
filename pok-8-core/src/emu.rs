use crate::instruction::*;
use crate::speaker::*;

use rand::random;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
const START_ADDR: u16 = 0x200;

const FONTSET_SIZE: usize = 80;

// TODO: Refactor fontset into a 2D array
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
    buzzer: Buzzer,
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
            buzzer: Buzzer::init(),
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
        let byte = self.fetch();

        // Decode
        let op = self.decode(byte);

        // Execute
        self.execute(op);
    }

    pub fn tick_timers(&mut self) -> () {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                self.buzzer.play();
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

    fn clear_screen(&mut self) -> () {
        // change all screen bits to 0
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
    }

    // Push value to CPU stack
    fn stack_push(&mut self, val: u16) -> () {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    // Pop value from CPU stack
    fn stack_pop(&mut self) -> u16 {
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

    fn decode(&mut self, byte: u16) -> Instruction {
        //unpack the digits so we can pattern match them
        let dig1 = (byte & 0xF000) >> 12;
        let dig2 = (byte & 0x0F00) >> 8;
        let dig3 = (byte & 0x00F0) >> 4;
        let dig4 = byte & 0x000F;

        match (dig1, dig2, dig3, dig4) {
            (0, 0, 0, 0) => Instruction::NOP,

            (0, 0, 0xE, 0) => Instruction::CLR,

            (0, 0, 0xE, 0xE) => Instruction::RET,

            (1, _, _, _) => Instruction::JMP(byte & 0xFFF),

            (2, _, _, _) => Instruction::CALL(byte & 0xFFF),

            (3, _, _, _) => Instruction::SKIPIFVNN(dig2, byte & 0xFF),

            (4, _, _, _) => Instruction::SKIPIFNOTVNN(dig2, byte & 0xFF),

            (5, _, _, 0) => Instruction::SKIPIFVV(dig2, dig3),

            (6, _, _, _) => Instruction::SETVNN(dig2, byte & 0xFF),

            (7, _, _, _) => Instruction::INCSETVNN(dig2, byte & 0xFF),

            (8, _, _, 0) => Instruction::SETVV(dig2, dig3),

            (8, _, _, 1) => Instruction::ORSETVV(dig2, dig3),

            (8, _, _, 2) => Instruction::ANDSETVV(dig2, dig3),

            (8, _, _, 3) => Instruction::XORSETVV(dig2, dig3),

            (8, _, _, 4) => Instruction::INCSETVV(dig2, dig3),

            (8, _, _, 5) => Instruction::DECSETVV(dig2, dig3),

            (8, _, _, 6) => Instruction::SHIFTRV(dig2),

            (8, _, _, 7) => Instruction::DIFFSETVV(dig2, dig3),

            (8, _, _, 0xE) => Instruction::SHIFTLV(dig2),

            (9, _, _, 0) => Instruction::SKIPIFNOTVV(dig2, dig3),

            (0xA, _, _, _) => Instruction::SETINNN(byte & 0xFFF),

            (0xB, _, _, _) => Instruction::JMPV(byte & 0xfFF),

            (0xC, _, _, _) => Instruction::RAND(dig2, byte & 0xFF),

            (0xD, _, _, _) => Instruction::DRAW(dig2, dig3, dig4),

            (0xE, _, 9, 0xE) => Instruction::SKIPIFKEY(dig2),

            (0xE, _, 0xA, 1) => Instruction::SKIPIFNOTKEY(dig2),

            (0xF, _, 0, 7) => Instruction::SETVDT(dig2),

            (0xF, _, 0, 0xA) => Instruction::WAITFORKEY(dig2),

            (0xF, _, 1, 5) => Instruction::SETDTV(dig2),

            (0xF, _, 1, 8) => Instruction::SETSTV(dig2),

            (0xF, _, 1, 0xE) => Instruction::INCSETIV(dig2),

            (0xF, _, 2, 9) => Instruction::SETIFONT(dig2),

            (0xF, _, 3, 3) => Instruction::BCDTORAM(dig2),

            (0xF, _, 5, 5) => Instruction::VTORAM(dig2),

            (0xF, _, 6, 5) => Instruction::RAMTOV(dig2),

            (_, _, _, _) => unimplemented!(
                "DECODING: Error parsing unknown byte {:#02X} into byte code",
                byte
            ),
        }
    }

    fn execute(&mut self, op: Instruction) -> () {
        match op {
            Instruction::NOP => return,

            Instruction::CLR => self.clear_screen(),

            Instruction::RET => {
                self.pc = self.stack_pop();
            }

            Instruction::JMP(address) => {
                self.pc = address;
            }

            Instruction::CALL(address) => {
                self.stack_push(self.pc);
                self.pc = address;
            }

            Instruction::SKIPIFVNN(x, nn) => {
                if self.v_reg[x as usize] == nn as u8 {
                    self.pc += 2;
                }
            }

            Instruction::SKIPIFNOTVNN(x, nn) => {
                if self.v_reg[x as usize] != nn as u8 {
                    self.pc += 2;
                }
            }

            Instruction::SKIPIFVV(x, y) => {
                if self.v_reg[x as usize] == self.v_reg[y as usize] {
                    self.pc += 2;
                }
            }

            Instruction::SETVNN(x, nn) => {
                self.v_reg[x as usize] = nn as u8;
            }

            Instruction::INCSETVNN(x, nn) => {
                self.v_reg[x as usize] = self.v_reg[x as usize].wrapping_add(nn as u8);
                //wrapping avoid panic at overflow, VF not set
            }

            Instruction::SETVV(x, y) => {
                self.v_reg[x as usize] = self.v_reg[y as usize];
            }

            Instruction::ORSETVV(x, y) => {
                self.v_reg[x as usize] |= self.v_reg[y as usize];
            }

            Instruction::ANDSETVV(x, y) => {
                self.v_reg[x as usize] &= self.v_reg[y as usize];
            }

            Instruction::XORSETVV(x, y) => {
                self.v_reg[x as usize] ^= self.v_reg[y as usize];
            }

            Instruction::INCSETVV(x, y) => {
                let (new_vx, carry) =
                    self.v_reg[x as usize].overflowing_add(self.v_reg[y as usize]);
                let new_vf = if carry { 1 } else { 0 }; // set VF if overflow

                self.v_reg[x as usize] = new_vx;
                self.v_reg[0xF] = new_vf;
            }

            Instruction::DECSETVV(x, y) => {
                let (new_vx, borrow) =
                    self.v_reg[x as usize].overflowing_sub(self.v_reg[y as usize]);
                let new_vf = if borrow { 0 } else { 1 }; // reset VF if borrow

                self.v_reg[x as usize] = new_vx;
                self.v_reg[0xF] = new_vf;
            }

            Instruction::SHIFTRV(x) => {
                self.v_reg[0xF] = self.v_reg[x as usize] & 1;
                self.v_reg[x as usize] >>= 1;
            }

            Instruction::DIFFSETVV(x, y) => {
                let (new_vx, borrow) =
                    self.v_reg[y as usize].overflowing_sub(self.v_reg[x as usize]);
                let new_vf = if borrow { 0 } else { 1 }; // reset if borrow

                self.v_reg[x as usize] = new_vx;
                self.v_reg[0xF] = new_vf;
            }

            Instruction::SHIFTLV(x) => {
                self.v_reg[0xF] = (self.v_reg[x as usize] >> 1) & 1;
                self.v_reg[x as usize] <<= 1;
            }

            Instruction::SKIPIFNOTVV(x, y) => {
                if self.v_reg[x as usize] != self.v_reg[y as usize] {
                    self.pc += 2;
                }
            }

            Instruction::SETINNN(nnn) => {
                self.i_reg = nnn;
            }

            Instruction::JMPV(nnn) => {
                self.pc = (self.v_reg[0] as u16) + nnn;
            }

            Instruction::RAND(x, nn) => {
                let rng: u8 = random();
                self.v_reg[x as usize] = rng & (nn as u8);
            }

            // DXYN - Draw Sprite, XY are VX/VY coord of sprite, N is the height of sprite
            // TODO: loook at this algorithm
            Instruction::DRAW(x, y, n) => {
                let x_coord = self.v_reg[x as usize] as u16;
                let y_coord = self.v_reg[y as usize] as u16;

                let num_rows = n;

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
            }

            Instruction::SKIPIFKEY(x) => {
                let key = self.keys[self.v_reg[x as usize] as usize];
                if key {
                    self.pc += 2;
                }
            }

            Instruction::SKIPIFNOTKEY(x) => {
                let key = self.keys[self.v_reg[x as usize] as usize];
                if !key {
                    self.pc += 2;
                }
            }

            Instruction::SETVDT(x) => {
                self.v_reg[x as usize] = self.dt;
            }

            Instruction::WAITFORKEY(x) => {
                // BLOCKING

                let mut pressed = false;
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_reg[x as usize] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    // redo opcode until key presesd
                    self.pc -= 2;
                }
            }

            Instruction::SETDTV(x) => {
                self.dt = self.v_reg[x as usize];
            }

            Instruction::SETSTV(x) => {
                self.st = self.v_reg[x as usize];
            }

            Instruction::INCSETIV(x) => {
                self.i_reg = self.i_reg.wrapping_add(self.v_reg[x as usize] as u16);
            }

            Instruction::SETIFONT(x) => {
                let c = self.v_reg[x as usize] as u16;
                self.i_reg = c * 5; // 5c is the beginning of the character
                                    // TODO: Look at this when refactoring the Fontset =
            }

            Instruction::BCDTORAM(x) => {
                let bcd = Self::double_dabble(&self.v_reg[x as usize]);

                // since VX is a u8 it ranges from 0 to 255, it will always be three digits
                for (i, bin) in bcd.iter().enumerate() {
                    self.ram[self.i_reg as usize + i] = *bin;
                }
            }

            // FX55 - Store V0 - VX into I
            Instruction::VTORAM(x) => {
                for idx in 0..=x {
                    self.ram[self.i_reg as usize + idx as usize] = self.v_reg[idx as usize];
                }
            }

            // FX65 - Load I into V0 - VX
            Instruction::RAMTOV(x) => {
                for idx in 0..=x {
                    self.v_reg[idx as usize] = self.ram[self.i_reg as usize + idx as usize];
                }
            }
        }
    }

    pub fn double_dabble(num: &u8) -> [u8; 3] {
        // TODO: Improve the writing of this algorithm

        // implementation of the double dable algorithm for finding BCD
        // 1st iteration: 7ns per cycle compared to the book's 25ns
        // 2nd iteration: 6ns per cycle compared to the book's 25ns

        let mut num: u32 = (0 | num) as u32; // convert to 32-bit num for padding putting the original number at the end, need 4bits for every digit (20 in this case)

        // masks to extract relevant digit from number
        let mask = 0b1111;

        for _ in 0..8 {
            // algo will only shift length of num times (8 times)

            for i in 0..3 { //here checking each digit there are only three as num(u8) has the limi 255

                let offset = 4*(2+i); // adds the requisite number of zeroes to target the desired digit
                
                if num &  ( mask << offset ) >= (5 << offset) {
                    num += 3 << offset //add three to the target digit
                }
            }

            num <<= 1; //left shift once
        }

        let mut res = [0; 3];

        for i in 0..3 {

            let offset =  4*(2+i); // distacnce to left shift to mask the correct digit
            let bottom = 4*(4-i); // distance to right shift to extract the correct 4 bits

            res[i] = ((num & (mask << offset)) >> bottom) as u8;
        }

        res
    }
}
