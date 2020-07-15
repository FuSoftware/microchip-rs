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

#[derive(Debug, Clone, Copy)]
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

pub struct MCS51 {
    pc: u16,
    program: Vec<u8>,
    pub registers: [u8; 8],
    pub accumulator: u8,
    ram: [u8; 255],
    stack: Vec<u8>,
}

impl MCS51 {

    pub fn new() -> MCS51 {
        MCS51 {
            pc: 0,
            registers: [0; 8],
            accumulator: 0,
            ram: [0; 255],
            program: vec![],
            stack: vec![]
        }
    }

    pub fn set_carry_flag(&mut self, value: bool) {
        todo!();
    }

    pub fn get_carry_flag(&mut self) -> bool {
        todo!();
    }

    pub fn set_program(&mut self, program: Vec<u8>) {
        self.program = program;
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.registers = [0; 8];
        self.accumulator = 0;
        self.ram = [0; 255];
    }

    pub fn clock(&mut self) {
        let opcode = self.program[self.pc as usize];
        self.opcode_dispatch(opcode);
        self.pc = self.pc + 1;
    }

    pub fn set_u8(&mut self, addressing: MCS51_ADDRESSING, value: u8) {
        match addressing {
            MCS51_ADDRESSING::ACCUMULATOR => self.accumulator = value,
            MCS51_ADDRESSING::REGISTER(reg) => self.registers[reg as usize] = value,
            MCS51_ADDRESSING::DIRECT(addr) => self.ram[addr as usize] = value,
            MCS51_ADDRESSING::INDIRECT_Ri(reg) => self.registers[*self.registers.get(reg as usize).unwrap() as usize] = value,
            _ => {
                println!("Unsupported addressing mode");
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
            MCS51_ADDRESSING::ADDR_16 => {
                let hi_byte = *self.program.get(self.pc as usize + 1).unwrap();
                let lo_byte = *self.program.get(self.pc as usize + 2).unwrap();
                let addr: u16 = (hi_byte as u16) << 8 + lo_byte as u16;
                return Some(addr);
            }
            _ => {
                println!("Unsupported addressing mode");
                return None;
            }
        }
    }

    pub fn get_u11(&self, addressing: MCS51_ADDRESSING) -> Option<u16> {
        match addressing {
            MCS51_ADDRESSING::ADDR_11 => {
                let hi_byte = *self.program.get(self.pc as usize ).unwrap() >> 3;
                let lo_byte = *self.program.get(self.pc as usize + 1 as usize).unwrap();
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
            0x01 => self.op_ajmp(MCS51_ADDRESSING::ADDR_11),
            0x02 => self.op_ljmp(MCS51_ADDRESSING::ADDR_16),
            0x03 => self.op_rr(),
            0x04 => self.op_inc(MCS51_ADDRESSING::ACCUMULATOR),
            0x05 => self.op_inc_u16(MCS51_ADDRESSING::DATA(1)),
            0x06 => self.op_inc(MCS51_ADDRESSING::INDIRECT_Ri(0)),
            0x07 => self.op_inc(MCS51_ADDRESSING::INDIRECT_Ri(1)),
            0x08 => self.op_inc(MCS51_ADDRESSING::REGISTER(0)),
            0x09 => self.op_inc(MCS51_ADDRESSING::REGISTER(1)),
            0x0A => self.op_inc(MCS51_ADDRESSING::REGISTER(2)),
            0x0B => self.op_inc(MCS51_ADDRESSING::REGISTER(3)),
            0x0C => self.op_inc(MCS51_ADDRESSING::REGISTER(4)),
            0x0D => self.op_inc(MCS51_ADDRESSING::REGISTER(5)),
            0x0E => self.op_inc(MCS51_ADDRESSING::REGISTER(6)),
            0x0F => self.op_inc(MCS51_ADDRESSING::REGISTER(7)),
            0x10 => self.op_jbc(MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2)),
            0x11 => self.op_acall(MCS51_ADDRESSING::ADDR_11),
            0x12 => self.op_lcall(MCS51_ADDRESSING::ADDR_16),
            0x13 => self.op_rrc(),
            0x14 => self.op_dec(MCS51_ADDRESSING::ACCUMULATOR),
            0x15 => self.op_dec_u16(MCS51_ADDRESSING::DATA(1)),
            0x16 => self.op_dec(MCS51_ADDRESSING::INDIRECT_Ri(0)),
            0x17 => self.op_dec(MCS51_ADDRESSING::INDIRECT_Ri(1)),
            0x18 => self.op_dec(MCS51_ADDRESSING::REGISTER(0)),
            0x19 => self.op_dec(MCS51_ADDRESSING::REGISTER(1)),
            0x1A => self.op_dec(MCS51_ADDRESSING::REGISTER(2)),
            0x1B => self.op_dec(MCS51_ADDRESSING::REGISTER(3)),
            0x1C => self.op_dec(MCS51_ADDRESSING::REGISTER(4)),
            0x1D => self.op_dec(MCS51_ADDRESSING::REGISTER(5)),
            0x1E => self.op_dec(MCS51_ADDRESSING::REGISTER(6)),
            0x1F => self.op_dec(MCS51_ADDRESSING::REGISTER(7)),
            0x23 => self.op_rl(),
            0x24 => self.op_add(MCS51_ADDRESSING::DATA(1)),
            0x25 => self.op_add(MCS51_ADDRESSING::IMMEDIATE),
            0x26 => self.op_add(MCS51_ADDRESSING::INDIRECT_Ri(0)),
            0x27 => self.op_add(MCS51_ADDRESSING::INDIRECT_Ri(1)),
            0x28 => self.op_add(MCS51_ADDRESSING::REGISTER(0)),
            0x29 => self.op_add(MCS51_ADDRESSING::REGISTER(1)),
            0x2A => self.op_add(MCS51_ADDRESSING::REGISTER(2)),
            0x2B => self.op_add(MCS51_ADDRESSING::REGISTER(3)),
            0x2C => self.op_add(MCS51_ADDRESSING::REGISTER(4)),
            0x2D => self.op_add(MCS51_ADDRESSING::REGISTER(5)),
            0x2E => self.op_add(MCS51_ADDRESSING::REGISTER(6)),
            0x2F => self.op_add(MCS51_ADDRESSING::REGISTER(7)),
            0x33 => self.op_rlc(),
            _ => println!("Unknown OPCODE")
        }
    }

    pub fn op_ajmp(&mut self, addr11: MCS51_ADDRESSING) {
        self.pc = self.pc + 2;        
        self.pc &= 0xF800;
        self.pc += self.get_u11(addr11).unwrap();
    }

    pub fn op_acall(&mut self, addr11: MCS51_ADDRESSING) {
        self.pc = self.pc + 2;
        self.stack.push((self.pc & 0xFF) as u8);
        self.stack.push(((self.pc >> 2) & 0xFF) as u8);
        self.pc &= 0xF800;
        self.pc += self.get_u11(addr11).unwrap();
    }

    pub fn op_add(&mut self) {

    }

    pub fn op_lcall(&mut self, addr16: MCS51_ADDRESSING) {
        self.pc = self.pc + 2;
        self.stack.push((self.pc & 0xFF) as u8);
        self.stack.push(((self.pc >> 2) & 0xFF) as u8);
        self.pc = self.get_u16(addr16).unwrap();
    }

    pub fn op_jbc(&mut self, bit_addr: MCS51_ADDRESSING, rel_addr: MCS51_ADDRESSING) {
        self.pc = self.pc + 3;
        let bit: bool;
        let rel: i8;

        todo!()
    }

    // Decrement
    pub fn op_dec(&mut self, operand: MCS51_ADDRESSING) {
        let op = self.get_u8(operand).unwrap();
        // No carry flag set
        if op == 0 {
            self.set_u8(operand, 255);
        } else {
            self.set_u8(operand, op - 1);
        }
    }

    pub fn op_dec_u16(&mut self, operand: MCS51_ADDRESSING) {
        let op = self.get_u8(operand).unwrap();
        // No carry flag set
        if op == 0 {
            self.set_u8(operand, 255);
            let op2 = self.get_u8(MCS51_ADDRESSING::DATA(2)).unwrap();
            if op == 0 {
                self.set_u8(MCS51_ADDRESSING::DATA(2), 255);
            } else {
                self.set_u8(MCS51_ADDRESSING::DATA(2), op2 - 1);
            }
        } else {
            self.set_u8(operand, op - 1);
        }
    }

    // Increment
    pub fn op_inc(&mut self, operand: MCS51_ADDRESSING) {
        let op = self.get_u8(operand).unwrap();
        // No carry flag set
        if op == 255 {
            self.set_u8(operand, 0);
        } else {
            self.set_u8(operand, op + 1);
        }
    }

    pub fn op_inc_u16(&mut self, operand: MCS51_ADDRESSING) {
        let op = self.get_u8(operand).unwrap();
        // No carry flag set
        if op == 255 {
            self.set_u8(operand, 0);
            let op2 = self.get_u8(MCS51_ADDRESSING::DATA(2)).unwrap();
            if op == 255 {
                self.set_u8(MCS51_ADDRESSING::DATA(2), 0);
            } else {
                self.set_u8(MCS51_ADDRESSING::DATA(2), op2 + 1);
            }
        } else {
            self.set_u8(operand, op + 1);
        }
    }

    pub fn op_rr(&mut self) {
        let underflow = self.accumulator & 1;
        self.accumulator = self.accumulator >> 1 + (underflow * 0x80);
    }

    pub fn op_rrc(&mut self) {
        let underflow = self.accumulator & 1 != 0;
        let carry = self.get_carry_flag();
        self.accumulator = self.accumulator >> 1 + (carry as u8 * 0x80);
        self.set_carry_flag(underflow);
    }

    pub fn op_rl(&mut self) {
        let overflow = self.accumulator & 0x80;
        self.accumulator = self.accumulator << 1 + overflow;
    }

    pub fn op_rlc(&mut self) {
        let overflow = self.accumulator & 0x80 != 0;
        let carry = self.get_carry_flag();
        self.accumulator = self.accumulator << 1 + carry as u8;
        self.set_carry_flag(overflow);
    }

    pub fn op_ljmp(&mut self, operand: MCS51_ADDRESSING) {
        let addr = self.get_u16(operand).unwrap();
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
}