pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
const START_ADDR: u16 = 0x00;

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
struct Emu {
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
        self. pc = START_ADDR;
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
        let lower_byte = self.ram[(self.pc+1) as usize] as u16;

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
        let dig4 = (op & 0x000F);

        
        // Process and execute op code
        match (dig1, dig2, dig3, dig4) {

        

            (_, _, _, _) => unimplemented!("Unimplemented opcode {}", op) 
        }






    }


}
