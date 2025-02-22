use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Status: u8 {
        /*
            The carry flag is set if the last operation caused an overflow from 
            bit 8 of the result or an underflow from bit 0. This condition is set
            during arithmetic, comparison and during logical shifts. It can be 
            explicitly set using the 'Set Carry Flag' (SEC) instruction and 
            cleared with the 'Clear Carry Flag' (CLC).
        */
        const CARRY = 1 << 0;

        /*
            The zero flag is set if the result of the last operation was zero.
        */
        const ZERO = 1 << 1;

        /*
            The interrupt disable flag is set if the program has executed a 
            'Set Interrupt Disable' (SEI) instruction. While this flag is set the 
            processor will not respond to interrupts from devices until it is 
            cleared by a 'Clear Interrupt Disable' (CLI) instruction.
        */
        const INTERRUPT_DISABLE = 1 << 2;

        /*
            While the decimal mode flag is set the processor will obey the rules of
            Binary Coded Decimal (BCD) arithmetic during addition and subtraction. 
            The flag can be explicitly set using 'Set Decimal Flag' (SED) and 
            cleared with 'Clear Decimal Flag' (CLD).
        */
        const DECIMAL = 1 << 3;

        /*
            The break command bit is set when a BRK instruction has been executed 
            and an interrupt has been generated to process it.
        */
        const BREAK = 1 << 4;

        /*
            In the NES CPU (Obelisk 6502) this bit is always 1 for historical 
            purposes and compatibility.
        */
        const UNUSED = 1 << 5;

        /*
            The overflow flag is set during arithmetic operations if the result has
            yielded an invalid 2's complement result (e.g. addint to positive 
            numbers and ending up with a negative result: 64 + 64 => -128). It is
            determined by looking at the carry between bits 6 and 7 and between bit
            7 and the carry flag.
        */
        const OVERFLOW = 1 << 6;

        /*
            The negative flag is set if the result of the last operation had bit 7
            set to a one.
        */
        const NEGATIVE = 1 << 7;
    }
}