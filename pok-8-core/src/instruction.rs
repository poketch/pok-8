
pub enum Instruction {

    NOP, // No Op
    CLR, // Clear Screen
    RET, // Return
    JMP(u16), // 1NNN - Jump to Address NNN
    CALL(u16), // 2NNN - Call sub at NNN
    SKIPIFVNN(u16, u16), //3XNN - Skip if VX == 0xNN
    SKIPIFNOTVNN(u16, u16), //4XNN - Skip if VX == 0xNN
    SKIPIFVV(u16, u16), // 5XY0 - Skip if VX == VY
    SETVNN(u16, u16), // 6XNN - VX = 0xNN
    INCSETVNN(u16, u16), // 7XNN - VX += 0xNN (Doesn't affect carry flag)
    SETVV(u16, u16), // 8XY0 - VX = VY
    ORSETVV(u16, u16), // 8XY1 - VX |= VY
    ANDSETVV(u16, u16), // 8XY2 - VX &= VY
    XORSETVV(u16, u16), // 8XY3 - VX ^= VY
    INCSETVV(u16, u16), // 8XY4 - VX += VY, Sets VF if carry
    DECSETVV(u16, u16), // 8XY5 - VX -= VY, Clear VF if carry
    SHIFTRV(u16), // 8XY6 - VX >>= 1, Dropped bit in VF
    DIFFSETVV(u16, u16), // 8XY7 VX = VY - VX, Clear VF if borrow
    SHIFTLV(u16), // 8XYE VX <<= 1, Store dropped bit in VF
    SKIPIFNOTVV(u16, u16), // 9XY0 - Skip if VX != VY
    SETINNN(u16), // ANNN - I = NNN
    JMPV(u16), // BNNN - Jump to V0 + 0xNNN
    RAND(u16, u16), // CXNN - VX = rand() & 0xNN
    DRAW(u16, u16, u16), // DXYN - Draw sprite at (VX, VY) N pixels tall, on/off based on I. VF set if any pixels flipped
    SKIPIFKEY(u16), // EX9E - Skip if key index in VX is pressed =
    SKIPIFNOTKEY(u16), // EXA1 - Skip if key at VX is not pressed
    SETVDT(u16), // FX07 - VX = Delay Timer
    WAITFORKEY(u16), // FX0A - Wait for key, index in VX, BLOCKING
    SETDTV(u16), // FX15 - Delay Timer = VX
    SETSTV(u16), // FX18 - Sound Timer = VX
    INCSETIV(u16), // FX1E - I += VX
    SETIFONT(u16), // FX29 - Set I to the font char in VX
    BCDTORAM(u16), // FX33 - stores BCD of VX into RAM[I]
    VTORAM(u16), // FX55 - V0 - VX into RAM starting at RAM[I], Inclusive Range
    RAMTOV(u16), // FX65 - RaM into V registers starting with RAM[I], Inclusive
    
}