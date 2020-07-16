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
    UNDEFINED,
}

#[derive(Debug, Clone, Copy)]
pub enum MCS51_REGISTERS {
    P0 = 0,
    SP,
    DPL,
    DPH,
    PCON,
    TCON,
    TMOD,
    TL0,
    TL1,
    TH0,
    TH1,
    P1,
    SCON,
    SBUF,
    P2,
    IE,
    P3,
    IP,
    T2CON,
    RCAP2L,
    RCAP2H,
    TL2,
    TH2,
    PSW,
    ACC,
    B,
    REGISTER_COUNT,
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
    BIT,
}

pub struct MCS51 {
    pc: u16,
    program: Vec<u8>,
    pub user_registers: [u8; 8],
    pub special_function_registers: [u8; MCS51_REGISTERS::REGISTER_COUNT as usize],
    pub ram: [u8; 255],
    pub stack: Vec<u8>,
}

impl MCS51 {
    pub fn new() -> MCS51 {
        MCS51 {
            pc: 0,
            user_registers: [0; 8],
            ram: [0; 255],
            program: vec![],
            stack: vec![],
            special_function_registers: [0; MCS51_REGISTERS::REGISTER_COUNT as usize],
        }
    }

    pub fn push_stack(&mut self, value: u8) {
        self.stack.push(value);
        self.write_sfr_rel(MCS51_REGISTERS::SP, 1, false);
    }

    pub fn pop_stack(&mut self) -> u8 {
        self.write_sfr_rel(MCS51_REGISTERS::SP, 1, true);
        return self.stack.pop().unwrap();
    }

    pub fn read_sfr(&self, register: MCS51_REGISTERS) -> Option<&u8> {
        return self.special_function_registers.get(register as usize);
    }

    pub fn write_sfr(&mut self, register: MCS51_REGISTERS, value: u8) {
        self.special_function_registers[register as usize] = value;
    }

    pub fn write_sfr_rel(&mut self, register: MCS51_REGISTERS, value: u8, sub: bool) {
        if sub {
            self.special_function_registers[register as usize] = self.special_function_registers[register as usize].wrapping_sub(value);
        } else {
            self.special_function_registers[register as usize] = self.special_function_registers[register as usize].wrapping_add(value);
        };
    }

    pub fn write_pc_rel(&mut self, value: u16, sub: bool) {
        if sub {
            self.pc = self.pc.wrapping_sub(value);
        } else {
            self.pc = self.pc.wrapping_add(value);
        }
    }

    pub fn get_current_register_bank(&self) -> u8 {
        let pr = self.read_sfr(MCS51_REGISTERS::PSW).unwrap();
        let bank = pr >> 3 & 0b11;
        return bank;
    }

    pub fn read_register(&self, register: u8) -> u8 {
        let bank = self.get_current_register_bank();
        return self.ram[register as usize + 0x08 * bank as usize];
    }

    pub fn write_register(&mut self, register: u8, value: u8) {
        let bank = self.get_current_register_bank();
        self.ram[register as usize + 0x08 * bank as usize] = value;
    }

    /*
    Bit Addressable Area: 16 bytes have been assigned for this segment, 20H-2FH. Each one of the 128 bits of this
    segment can be directly addressed (0-7FH).
    The bits can be referred to in two ways both of which are acceptable by the ASM-51. One way is to refer to their
    addresses, ie. 0 to 7FH. The other way is with reference to bytes 20H to 2FH. Thus, bits 0–7 can also be referred to
    as bits 20.0–20.7, and bits 8-FH are the same as 21.0–21.7 and so on.
    Each of the 16 bytes in this segment can also be addressed as a byte.
    */

    pub fn read_bit(&self, address: u8) -> bool {
        let register = address >> 3;
        let bit = address & 0x7;

        let value = self.read((register * 0x08) + 0x80);
        let val = *value.unwrap();
        return (val & bit) != 0;
    }

    pub fn write_bit(&mut self, address: u8, value: bool) {
        let register = (address >> 3) as u8;
        let bit = (address & 0x7) as u8;
        let addr = (register * 0x08) + 0x80;
        let mut src = *self.read(addr).unwrap();

        if value {
            src |= (value as u8) << bit;
        } else {
            src &= ((!value) as u8) << bit;
        }

        self.write(addr, src);
    }

    pub fn read(&self, address: u8) -> Option<&u8> {
        match address {
            0x00..=0x1F => self.ram.get(address as usize),
            0x20..=0x2F => self.read(0x80 + (0x08 * (address - 0x20))),
            0x30..=0x7F => self.ram.get(address as usize),
            0x80 => self
                .special_function_registers
                .get(MCS51_REGISTERS::P0 as usize),
            0x81 => self
                .special_function_registers
                .get(MCS51_REGISTERS::SP as usize),
            0x82 => self
                .special_function_registers
                .get(MCS51_REGISTERS::DPL as usize),
            0x83 => self
                .special_function_registers
                .get(MCS51_REGISTERS::DPH as usize),
            0x87 => self
                .special_function_registers
                .get(MCS51_REGISTERS::PCON as usize),
            0x88 => self
                .special_function_registers
                .get(MCS51_REGISTERS::TCON as usize),
            0x89 => self
                .special_function_registers
                .get(MCS51_REGISTERS::TMOD as usize),
            0x8A => self
                .special_function_registers
                .get(MCS51_REGISTERS::TL0 as usize),
            0x8B => self
                .special_function_registers
                .get(MCS51_REGISTERS::TL1 as usize),
            0x8C => self
                .special_function_registers
                .get(MCS51_REGISTERS::TH0 as usize),
            0x8D => self
                .special_function_registers
                .get(MCS51_REGISTERS::TH1 as usize),
            0x90 => self
                .special_function_registers
                .get(MCS51_REGISTERS::P1 as usize),
            0x98 => self
                .special_function_registers
                .get(MCS51_REGISTERS::SCON as usize),
            0x99 => self
                .special_function_registers
                .get(MCS51_REGISTERS::SBUF as usize),
            0xA0 => self
                .special_function_registers
                .get(MCS51_REGISTERS::P2 as usize),
            0xA8 => self
                .special_function_registers
                .get(MCS51_REGISTERS::IE as usize),
            0xB0 => self
                .special_function_registers
                .get(MCS51_REGISTERS::P3 as usize),
            0xB8 => self
                .special_function_registers
                .get(MCS51_REGISTERS::IP as usize),
            0xC8 => self
                .special_function_registers
                .get(MCS51_REGISTERS::T2CON as usize),
            0xCA => self
                .special_function_registers
                .get(MCS51_REGISTERS::RCAP2L as usize),
            0xCB => self
                .special_function_registers
                .get(MCS51_REGISTERS::RCAP2H as usize),
            0xCC => self
                .special_function_registers
                .get(MCS51_REGISTERS::TL2 as usize),
            0xCD => self
                .special_function_registers
                .get(MCS51_REGISTERS::TH2 as usize),
            0xD0 => self
                .special_function_registers
                .get(MCS51_REGISTERS::PSW as usize),
            0xE0 => self
                .special_function_registers
                .get(MCS51_REGISTERS::ACC as usize),
            0xF0 => self
                .special_function_registers
                .get(MCS51_REGISTERS::B as usize),
            _ => None,
        }
    }

    pub fn write(&mut self, address: u8, value: u8) {
        match address {
            0x00..=0x1F => self.ram[address as usize] = value,
            0x20..=0x2F => self.write(0x80 + (0x08 * (address - 0x20)), value),
            0x30..=0x7F => self.ram[address as usize] = value,
            0x80 => self.special_function_registers[MCS51_REGISTERS::P0 as usize] = value,
            0x81 => self.special_function_registers[MCS51_REGISTERS::SP as usize] = value,
            0x82 => self.special_function_registers[MCS51_REGISTERS::DPL as usize] = value,
            0x83 => self.special_function_registers[MCS51_REGISTERS::DPH as usize] = value,
            0x87 => self.special_function_registers[MCS51_REGISTERS::PCON as usize] = value,
            0x88 => self.special_function_registers[MCS51_REGISTERS::TCON as usize] = value,
            0x89 => self.special_function_registers[MCS51_REGISTERS::TMOD as usize] = value,
            0x8A => self.special_function_registers[MCS51_REGISTERS::TL0 as usize] = value,
            0x8B => self.special_function_registers[MCS51_REGISTERS::TL1 as usize] = value,
            0x8C => self.special_function_registers[MCS51_REGISTERS::TH0 as usize] = value,
            0x8D => self.special_function_registers[MCS51_REGISTERS::TH1 as usize] = value,
            0x90 => self.special_function_registers[MCS51_REGISTERS::P1 as usize] = value,
            0x98 => self.special_function_registers[MCS51_REGISTERS::SCON as usize] = value,
            0x99 => self.special_function_registers[MCS51_REGISTERS::SBUF as usize] = value,
            0xA0 => self.special_function_registers[MCS51_REGISTERS::P2 as usize] = value,
            0xA8 => self.special_function_registers[MCS51_REGISTERS::IE as usize] = value,
            0xB0 => self.special_function_registers[MCS51_REGISTERS::P3 as usize] = value,
            0xB8 => self.special_function_registers[MCS51_REGISTERS::IP as usize] = value,
            0xC8 => self.special_function_registers[MCS51_REGISTERS::T2CON as usize] = value,
            0xCA => self.special_function_registers[MCS51_REGISTERS::RCAP2L as usize] = value,
            0xCB => self.special_function_registers[MCS51_REGISTERS::RCAP2H as usize] = value,
            0xCC => self.special_function_registers[MCS51_REGISTERS::TL2 as usize] = value,
            0xCD => self.special_function_registers[MCS51_REGISTERS::TH2 as usize] = value,
            0xD0 => self.special_function_registers[MCS51_REGISTERS::PSW as usize] = value,
            0xE0 => self.special_function_registers[MCS51_REGISTERS::ACC as usize] = value,
            0xF0 => self.special_function_registers[MCS51_REGISTERS::B as usize] = value,
            _ => (),
        }
    }

    pub fn set_carry_flag(&mut self, value: bool) {
        let reg = *self.read_sfr(MCS51_REGISTERS::PSW).unwrap();
        if value {
            self.write_sfr(MCS51_REGISTERS::PSW, reg | 0x80);
        } else {
            self.write_sfr(MCS51_REGISTERS::PSW, reg & !0x80);
        }
    }

    pub fn get_carry_flag(&mut self) -> bool {
        return self.read_sfr(MCS51_REGISTERS::PSW).unwrap() & 0x80 != 0;
    }

    pub fn set_aux_carry_flag(&mut self, value: bool) {
        let reg = *self.read_sfr(MCS51_REGISTERS::PSW).unwrap();
        if value {
            self.write_sfr(MCS51_REGISTERS::PSW, reg | 0x40);
        } else {
            self.write_sfr(MCS51_REGISTERS::PSW, reg & !0x40);
        }
    }

    pub fn get_aux_carry_flag(&mut self) -> bool {
        return self.read_sfr(MCS51_REGISTERS::PSW).unwrap() & 0x40 != 0;
    }

    pub fn set_overflow_flag(&mut self, value: bool) {
        let reg = *self.read_sfr(MCS51_REGISTERS::PSW).unwrap();
        if value {
            self.write_sfr(MCS51_REGISTERS::PSW, reg | 0x04);
        } else {
            self.write_sfr(MCS51_REGISTERS::PSW, reg & !0x04);
        }
    }

    pub fn get_overflow_flag(&mut self) -> bool {
        return self.read_sfr(MCS51_REGISTERS::PSW).unwrap() & 0x04 != 0;
    }

    pub fn set_program(&mut self, program: Vec<u8>) {
        self.program = program;
    }

    pub fn get_accumulator(&self) -> u8 {
        return self.special_function_registers[MCS51_REGISTERS::ACC as usize];
    }

    pub fn set_accumulator(&mut self, value: u8) {
        self.special_function_registers[MCS51_REGISTERS::ACC as usize] = value;
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.user_registers = [0; 8];
        self.ram = [0; 255];
        self.reset_registers();
    }

    pub fn reset_registers(&mut self) {
        self.special_function_registers[MCS51_REGISTERS::P0 as usize] = 0xFF;
        self.special_function_registers[MCS51_REGISTERS::SP as usize] = 0x07;
        self.special_function_registers[MCS51_REGISTERS::DPL as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::DPH as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::PCON as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::TCON as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::TMOD as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::TL0 as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::TL1 as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::TH0 as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::TH1 as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::P1 as usize] = 0xFF;
        self.special_function_registers[MCS51_REGISTERS::SCON as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::SBUF as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::P2 as usize] = 0xFF;
        self.special_function_registers[MCS51_REGISTERS::IE as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::P3 as usize] = 0xFF;
        self.special_function_registers[MCS51_REGISTERS::IP as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::T2CON as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::RCAP2L as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::RCAP2H as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::TL2 as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::TH2 as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::PSW as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::ACC as usize] = 0x00;
        self.special_function_registers[MCS51_REGISTERS::B as usize] = 0x00;
    }

    pub fn clock(&mut self) {
        let opcode = self.program[self.pc as usize];
        self.opcode_dispatch(opcode);
        self.pc = self.pc + 1;
    }

    pub fn clock_skip(&mut self) {
        self.pc = self.pc + 1;
    }

    pub fn set_u8(&mut self, addressing: MCS51_ADDRESSING, value: u8) {
        match addressing {
            MCS51_ADDRESSING::ACCUMULATOR => self.write_sfr(MCS51_REGISTERS::ACC, value),
            MCS51_ADDRESSING::REGISTER(reg) => self.write_register(reg, value),
            MCS51_ADDRESSING::DIRECT(offset) => self.write(
                *self
                    .program
                    .get(self.pc as usize + offset as usize)
                    .unwrap(),
                value,
            ),
            MCS51_ADDRESSING::INDIRECT_Ri(reg) => self.write(self.read_register(reg), value),
            _ => {
                println!("Unsupported addressing mode");
            }
        }
    }

    pub fn get_u8(&self, addressing: MCS51_ADDRESSING) -> Option<u8> {
        match addressing {
            MCS51_ADDRESSING::ACCUMULATOR => Some(*self.read_sfr(MCS51_REGISTERS::ACC).unwrap()),
            MCS51_ADDRESSING::REGISTER(reg) => Some(self.read_register(reg)),
            MCS51_ADDRESSING::DIRECT(offset) => Some(
                *self
                    .read(
                        *self
                            .program
                            .get(self.pc as usize + offset as usize)
                            .unwrap(),
                    )
                    .unwrap(),
            ),
            MCS51_ADDRESSING::INDIRECT_Ri(reg) => {
                Some(*self.read(self.read_register(reg)).unwrap())
            }
            MCS51_ADDRESSING::DATA(offset) => Some(
                *self
                    .program
                    .get(self.pc as usize + offset as usize)
                    .unwrap(),
            ),
            _ => {
                println!("Unsupported addressing mode");
                return None;
            }
        }
    }

    pub fn get_i8(&self, addressing: MCS51_ADDRESSING) -> Option<i8> {
        match addressing {
            MCS51_ADDRESSING::DATA(offset) => {
                Some(i8::from_be_bytes([*self.read(offset).unwrap()]))
            }
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
                let hi_byte = *self.program.get(self.pc as usize).unwrap() >> 3;
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
            0x20 => self.op_jb(MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2)),
            0x21 => self.op_ajmp(MCS51_ADDRESSING::ADDR_11),
            0x22 => self.op_ret(),
            0x23 => self.op_rl(),
            0x24 => self.op_add(MCS51_ADDRESSING::DATA(1)),
            0x25 => self.op_add(MCS51_ADDRESSING::DIRECT(1)),
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
            0x30 => self.op_jnb(MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2)),
            0x31 => self.op_acall(MCS51_ADDRESSING::ADDR_11),
            0x32 => self.op_reti(),
            0x33 => self.op_rlc(),
            0x34 => self.op_addc(MCS51_ADDRESSING::DATA(1)),
            0x35 => self.op_addc(MCS51_ADDRESSING::DIRECT(1)),
            0x36 => self.op_addc(MCS51_ADDRESSING::INDIRECT_Ri(0)),
            0x37 => self.op_addc(MCS51_ADDRESSING::INDIRECT_Ri(1)),
            0x38 => self.op_addc(MCS51_ADDRESSING::REGISTER(0)),
            0x39 => self.op_addc(MCS51_ADDRESSING::REGISTER(1)),
            0x3A => self.op_addc(MCS51_ADDRESSING::REGISTER(2)),
            0x3B => self.op_addc(MCS51_ADDRESSING::REGISTER(3)),
            0x3C => self.op_addc(MCS51_ADDRESSING::REGISTER(4)),
            0x3D => self.op_addc(MCS51_ADDRESSING::REGISTER(5)),
            0x3E => self.op_addc(MCS51_ADDRESSING::REGISTER(6)),
            0x3F => self.op_addc(MCS51_ADDRESSING::REGISTER(7)),
            0x40 => self.op_jc(MCS51_ADDRESSING::DATA(1)),
            0x41 => self.op_ajmp(MCS51_ADDRESSING::ADDR_11),
            0x42 => self.op_orl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::ACCUMULATOR),
            0x43 => self.op_orl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::DATA(2)),
            0x44 => self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DATA(1)),
            0x45 => self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DIRECT(1)),
            0x46 => self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::INDIRECT_Ri(0)),
            0x47 => self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::INDIRECT_Ri(1)),
            0x48 => self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(0)),
            0x49 => self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(1)),
            0x4A => self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(2)),
            0x4B => self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(3)),
            0x4C => self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(4)),
            0x4D => self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(5)),
            0x4E => self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(6)),
            0x4F => self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(7)),
            0x74 => self.op_mov(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DATA(1)),
            0x75 => self.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::DATA(1)),
            0x76 => self.op_mov(MCS51_ADDRESSING::INDIRECT_Ri(0), MCS51_ADDRESSING::DATA(1)),
            0x77 => self.op_mov(MCS51_ADDRESSING::INDIRECT_Ri(1), MCS51_ADDRESSING::DATA(1)),
            0x78 => self.op_mov(MCS51_ADDRESSING::REGISTER(0), MCS51_ADDRESSING::DATA(1)),
            0x79 => self.op_mov(MCS51_ADDRESSING::REGISTER(1), MCS51_ADDRESSING::DATA(1)),
            0x7A => self.op_mov(MCS51_ADDRESSING::REGISTER(2), MCS51_ADDRESSING::DATA(1)),
            0x7B => self.op_mov(MCS51_ADDRESSING::REGISTER(3), MCS51_ADDRESSING::DATA(1)),
            0x7C => self.op_mov(MCS51_ADDRESSING::REGISTER(4), MCS51_ADDRESSING::DATA(1)),
            0x7D => self.op_mov(MCS51_ADDRESSING::REGISTER(5), MCS51_ADDRESSING::DATA(1)),
            0x7E => self.op_mov(MCS51_ADDRESSING::REGISTER(6), MCS51_ADDRESSING::DATA(1)),
            0x7F => self.op_mov(MCS51_ADDRESSING::REGISTER(7), MCS51_ADDRESSING::DATA(1)),
            _ => println!("Unknown OPCODE"),
        }
    }

    pub fn op_orl(&mut self, dest: MCS51_ADDRESSING, src: MCS51_ADDRESSING) {
        let op1 = self.get_u8(src).unwrap();
        let op2 = self.get_u8(dest).unwrap();

        let result = op1 | op2;

        self.set_u8(dest, result);
    }

    pub fn op_jc(&mut self, code_addr: MCS51_ADDRESSING) {
        let cf = self.get_carry_flag();
        self.pc = self.pc + 2;

        if cf {
            let code = self.get_u8(code_addr).unwrap();
            self.write_pc_rel(code as u16, code & 0x80 != 0)
        }
    }

    pub fn op_reti(&mut self) {
        todo!();
    }

    pub fn op_mov(&mut self, dest: MCS51_ADDRESSING, src: MCS51_ADDRESSING) {
        let src_dat = self.get_u8(src).unwrap();
        self.set_u8(dest, src_dat);
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

    pub fn op_add(&mut self, operand: MCS51_ADDRESSING) {
        let data = self.get_u8(operand).unwrap();
        let acc = self.get_accumulator();

        // Bit 3 overflow
        let tmp = (data as u16 & 0xF) + (acc as u16 & 0xF);
        self.set_aux_carry_flag(tmp > 0xF);

        // Bit 6 overflow
        let tmp = (data as u16 & 0x7F) + (acc as u16 & 0x7F);
        let ov = tmp > 0x7F;

        let result = data as u16 + acc as u16;
        let carry = result > 0xFF;

        self.set_overflow_flag(carry ^ ov);
        self.set_carry_flag(carry);

        self.set_accumulator((result & 0xFF) as u8);
    }

    pub fn op_addc(&mut self, operand: MCS51_ADDRESSING) {
        let data = self.get_u8(operand).unwrap();
        let acc = self.get_accumulator();
        let c = self.get_carry_flag() as u8;

        // Bit 3 overflow
        let tmp = (data as u16 & 0xF) + (acc as u16 & 0xF) + (c as u16);
        self.set_aux_carry_flag(tmp > 0xF);

        // Bit 6 overflow
        let tmp = (data as u16 & 0x7F) + (acc as u16 & 0x7F) + (c as u16);
        let ov = tmp > 0x7F;

        let result = data as u16 + acc as u16 + c as u16;
        let carry = result > 0xFF;

        self.set_overflow_flag(carry ^ ov);
        self.set_carry_flag(carry);

        self.set_accumulator((result & 0xFF) as u8);
    }

    pub fn op_lcall(&mut self, addr16: MCS51_ADDRESSING) {
        self.pc = self.pc + 2;
        self.stack.push((self.pc & 0xFF) as u8);
        self.stack.push(((self.pc >> 2) & 0xFF) as u8);
        self.pc = self.get_u16(addr16).unwrap();
    }

    pub fn op_jbc(&mut self, bit_addr: MCS51_ADDRESSING, code_addr: MCS51_ADDRESSING) {
        self.pc = self.pc + 3;
        let bit_address = self.get_u8(bit_addr).unwrap();

        let bit: bool = self.read_bit(bit_address);
        let rel = self.get_i8(code_addr).unwrap();

        if bit {
            self.write_bit(bit_address, false);
            if rel < 0 {
                self.write_pc_rel((rel & 0x7F) as u16, true);
            } else {
                self.write_pc_rel(rel as u16, false);
            }
        }
    }

    pub fn op_jnb(&mut self, bit_addr: MCS51_ADDRESSING, code_addr: MCS51_ADDRESSING) {
        self.pc = self.pc + 3;
        let bit_address = self.get_u8(bit_addr).unwrap();

        let bit: bool = self.read_bit(bit_address);
        let rel = self.get_i8(code_addr).unwrap();

        if !bit {
            if rel < 0 {
                self.write_pc_rel((rel & 0x7F) as u16, true);
            } else {
                self.write_pc_rel(rel as u16, false);
            }
        }
    }

    pub fn op_jb(&mut self, bit_addr: MCS51_ADDRESSING, code_addr: MCS51_ADDRESSING) {
        self.pc = self.pc + 3;
        let bit_address = self.get_u8(bit_addr).unwrap();

        let bit: bool = self.read_bit(bit_address);
        let rel = self.get_i8(code_addr).unwrap();

        if bit {
            if rel < 0 {
                self.write_pc_rel((rel & 0x7F) as u16, true);
            } else {
                self.write_pc_rel(rel as u16, false);
            }
        }
    }

    pub fn op_ret(&mut self) {
        let pc_lo = self.pop_stack();
        let pc_hi = self.pop_stack();
        self.pc = (pc_hi as u16) << 8 + pc_lo as u16;
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
        let acc = self.get_accumulator();
        let underflow = acc & 1;
        self.set_accumulator(acc >> 1 + (underflow * 0x80));
    }

    pub fn op_rrc(&mut self) {
        let acc = self.get_accumulator();
        let underflow = acc & 1 != 0;
        let carry = self.get_carry_flag();
        self.set_accumulator(acc >> 1 + (carry as u8 * 0x80));
        self.set_carry_flag(underflow);
    }

    pub fn op_rl(&mut self) {
        let acc = self.get_accumulator();
        let overflow = acc & 0x80;
        self.set_accumulator(acc << 1 + overflow);
    }

    pub fn op_rlc(&mut self) {
        let acc = self.get_accumulator();
        let overflow = acc & 0x80 != 0;
        let carry = self.get_carry_flag();
        self.set_accumulator(acc << 1 + carry as u8);
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

    pub fn op_nop(&mut self) {}
}
