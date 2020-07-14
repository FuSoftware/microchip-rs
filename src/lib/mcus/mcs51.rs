pub enum MCS51_INST {
    ACALL,
    ADD,
    ADDC,
    AJMP,
    AJNL,
    CJNE,
    CLR,
    CPL,
    DA,
    DEC,
    DIV,
    DJNZ,
    INC,
    JB,
    JBC,
    JC,
    JMP,
    JNB,
    JNC,
    JNZ,
    JZ,
    LCALL,
    LJMP,
    MOV,
    MOVC,
    MOVX,
    MUL,
    NOP,
    ORL,
    POP,
    PUSH,
    RET,
    RETI,
    RL,
    RLC,
    RR,
    RRC,
    SETB,
    SJMP,
    SUBB,
    SWAP,
    XCH,
    XCHD,
    XRL,
    UNDEFINED
}

pub enum MCS51_ADDRESSING {
    ACCUMULATOR,
    REGISTER(u8),
    DIRECT(u8),
    INDIRECT_Ri(u8),
    DATA(u8),
    DATA_16,
    ADDR_16,
    ADDR_11,
    RELATIVE,
    BIT
}

pub struct Instruction {
    operation: MCS51_INST,
    op1_addressing: MCS51_ADDRESSING,
    cycles: u8,
}

impl Instruction {

}

pub struct MCS51 {
    pc: u16,
    program: Vec<u8>,
    registers: [u8; 8],
    accumulator: u8,
    ram: [u8; 255]
}

impl MCS51 {
    pub fn get_u8_mut(&mut self, addressing: MCS51_ADDRESSING) -> Option<&mut u8> {
        match addressing {
            MCS51_ADDRESSING::ACCUMULATOR => Some(&mut self.accumulator),
            MCS51_ADDRESSING::REGISTER(reg) => self.registers.get_mut(reg as usize),
            MCS51_ADDRESSING::DIRECT(addr) => self.ram.get_mut(addr as usize),
            MCS51_ADDRESSING::INDIRECT_Ri(reg) => self.registers.get_mut(*self.registers.get(reg as usize).unwrap() as usize),
            _ => {
                println!("Unsupported addressing mode");
                return None;
            }
        }
    }

    pub fn get_u8(&self, addressing: MCS51_ADDRESSING) -> Option<u8> {
        match addressing {
            MCS51_ADDRESSING::ACCUMULATOR => Some(self.accumulator),
            MCS51_ADDRESSING::REGISTER(reg) => Some(*self.registers.get(reg as usize).unwrap()),
            MCS51_ADDRESSING::DIRECT(addr) => Some(*self.ram.get(addr as usize).unwrap()),
            MCS51_ADDRESSING::INDIRECT_Ri(reg) => Some(*self.registers.get(*self.registers.get(reg as usize).unwrap() as usize).unwrap()),
            MCS51_ADDRESSING::DATA(offset) => Some(*self.program.get(self.pc as usize + offset as usize).unwrap()),
            _ => {
                println!("Unsupported addressing mode");
                return None;
            }
        }
    }

    pub fn get_u16(&self, addressing: MCS51_ADDRESSING) -> Option<u16> {
        match addressing {
            MCS51_ADDRESSING::DATA(offset) => {
                let hi_byte = *self.program.get(self.pc as usize + offset as usize).unwrap();
                let lo_byte = *self.program.get(self.pc as usize + offset as usize + 1).unwrap();
                let addr: u16 = (hi_byte as u16) << 8 + lo_byte as u16;
                return Some(addr);
            }
            _ => {
                println!("Unsupported addressing mode");
                return None;
            }
        }
    }



    pub fn opcode_dispatch(&mut self, opcode: u8) {
        match opcode {
            0x00 => self.op_nop(),
            0x01 => self.op_ajmp(),
            0x02 => self.op_ljmp(self.get_u16(MCS51_ADDRESSING::DATA(1)).unwrap()),
            0x03 => self.op_rr(),
            0x04 => self.op_inc(self.get_u8_mut(MCS51_ADDRESSING::ACCUMULATOR).unwrap()),
            0x05 => self.op_inc(self.get_u8_mut(MCS51_ADDRESSING::ACCUMULATOR).unwrap()),
            0x06 => self.op_inc(self.get_u8_mut(MCS51_ADDRESSING::ACCUMULATOR).unwrap()),
            0x07 => self.op_inc(self.get_u8_mut(MCS51_ADDRESSING::ACCUMULATOR).unwrap()),
            0x08 => self.op_inc(self.get_u8_mut(MCS51_ADDRESSING::ACCUMULATOR).unwrap()),
            0x09 => self.op_inc(self.get_u8_mut(MCS51_ADDRESSING::ACCUMULATOR).unwrap()),
            0x0A => self.op_inc(self.get_u8_mut(MCS51_ADDRESSING::ACCUMULATOR).unwrap()),
            0x0B => self.op_inc(self.get_u8_mut(MCS51_ADDRESSING::ACCUMULATOR).unwrap()),
            0x0C => self.op_inc(self.get_u8_mut(MCS51_ADDRESSING::ACCUMULATOR).unwrap()),
            0x0D => self.op_inc(self.get_u8_mut(MCS51_ADDRESSING::ACCUMULATOR).unwrap()),
            0x0E => self.op_inc(self.get_u8_mut(MCS51_ADDRESSING::ACCUMULATOR).unwrap()),
            0x0F => self.op_inc(self.get_u8_mut(MCS51_ADDRESSING::ACCUMULATOR).unwrap()),
            0x23 => self.op_rl(),
            _ => println!("Unknown OPCODE")
        }
    }

    pub fn op_ajmp(&mut self) {
        let hi_byte = *self.program.get(self.pc as usize).unwrap();
        let lo_byte = *self.program.get(self.pc as usize + 1).unwrap();
        let addr = (hi_byte as u16) >> 5 + (lo_byte as u16);
        self.pc = self.pc + 2;
        self.pc &= 0xF800;
        self.pc += addr;
    }

    pub fn op_inc(&mut self, data: &mut u8) {
        // No carry flag set
        if *data == 255 {
            *data = 0;
        } else {
            *data += 1;
        }
    }

    pub fn op_rr(&mut self) {
        let underflow = self.accumulator & 1;
        self.accumulator = self.accumulator >> 1 + (underflow * 0x80);
    }

    pub fn op_rl(&mut self) {
        let overflow = self.accumulator & 0x80;
        self.accumulator = self.accumulator << 1 + overflow;
    }

    pub fn op_ljmp(&mut self, addr: u16) {
        self.pc = addr;
    }

    pub fn op_sjmp(&mut self, addr_rel: i8) {
        self.pc = self.pc + 2;

        if addr_rel < 0 {
            self.pc -= addr_rel.abs() as u16;
        } else {
            self.pc += addr_rel as u16;
        }
    }

    pub fn op_nop(&mut self) {

    }

    pub fn clock() {

    }
}