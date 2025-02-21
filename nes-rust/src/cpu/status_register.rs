pub struct StatusRegister(u8);

/*
    Helper struct that makes it clear which bits are being set within the 
    processor status register of the 6502 CPU.
 */
impl StatusRegister {
    /*
        The carry flag is set if the last operation caused an overflow from 
        bit 8 of the result or an underflow from bit 0. This condition is set
        during arithmetic, comparison and during logical shifts. It can be 
        explicitly set using the 'Set Carry Flag' (SEC) instruction and 
        cleared with the 'Clear Carry Flag' (CLC).
     */
    pub const CARRY: u8 = 1 << 0;

    /*
        The zero flag is set if the result of the last operation was zero.
     */
    pub const ZERO: u8 = 1 << 1;

    /*
        The interrupt disable flag is set if the program has executed a 
        'Set Interrupt Disable' (SEI) instruction. While this flag is set the 
        processor will not respond to interrupts from devices until it is 
        cleared by a 'Clear Interrupt Disable' (CLI) instruction.
     */
    pub const INTERRUPT_DISABLE: u8 = 1 << 2;

    /*
        While the decimal mode flag is set the processor will obey the rules of
        Binary Coded Decimal (BCD) arithmetic during addition and subtraction. 
        The flag can be explicitly set using 'Set Decimal Flag' (SED) and 
        cleared with 'Clear Decimal Flag' (CLD).
     */
    pub const DECIMAL: u8 = 1 << 3;

    /*
        The break command bit is set when a BRK instruction has been executed 
        and an interrupt has been generated to process it.
     */
    pub const BREAK: u8 = 1 << 4;

    /*
        In the NES CPU (Obelisk 6502) this bit is always 1 for historical 
        purposes and compatibility.
     */
    pub const UNUSED: u8 = 1 << 5;

    /*
        The overflow flag is set during arithmetic operations if the result has
        yielded an invalid 2's complement result (e.g. addint to positive 
        numbers and ending up with a negative result: 64 + 64 => -128). It is
        determined by looking at the carry between bits 6 and 7 and between bit
        7 and the carry flag.
     */
    pub const OVERFLOW: u8 = 1 << 6;

    /*
        The negative flag is set if the result of the last operation had bit 7
        set to a one.
     */
    pub const NEGATIVE: u8 = 1 << 7;

    /*
        Note:

        The use of the #[inline(always)] attributes utilized below guarantees
        that these functions will be inlined by the rust compiler, meaning that
        there is little if no overhead to this implementation compared to using
        a raw u8 value to store the processor status.
     */

    // Clears a particular flag in the status register.
    #[inline(always)]
    pub fn clear(&mut self, flag: u8) {
        self.0 &= !flag;
    }

    // Determines if the indicated flag is set in the status register.
    #[inline(always)]
    pub fn is_set(&mut self, flag:u8) -> bool {
        self.0 & flag != 0
    }

    // Creates a new StatusRegister struct.
    #[inline(always)]
    pub fn new() -> Self {
        StatusRegister(Self::UNUSED) // Ensures that bit 5 is always 1
    }

    // Sets a particular flag in the status register.
    #[inline(always)]
    pub fn set(&mut self, flag: u8) {
        self.0 |= flag;
    }

    pub fn get(&mut self) -> u8 {
        self.0
    }

    pub fn reset(&mut self) {
        self.0 = Self::UNUSED;
    }
}

