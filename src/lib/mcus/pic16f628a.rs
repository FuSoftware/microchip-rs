pub enum PIC16F628A_INSTRUCTION {
    ADDWF { f: u8, d: bool },
    ANDWF { f: u8, d: bool },
    CLRF { f: u8 },
    CLRW,
    COMF { f: u8, d: bool },
    DECF { f: u8, d: bool },
    DECFSZ { f: u8, d: bool },
    INCF { f: u8, d: bool },
    INCFSZ { f: u8, d: bool },
    IORWF { f: u8, d: bool },
    MOVF { f: u8, d: bool },
    MOVWF { f: u8 },
    NOP,
    RLF { f: u8, d: bool },
    RRF { f: u8, d: bool },
    SUBWF { f: u8, d: bool },
    SWAPF { f: u8, d: bool },
    XORWF { f: u8, d: bool },

    BCF { f: u8, b: u8 },
    BSF { f: u8, b: u8 },
    BTFSC { f: u8, b: u8 },
    BTFSS { f: u8, b: u8 },

    ADDLW { k: u8 },
    ANDLW { k: u8 },
    CALL { k: u16 },
    CLRWDT,
    GOTO { k: u16 },
    IORLW { k: u8 },
    MOVLW { k: u8 },
    RETFIE,
    RETLW { k: u8 },
    RETURN,
    SLEEP,
    SUBLW { k: u8 },
    XORLW { k: u8 },
    UNKNOWN,
}

impl PIC16F628A_INSTRUCTION {
    pub fn parse(opcode: u16) -> PIC16F628A_INSTRUCTION {
        match opcode {
            0b00000000 => PIC16F628A_INSTRUCTION::NOP,
            0b00001000 => PIC16F628A_INSTRUCTION::RETURN,
            0b00001001 => PIC16F628A_INSTRUCTION::RETFIE,
            0b01100011 => PIC16F628A_INSTRUCTION::SLEEP,
            0b01100100 => PIC16F628A_INSTRUCTION::CLRWDT,
            0b100000000 => PIC16F628A_INSTRUCTION::CLRW,
            _ => {
                let id: u16 = opcode >> 12;
                match id {
                    0b00 => {
                        // Byte oriented
                        let code = opcode >> 8 & 0x0F;
                        let f: u8 = (opcode & 0x7F) as u8;
                        let d = ((opcode >> 7) & 1) > 0;

                        match code {
                            0 => PIC16F628A_INSTRUCTION::MOVWF { f },
                            1 => PIC16F628A_INSTRUCTION::CLRF { f },
                            2 => PIC16F628A_INSTRUCTION::SUBWF { f, d },
                            3 => PIC16F628A_INSTRUCTION::DECF { f, d },
                            4 => PIC16F628A_INSTRUCTION::IORWF { f, d },
                            5 => PIC16F628A_INSTRUCTION::ANDWF { f, d },
                            6 => PIC16F628A_INSTRUCTION::XORWF { f, d },
                            7 => PIC16F628A_INSTRUCTION::ADDWF { f, d },
                            8 => PIC16F628A_INSTRUCTION::MOVF { f, d },
                            9 => PIC16F628A_INSTRUCTION::COMF { f, d },
                            10 => PIC16F628A_INSTRUCTION::INCF { f, d },
                            11 => PIC16F628A_INSTRUCTION::DECFSZ { f, d },
                            12 => PIC16F628A_INSTRUCTION::RRF { f, d },
                            13 => PIC16F628A_INSTRUCTION::RLF { f, d },
                            14 => PIC16F628A_INSTRUCTION::SWAPF { f, d },
                            15 => PIC16F628A_INSTRUCTION::INCFSZ { f, d },
                            _ => PIC16F628A_INSTRUCTION::UNKNOWN,
                        }
                    }

                    0b01 => {
                        // Bit oriented
                        let code = opcode >> 10 & 0b0011;
                        let b: u8 = (opcode >> 7 & 0b11) as u8;
                        let f: u8 = (opcode & 0x7F) as u8;

                        match code {
                            0 => PIC16F628A_INSTRUCTION::BCF { f, b },
                            1 => PIC16F628A_INSTRUCTION::BSF { f, b },
                            2 => PIC16F628A_INSTRUCTION::BTFSC { f, b },
                            3 => PIC16F628A_INSTRUCTION::BTFSS { f, b },
                            _ => PIC16F628A_INSTRUCTION::UNKNOWN,
                        }
                    }

                    _ => {
                        // Literal and control
                        match opcode >> 11 & 0x7 {
                            0b100 => PIC16F628A_INSTRUCTION::CALL { k: opcode & 0x7FF },
                            0b101 => PIC16F628A_INSTRUCTION::GOTO { k: opcode & 0x7FF },
                            _ => match opcode >> 10 & 0xF {
                                0b1100 => PIC16F628A_INSTRUCTION::MOVLW {
                                    k: (opcode & 0xFF) as u8,
                                },
                                0b1101 => PIC16F628A_INSTRUCTION::RETLW {
                                    k: (opcode & 0xFF) as u8,
                                },
                                _ => match opcode >> 9 & 0x1F {
                                    0b11110 => PIC16F628A_INSTRUCTION::SUBLW {
                                        k: (opcode & 0xFF) as u8,
                                    },
                                    0b11111 => PIC16F628A_INSTRUCTION::ADDLW {
                                        k: (opcode & 0xFF) as u8,
                                    },
                                    _ => match opcode >> 8 & 0x3F {
                                        0b111000 => PIC16F628A_INSTRUCTION::IORLW {
                                            k: (opcode & 0xFF) as u8,
                                        },
                                        0b111001 => PIC16F628A_INSTRUCTION::ANDLW {
                                            k: (opcode & 0xFF) as u8,
                                        },
                                        0b111010 => PIC16F628A_INSTRUCTION::XORLW {
                                            k: (opcode & 0xFF) as u8,
                                        },
                                        _ => PIC16F628A_INSTRUCTION::UNKNOWN,
                                    },
                                },
                            },
                        }
                    }
                }
            }
        }
    }
}

#[repr(usize)]
pub enum PIC16F628A_REGISTERS {
    TMR0 = 0,
    PCL,
    STATUS,
    FSR,
    PORTA,
    PORTB,
    PCLATH,
    INTCON,
    PIR1,
    TMR1L,
    TMR1H,
    T1CON,
    TMR2,
    T2CON,
    CCPR1L,
    CCPR1H,
    CCP1CON,
    RCSTA,
    TXREG,
    RCREG,
    CMCON,
    OPTION,
    TRISA,
    TRISB,
    PIE1,
    PCON,
    PR2,
    TXSTA,
    SPBRG,
    EEDATA,
    EEADR,
    EECON1,
    EECON2,
    VRCON,
    REGISTER_COUNT,
}

pub struct PIC16F628A {
    f: u8,
    d: bool,
    b: u8,
    k: u8,
    w: u8,
    k_addr: u16,
    opcode: u16,
    status: u8,
    additional_cycles: u8,
    additional_pc: u8,
    program_memory: [u8; 0x7FF],
    stack: Vec<u16>,
    registers: [u8; PIC16F628A_REGISTERS::REGISTER_COUNT as usize],
    common_memory: [u8; 16],
    GPR1: [u8; 80],
    GPR2: [u8; 80],
    GPR3: [u8; 48],
}

impl PIC16F628A {
    pub fn new() -> PIC16F628A {
        PIC16F628A {
            f: 0,
            d: false,
            b: 0,
            k: 0,
            w: 0,
            k_addr: 0,
            opcode: 0,
            status: 0,
            additional_cycles: 0,
            additional_pc: 0,
            program_memory: [0; 0x7FF],
            stack: Vec::new(),
            registers: [0; PIC16F628A_REGISTERS::REGISTER_COUNT as usize],
            common_memory: [0; 16],
            GPR1: [0; 80],
            GPR2: [0; 80],
            GPR3: [0; 48],
        }
    }

    pub fn pc_offset(&mut self, offset: i8) {
        let pc = self.pc_read();

        if offset < 0 {
            self.pc_write(pc.wrapping_sub(offset.abs() as u16));
        } else {
            self.pc_write(pc.wrapping_add(offset as u16));
        }
        
    }

    pub fn pc_write(&mut self, value: u16) {
        self.write_register(PIC16F628A_REGISTERS::PCL, (value & 0xFF) as u8);
        self.write_register(PIC16F628A_REGISTERS::PCLATH, (value >> 8) as u8);
    }

    pub fn write_register_bit(&mut self, register: PIC16F628A_REGISTERS, bit: u8, value: bool) {
        let reg = self.get_register_mut(register).unwrap();

        if value {
            *reg |= 1 << bit;
        } else {
            *reg &= !(1 << bit);
        }
    }

    pub fn pc_read(&self) -> u16 {
        let l = self.read_register(PIC16F628A_REGISTERS::PCL) as u16;
        let h = self.read_register(PIC16F628A_REGISTERS::PCLATH) as u16;
        return l + (h << 8);
    }

    pub fn get_current_bank(&self) -> u8 {
        let reg = self.get_register(PIC16F628A_REGISTERS::STATUS).unwrap();
        return (reg >> 5) & 0b011;
    }

    pub fn set_bank(&mut self, bank: u8) {
        let status = self.get_register_mut(PIC16F628A_REGISTERS::STATUS).unwrap();
        *status &= 0b10011111;
        *status |= (bank & 0b11) << 5;
    }

    pub fn reset(&mut self) {
        // Initialize registers
        self.registers[PIC16F628A_REGISTERS::TMR0 as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::PCL as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::STATUS as usize] = 0b00011000;
        self.registers[PIC16F628A_REGISTERS::FSR as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::PORTA as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::PORTB as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::PCLATH as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::INTCON as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::PIR1 as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::TMR1L as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::TMR1H as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::T1CON as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::TMR2 as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::T2CON as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::CCPR1L as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::CCPR1H as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::CCP1CON as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::RCSTA as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::TXREG as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::RCREG as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::CMCON as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::OPTION as usize] = 0b11111111;
        self.registers[PIC16F628A_REGISTERS::TRISA as usize] = 0b11111111;
        self.registers[PIC16F628A_REGISTERS::TRISB as usize] = 0b11111111;
        self.registers[PIC16F628A_REGISTERS::PIE1 as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::PCON as usize] = 0b00001000;
        self.registers[PIC16F628A_REGISTERS::PR2 as usize] = 0b11111111;
        self.registers[PIC16F628A_REGISTERS::TXSTA as usize] = 0b00000010;
        self.registers[PIC16F628A_REGISTERS::SPBRG as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::EEDATA as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::EEADR as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::EECON1 as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::EECON2 as usize] = 0b00000000;
        self.registers[PIC16F628A_REGISTERS::VRCON as usize] = 0b00000000;
    }

    pub fn get_register_mut(&mut self, register: PIC16F628A_REGISTERS) -> Option<&mut u8> {
        self.registers.get_mut(register as usize)
    }

    pub fn get_register(&self, register: PIC16F628A_REGISTERS) -> Option<&u8> {
        self.registers.get(register as usize)
    }

    pub fn read_register(&self, register: PIC16F628A_REGISTERS) -> u8 {
        self.registers[register as usize]
    }

    pub fn write_register(&mut self, register: PIC16F628A_REGISTERS, value: u8) {
        self.registers[register as usize] = value;
    }

    pub fn get_flag(&self, register: PIC16F628A_REGISTERS, flag: u8) -> bool {
        let reg = self.get_register(register).unwrap();
        return (*reg & flag) > 0;
    }

    pub fn set_flag(&mut self, register: PIC16F628A_REGISTERS, flag: u8, value: bool) {
        let reg = self.get_register_mut(register).unwrap();
        if value {
            *reg |= flag;
        } else {
            *reg &= 0xFF - flag;
        }
    }

    pub fn set_carry_flag(&mut self, value: bool) {
        self.set_flag(PIC16F628A_REGISTERS::STATUS, 0x01, value);
    }

    pub fn get_carry_flag(&self) -> bool {
        return self.get_flag(PIC16F628A_REGISTERS::STATUS, 0x01);
    }

    pub fn set_digital_carry_flag(&mut self, value: bool) {
        self.set_flag(PIC16F628A_REGISTERS::STATUS, 0x02, value);
    }

    pub fn get_digital_carry_flag(&self) -> bool {
        return self.get_flag(PIC16F628A_REGISTERS::STATUS, 0x02);
    }

    pub fn set_zero_flag(&mut self, value: bool) {
        self.set_flag(PIC16F628A_REGISTERS::STATUS, 0x04, value);
    }

    pub fn get_zero_flag(&self) -> bool {
        return self.get_flag(PIC16F628A_REGISTERS::STATUS, 0x04);
    }

    pub fn set_memory_address(&mut self, address: u8, value: u8) {
        if let Some(addr) = self.get_memory_address_mut(address) {
            *addr = value;
        }
    }

    pub fn get_memory_address(&self, address: u8) -> Option<&u8> {
        let bank = self.get_current_bank();
        let fixed_address = address & 0x7F;

        match bank {
            0 => match fixed_address {
                0x01 => self.get_register(PIC16F628A_REGISTERS::TMR0),
                0x02 => self.get_register(PIC16F628A_REGISTERS::PCL),
                0x03 => self.get_register(PIC16F628A_REGISTERS::STATUS),
                0x04 => self.get_register(PIC16F628A_REGISTERS::FSR),
                0x05 => self.get_register(PIC16F628A_REGISTERS::PORTA),
                0x06 => self.get_register(PIC16F628A_REGISTERS::PORTB),
                0x0A => self.get_register(PIC16F628A_REGISTERS::PCLATH),
                0x0B => self.get_register(PIC16F628A_REGISTERS::INTCON),
                0x0C => self.get_register(PIC16F628A_REGISTERS::PIR1),
                0x0E => self.get_register(PIC16F628A_REGISTERS::TMR1L),
                0x0F => self.get_register(PIC16F628A_REGISTERS::TMR1H),
                0x10 => self.get_register(PIC16F628A_REGISTERS::T1CON),
                0x11 => self.get_register(PIC16F628A_REGISTERS::TMR2),
                0x12 => self.get_register(PIC16F628A_REGISTERS::T2CON),
                0x15 => self.get_register(PIC16F628A_REGISTERS::CCPR1L),
                0x16 => self.get_register(PIC16F628A_REGISTERS::CCPR1H),
                0x17 => self.get_register(PIC16F628A_REGISTERS::CCP1CON),
                0x18 => self.get_register(PIC16F628A_REGISTERS::RCSTA),
                0x19 => self.get_register(PIC16F628A_REGISTERS::TXREG),
                0x1A => self.get_register(PIC16F628A_REGISTERS::RCREG),
                0x1F => self.get_register(PIC16F628A_REGISTERS::CMCON),
                0x20..=0x6F => self.GPR1.get((fixed_address - 0x20) as usize),
                0x70..=0x7F => self.common_memory.get((fixed_address - 0x70) as usize),
                _ => None,
            },

            1 => match fixed_address {
                0x01 => self.get_register(PIC16F628A_REGISTERS::OPTION),
                0x02 => self.get_register(PIC16F628A_REGISTERS::PCL),
                0x03 => self.get_register(PIC16F628A_REGISTERS::STATUS),
                0x04 => self.get_register(PIC16F628A_REGISTERS::FSR),
                0x05 => self.get_register(PIC16F628A_REGISTERS::TRISA),
                0x06 => self.get_register(PIC16F628A_REGISTERS::TRISB),
                0x0A => self.get_register(PIC16F628A_REGISTERS::PCLATH),
                0x0B => self.get_register(PIC16F628A_REGISTERS::INTCON),
                0x0C => self.get_register(PIC16F628A_REGISTERS::PIE1),
                0x0E => self.get_register(PIC16F628A_REGISTERS::PCON),
                0x12 => self.get_register(PIC16F628A_REGISTERS::PR2),
                0x18 => self.get_register(PIC16F628A_REGISTERS::TXSTA),
                0x19 => self.get_register(PIC16F628A_REGISTERS::SPBRG),
                0x1A => self.get_register(PIC16F628A_REGISTERS::EEDATA),
                0x1B => self.get_register(PIC16F628A_REGISTERS::EEADR),
                0x1C => self.get_register(PIC16F628A_REGISTERS::EECON1),
                0x1D => self.get_register(PIC16F628A_REGISTERS::EECON2),
                0x1F => self.get_register(PIC16F628A_REGISTERS::VRCON),
                0x20..=0x6F => self.GPR2.get((fixed_address - 0x20) as usize),
                0x70..=0x7F => self.common_memory.get((fixed_address - 0x70) as usize),
                _ => None,
            },

            2 => match fixed_address {
                0x01 => self.get_register(PIC16F628A_REGISTERS::TMR0),
                0x02 => self.get_register(PIC16F628A_REGISTERS::PCL),
                0x03 => self.get_register(PIC16F628A_REGISTERS::STATUS),
                0x04 => self.get_register(PIC16F628A_REGISTERS::FSR),
                0x06 => self.get_register(PIC16F628A_REGISTERS::PORTB),
                0x0A => self.get_register(PIC16F628A_REGISTERS::PCLATH),
                0x0B => self.get_register(PIC16F628A_REGISTERS::INTCON),
                0x20..=0x4F => self.GPR3.get((fixed_address - 0x20) as usize),
                0x70..=0x7F => self.common_memory.get((fixed_address - 0x70) as usize),
                _ => None,
            },

            3 => match fixed_address {
                0x01 => self.get_register(PIC16F628A_REGISTERS::OPTION),
                0x02 => self.get_register(PIC16F628A_REGISTERS::PCL),
                0x03 => self.get_register(PIC16F628A_REGISTERS::STATUS),
                0x04 => self.get_register(PIC16F628A_REGISTERS::FSR),
                0x06 => self.get_register(PIC16F628A_REGISTERS::TRISB),
                0x0A => self.get_register(PIC16F628A_REGISTERS::PCLATH),
                0x0B => self.get_register(PIC16F628A_REGISTERS::INTCON),
                0x70..=0x7F => self.common_memory.get((fixed_address - 0x70) as usize),
                _ => None,
            },

            _ => {
                return None;
            }
        }
    }

    pub fn get_memory_address_mut(&mut self, address: u8) -> Option<&mut u8> {
        let bank = self.get_current_bank();
        let fixed_address = address & 0x7F;

        match bank {
            0 => match fixed_address {
                0x01 => self.get_register_mut(PIC16F628A_REGISTERS::TMR0),
                0x02 => self.get_register_mut(PIC16F628A_REGISTERS::PCL),
                0x03 => self.get_register_mut(PIC16F628A_REGISTERS::STATUS),
                0x04 => self.get_register_mut(PIC16F628A_REGISTERS::FSR),
                0x05 => self.get_register_mut(PIC16F628A_REGISTERS::PORTA),
                0x06 => self.get_register_mut(PIC16F628A_REGISTERS::PORTB),
                0x0A => self.get_register_mut(PIC16F628A_REGISTERS::PCLATH),
                0x0B => self.get_register_mut(PIC16F628A_REGISTERS::INTCON),
                0x0C => self.get_register_mut(PIC16F628A_REGISTERS::PIR1),
                0x0E => self.get_register_mut(PIC16F628A_REGISTERS::TMR1L),
                0x0F => self.get_register_mut(PIC16F628A_REGISTERS::TMR1H),
                0x10 => self.get_register_mut(PIC16F628A_REGISTERS::T1CON),
                0x11 => self.get_register_mut(PIC16F628A_REGISTERS::TMR2),
                0x12 => self.get_register_mut(PIC16F628A_REGISTERS::T2CON),
                0x15 => self.get_register_mut(PIC16F628A_REGISTERS::CCPR1L),
                0x16 => self.get_register_mut(PIC16F628A_REGISTERS::CCPR1H),
                0x17 => self.get_register_mut(PIC16F628A_REGISTERS::CCP1CON),
                0x18 => self.get_register_mut(PIC16F628A_REGISTERS::RCSTA),
                0x19 => self.get_register_mut(PIC16F628A_REGISTERS::TXREG),
                0x1A => self.get_register_mut(PIC16F628A_REGISTERS::RCREG),
                0x1F => self.get_register_mut(PIC16F628A_REGISTERS::CMCON),
                0x20..=0x6F => self.GPR1.get_mut((fixed_address - 0x20) as usize),
                0x70..=0x7F => self.common_memory.get_mut((fixed_address - 0x70) as usize),
                _ => None,
            },

            1 => match fixed_address {
                0x01 => self.get_register_mut(PIC16F628A_REGISTERS::OPTION),
                0x02 => self.get_register_mut(PIC16F628A_REGISTERS::PCL),
                0x03 => self.get_register_mut(PIC16F628A_REGISTERS::STATUS),
                0x04 => self.get_register_mut(PIC16F628A_REGISTERS::FSR),
                0x05 => self.get_register_mut(PIC16F628A_REGISTERS::TRISA),
                0x06 => self.get_register_mut(PIC16F628A_REGISTERS::TRISB),
                0x0A => self.get_register_mut(PIC16F628A_REGISTERS::PCLATH),
                0x0B => self.get_register_mut(PIC16F628A_REGISTERS::INTCON),
                0x0C => self.get_register_mut(PIC16F628A_REGISTERS::PIE1),
                0x0E => self.get_register_mut(PIC16F628A_REGISTERS::PCON),
                0x12 => self.get_register_mut(PIC16F628A_REGISTERS::PR2),
                0x18 => self.get_register_mut(PIC16F628A_REGISTERS::TXSTA),
                0x19 => self.get_register_mut(PIC16F628A_REGISTERS::SPBRG),
                0x1A => self.get_register_mut(PIC16F628A_REGISTERS::EEDATA),
                0x1B => self.get_register_mut(PIC16F628A_REGISTERS::EEADR),
                0x1C => self.get_register_mut(PIC16F628A_REGISTERS::EECON1),
                0x1D => self.get_register_mut(PIC16F628A_REGISTERS::EECON2),
                0x1F => self.get_register_mut(PIC16F628A_REGISTERS::VRCON),
                0x20..=0x6F => self.GPR2.get_mut((fixed_address - 0x20) as usize),
                0x70..=0x7F => self.common_memory.get_mut((fixed_address - 0x70) as usize),
                _ => None,
            },

            2 => match fixed_address {
                0x01 => self.get_register_mut(PIC16F628A_REGISTERS::TMR0),
                0x02 => self.get_register_mut(PIC16F628A_REGISTERS::PCL),
                0x03 => self.get_register_mut(PIC16F628A_REGISTERS::STATUS),
                0x04 => self.get_register_mut(PIC16F628A_REGISTERS::FSR),
                0x06 => self.get_register_mut(PIC16F628A_REGISTERS::PORTB),
                0x0A => self.get_register_mut(PIC16F628A_REGISTERS::PCLATH),
                0x0B => self.get_register_mut(PIC16F628A_REGISTERS::INTCON),
                0x20..=0x4F => self.GPR3.get_mut((fixed_address - 0x20) as usize),
                0x70..=0x7F => self.common_memory.get_mut((fixed_address - 0x70) as usize),
                _ => None,
            },

            3 => match fixed_address {
                0x01 => self.get_register_mut(PIC16F628A_REGISTERS::OPTION),
                0x02 => self.get_register_mut(PIC16F628A_REGISTERS::PCL),
                0x03 => self.get_register_mut(PIC16F628A_REGISTERS::STATUS),
                0x04 => self.get_register_mut(PIC16F628A_REGISTERS::FSR),
                0x06 => self.get_register_mut(PIC16F628A_REGISTERS::TRISB),
                0x0A => self.get_register_mut(PIC16F628A_REGISTERS::PCLATH),
                0x0B => self.get_register_mut(PIC16F628A_REGISTERS::INTCON),
                0x70..=0x7F => self.common_memory.get_mut((fixed_address - 0x70) as usize),
                _ => None,
            },

            _ => {
                return None;
            }
        }
    }

    pub fn write(&mut self, address: u8, data: u8) {
        todo!();
    }

    pub fn read(&mut self, address: u8) -> u8 {
        todo!();
    }

    pub fn read_bank_address(&mut self, address: (u8, u8)) -> u8 {
        todo!();
    }

    pub fn write_bank_address(&mut self, address: (u8, u8), data: u8) {
        todo!();
    }

    pub fn run_opcode(&mut self, opcode: u16) {
        self.opcode = opcode;
        let instruction = PIC16F628A_INSTRUCTION::parse(opcode);

        match instruction {
            PIC16F628A_INSTRUCTION::RRF { f, d } => self.op_rrf_new(f, d),
            _ => println!("Unimplemented"),
        }
    }

    pub fn run_opcode_old(&mut self, opcode: u16) {
        self.opcode = opcode;

        match opcode {
            0b00000000 => self.op_nop(),
            0b00001000 => self.op_return(),
            0b00001001 => self.op_retfie(),
            0b01100011 => self.op_sleep(),
            0b01100100 => self.op_clrwdt(),
            0b100000000 => self.op_clrw(),
            _ => {
                let id: u16 = opcode >> 12;
                match id {
                    0b00 => {
                        // Byte oriented
                        let code = opcode >> 8 & 0x0F;
                        let f: u8 = (opcode & 0x7F) as u8;
                        let d = ((opcode >> 7) & 1) > 0;

                        match code {
                            0 => self.op_movwf(),
                            1 => self.op_clrf(f),
                            2 => self.op_subwf(f, d),
                            3 => self.op_decf(f, d),
                            4 => self.op_iorwf(f, d),
                            5 => self.op_andwf(f, d),
                            6 => self.op_xorwf(),
                            7 => self.op_addwf(f, d),
                            8 => self.op_movf(f, d),
                            9 => self.op_comf(f, d),
                            10 => self.op_incf(f, d),
                            11 => self.op_decfsz(f, d),
                            12 => self.op_rrf(),
                            13 => self.op_rlf(),
                            14 => self.op_swapf(),
                            15 => self.op_incfsz(f, d),
                            _ => println!("Unused OPCODE {}", opcode),
                        }
                    }

                    0b01 => {
                        // Bit oriented
                        let code = opcode >> 10 & 0b0011;
                        let b: u8 = (opcode >> 7 & 0b11) as u8;
                        let f: u8 = (opcode & 0x7F) as u8;

                        match code {
                            0 => self.op_bcf(),
                            1 => self.op_bsf(),
                            2 => self.op_btfsc(),
                            3 => self.op_btfss(),
                            _ => println!("Unused OPCODE {}", opcode),
                        }
                    }

                    _ => {
                        // Literal and control
                        match opcode >> 11 & 0x7 {
                            0b100 => self.op_call(),
                            0b101 => self.op_goto(),
                            _ => match opcode >> 10 & 0xF {
                                0b1100 => self.op_movlw(),
                                0b1101 => self.op_retlw(),
                                _ => match opcode >> 9 & 0x1F {
                                    0b11110 => self.op_sublw(),
                                    0b11111 => self.op_addlw(),
                                    _ => match opcode >> 8 & 0x3F {
                                        0b111000 => self.op_iorlw(),
                                        0b111001 => self.op_andlw(),
                                        0b111010 => self.op_xorlw(),
                                        _ => println!("Unused OPCODE {}", opcode),
                                    },
                                },
                            },
                        }
                    }
                }
            }
        }
    }

    fn op_addwf(&mut self, f: u8, d: bool) {
        let data = self.read(f) as u16;
        let acc = self.w as u16;
        let res = acc + data;

        self.set_digital_carry_flag((acc & 0xF + data & 0xF) > 0xF);
        self.set_carry_flag(res & 0xFF00 != 0);
        self.set_zero_flag(res == 0);

        if d {
            self.w = res as u8;
        } else {
            self.write(f, res as u8);
        }
    }

    fn op_andwf(&mut self, f: u8, d: bool) {
        let res = self.w & self.read(f);

        self.set_zero_flag(res == 0);

        if d {
            self.w = res as u8;
        } else {
            self.write(f, res as u8);
        }
    }

    fn op_clrf(&mut self, f: u8) {
        self.write(f, 0);
        self.set_carry_flag(true);
    }

    fn op_clrw(&mut self) {
        self.w = 0;
        self.set_carry_flag(true);
    }

    fn op_comf(&mut self, f: u8, d: bool) {
        let data = !self.read(f);

        if d {
            self.write(f, data);
        } else {
            self.w = data;
        }
        
    }

    fn op_decf(&mut self, f: u8, d: bool) {
        let mut data = self.read(f);

        data = data.wrapping_sub(1);
        self.set_zero_flag(data == 0);

        if d {
            self.write(f, data);
        } else {
            self.w = data;
        }
    }

    fn op_decfsz(&mut self, f: u8, d: bool) {
        let mut data = self.read(f);

        data = data.wrapping_sub(1);
        
        if data == 0 {
            self.additional_pc += 1;
            self.additional_cycles += 1;
        }

        if d {
            self.write(f, data);
        } else {
            self.w = data;
        }
    }

    fn op_incf(&mut self, f: u8, d: bool) {
        let mut data = self.read(f);

        data = data.wrapping_add(1);
        self.set_zero_flag(data == 0);

        if d {
            self.write(f, data);
        } else {
            self.w = data;
        }
    }

    fn op_incfsz(&mut self, f: u8, d: bool) {
        let mut data = self.read(f);

        data = data.wrapping_add(1);
        
        if data == 0 {
            self.additional_pc += 1;
            self.additional_cycles += 1;
        }

        if d {
            self.write(f, data);
        } else {
            self.w = data;
        }
    }

    fn op_iorwf(&mut self, f: u8, d: bool) {
        let res = self.w | self.read(f);

        self.set_zero_flag(res == 0);

        if d {
            self.w = res as u8;
        } else {
            self.write(f, res as u8);
        }
    }

    fn op_movf(&mut self, f: u8, d: bool) {
        let data = self.read(f);
        self.set_zero_flag(data == 0);

        if d {
            self.write(f, data);
        } else {
            self.w = f;
        }
    }

    // Move W to f
    fn op_movwf_new(&mut self, f: u8) {
        self.write(f, self.w);
    }

    fn op_movwf(&mut self) {
        self.f = (self.opcode & 0x7f) as u8;
        self.op_movwf_new(self.f);
    }

    // No Operation
    fn op_nop(&mut self) {
        return;
    }

    // Rotate Left f through Carry
    fn op_rlf_new(&mut self, f: u8, d: bool) {
        let mut data = self.read(f) as u16;
        data = (data << 1) + self.get_carry_flag() as u16;

        self.set_carry_flag(data > 0xFF);

        if d {
            self.write(f, data as u8);
        } else {
            self.w = data as u8;
        }
    }

    fn op_rlf(&mut self) {
        self.f = (self.opcode & 0x7f) as u8;
        self.d = (self.opcode >> 7) & 1 > 0;

        self.op_rlf_new(self.f, self.d);
    }

    // Rotate Right f through Carry
    fn op_rrf_new(&mut self, f: u8, d: bool) {
        let mut data = self.read(f) as u16;
        let new_carry = (data & 1) > 0;
        data = data >> 1 + ((self.get_carry_flag() as u16) << 7);

        self.set_carry_flag(new_carry);

        if d {
            self.write(f, data as u8);
        } else {
            self.w = data as u8;
        }
    }

    fn op_rrf(&mut self) {
        self.f = (self.opcode & 0x7f) as u8;
        self.d = (self.opcode >> 7) & 1 > 0;

        self.op_rrf_new(self.f, self.d);
    }

    fn op_subwf(&mut self, f: u8, d: bool) {
        let data = self.read(f) as i8;
        let acc = self.w as i8;

        let new_data = data.wrapping_sub(acc);

        self.set_zero_flag(new_data == 0);
        self.set_carry_flag(new_data >= 0);
        // TODO Set DC Flag

        if d {
            self.write(f, new_data as u8);
        } else {
            self.w = new_data as u8;
        }
    }

    fn op_swapf(&mut self, f: u8, d: bool) {
        let data = self.read(f);
        let new_data = (data << 4) + (data >> 4);

        if d {
            self.write(f, new_data as u8);
        } else {
            self.w = new_data as u8;
        }
    }

    fn op_xorwf(&mut self, f: u8, d: bool) {
        let data = self.read(f);
        let new_data = data ^ self.w;

        if d {
            self.write(f, new_data as u8);
        } else {
            self.w = new_data as u8;
        }
    }

    // Bit operations

    fn op_bcf(&mut self, f: u8, b: u8) {
        let data = self.read(f);
        let new_data = data & !(1 << b);
        self.write(f, data);
    }

    fn op_bsf(&mut self, f: u8, b: u8) {
        let data = self.read(f);
        let new_data = data | (1 << b);
        self.write(f, data);
    }

    fn op_btfsc(&mut self, f: u8, b: u8) {
        let data = self.read(f);

        if (data & (1 << b)) == 0 {
            self.additional_pc += 1;
        }
    }

    fn op_btfss(&mut self, f: u8, b: u8) {
        let data = self.read(f);

        if (data & (1 << b)) > 0 {
            self.additional_pc += 1;
        }
    }

    // --------
    // Litteral and control
    // --------

    // Add literal to W
    fn op_addlw(&mut self, k: u8) {
        let result = self.w as u16 + k as u16;

        self.set_carry_flag(result > 0xFF);
        self.set_zero_flag((result & 0xFF) == 0);
        //TODO Set DC flag

        self.w = result as u8;
    }

    // AND literal with W
    fn op_andlw(&mut self, k: u8) {
        let result = self.w & k;
        self.set_zero_flag(result == 0);
        self.w = result;
    }

    // Call subroutine
    fn op_call(&mut self, k: u16) {
        todo!();
    }

    // Clear watchdog timer
    fn op_clrwdt(&mut self) {
        self.write_register_bit(PIC16F628A_REGISTERS::STATUS, 3, true); //PD
        self.write_register_bit(PIC16F628A_REGISTERS::STATUS, 4, true); //T0
        // TODO Set Watchdog prescaler
    }

    // Go to address
    fn op_goto(&mut self, k: u16) {
        self.pc_write(k);
    }

    // Inclusive OR literal with W
    fn op_iorlw(&mut self) {
        todo!();
    }

    // Move literal to W
    fn op_movlw(&mut self) {
        todo!();
    }

    // Return with literal in W
    fn op_retlw(&mut self) {
        todo!();
    }

    // Return from interrupt
    fn op_retfie(&mut self) {
        todo!();
    }

    // Return from subroutine
    fn op_return(&mut self) {
        todo!();
    }

    // Go into standby mode
    fn op_sleep(&mut self) {
        todo!();
    }

    // Subtract W from literal
    fn op_sublw(&mut self) {
        todo!();
    }

    // Exclusive OR literal with W
    fn op_xorlw(&mut self) {
        todo!();
    }
}
