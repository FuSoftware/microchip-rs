/*
100	(zero page),Y
101	zero page,X
110	absolute,Y
111	absolute,X
*/

enum MOS6502_ADDR {
    // 000 (zero page,X)
    // Indexed indirect addressing is normally used in conjunction with a table of address held on zero page. 
    // The address of the table is taken from the instruction and the X register added to it 
    // (with zero page wrap around) to give the location of the least significant byte of the target address.
    INDEXED_INDIRECT_X,

    // 001 zero page
    // An instruction using zero page addressing mode has only an 8 bit address operand. 
    // This limits it to addressing only the first 256 bytes of memory (e.g. $0000 to $00FF) where the most significant byte of the address is always zero. 
    // In zero page mode only the least significant byte of the address is held in the instruction making it shorter by one byte (important for space saving)
    // and one less memory fetch during execution (important for speed).
    ZERO_PAGE,

    // 010 #immediate
    // Immediate addressing allows the programmer to directly specify an 8 bit constant within the instruction. 
    // It is indicated by a '#' symbol followed by an numeric expression.
    IMMEDIATE,

    // 011 absolute
    // Instructions using absolute addressing contain a full 16 bit address to identify the target location.
    ABSOLUTE,

    // 100 (zero page),Y
    // Indirect indirect addressing is the most common indirection mode used on the 6502. 
    // In instruction contains the zero page location of the least significant byte of 16 bit address. 
    // The Y register is dynamically added to this value to generated the actual target address for operation.
    INDIRECT_INDEXED_Y,

    // 101	zero page,X
    // The address to be accessed by an instruction using indexed zero page addressing is calculated by taking the 8 bit zero page address from the instruction and adding the current value of the X register to it. 
    // For example if the X register contains $0F and the instruction LDA $80,X is executed then the accumulator will be loaded from $008F (e.g. $80 + $0F => $8F).
    ZERO_PAGE_X,

    // 110 absolute,Y
    // The Y register indexed absolute addressing mode is the same as the previous mode only with the contents of the 
    // Y register added to the 16 bit address from the instruction.
    ABSOLUTE_Y,

    // 111 absolute,X
    // The address to be accessed by an instruction using X register indexed absolute addressing is computed by taking the 16 bit address from the instruction and added the contents of the X register. 
    // For example if X contains $92 then an STA $2000,X instruction will store the accumulator at $2092 (e.g. $2000 + $92).
    ABSOLUTE_X,
}

struct MOS6502 {

}

impl MOS6502 {
    pub fn new() -> MOS6502 {
        MOS6502 {
            
        }
    }

    pub fn next_instruction(&mut self) {

    }

    pub fn fetch(&mut self, addr: MOS6502_ADDR) -> u8 {
        todo!();
    }

    pub fn opcode_dispatch(&mut self, opcode: u8) {
        match opcode & 0b11 {
            0 => {

            }

            1 => {

            }

            2 => {

            }

            3 => {

            }
        }
    }

    // Logical OR between memory adress and the accumulator, stored back into the accumulator
    pub fn op_ora(&mut self) {

    }

    pub fn op_and(&mut self) {

    }
    
    pub fn op_eor(&mut self) {

    }
    
    pub fn op_adc(&mut self) {

    }
    
    pub fn op_sta(&mut self) {

    }
    
    pub fn op_lda(&mut self) {

    }
    
    pub fn op_cmp(&mut self) {

    }
    
    pub fn op_sbc(&mut self) {

    }
    
}