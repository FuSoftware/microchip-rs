use std::fmt;
use std::collections::BTreeMap;

pub struct MCS51_Decompiler_Instruction {
    address: u16,
    instruction: Vec<u16>,
    code: String,
    pub next: Vec<u16>,
}

impl MCS51_Decompiler_Instruction {
    pub fn new() -> MCS51_Decompiler_Instruction {
        MCS51_Decompiler_Instruction {
            address: 0x00,
            instruction: Vec::new(),
            code: "".to_owned(),
            next: Vec::new(),
        }
    }    
}

impl fmt::Display for MCS51_Decompiler_Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {:02x?}", self.code, self.next)
    }
}

pub struct MCS51_Decompiler {
    pub program: Vec<u8>
}

impl MCS51_Decompiler {
    pub fn new() -> MCS51_Decompiler {
        MCS51_Decompiler {
            program: Vec::new()
        }
    }

    pub fn get_u16(&mut self, address: u16, offset: u16) -> u16 {
        let hi_byte = self.program[address as usize + offset as usize] as u16;
        let lo_byte = self.program[address as usize + offset as usize + 1] as u16;
        return (hi_byte << 8) + lo_byte;
    }

    pub fn get_u8(&mut self, address: u16, offset: u16) -> u8 {
        let addr = (address + offset) as usize;
        return *self.program.get(addr).unwrap();
    }

    pub fn get_opcode(&mut self, address: u16) -> u8 {
        return self.program[address as usize];
    }
    pub fn get_instruction(&mut self, address: u16) -> MCS51_Decompiler_Instruction {
        let opcode = self.program[address as usize];

        match opcode {
            0x00 => {

            }

            0x01 => {

            }

            0x02 => {
                let dest = self.get_u16(address, 1);

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, dest],
                    code: format!("LJMP {:04x}", dest),
                    next: vec![dest],
                }
            }

            0x06..=0x07 => {
                let register = opcode - 0x06;
                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16],
                    code: format!("INC @R{}", register),
                    next: vec![address + 1],
                }
            }

            0x08..=0x0F => {
                let register = opcode & 0x7;
                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16],
                    code: format!("INC R{}", register),
                    next: vec![address + 1],
                }
            }

            0x12 => {
                let dest = self.get_u16(address, 1);

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, dest],
                    code: format!("LCALL {:04x}", dest),
                    next: vec![address + 3, dest],
                }
            }

            0x20 => {
                let bit_addr = self.get_u8(address, 1) as u16;
                let code_addr = self.get_u8(address, 2) as u16;

                let new_address: u16 = if code_addr & 0x80 > 0 {
                    address.wrapping_sub((code_addr as i8 * -1) as u16) + 3
                } else {
                    address.wrapping_add(code_addr as u16) + 2
                };

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, bit_addr as u16, code_addr as u16],
                    code: format!("JB #{:02x}, {:02x}", bit_addr, code_addr), //TODO Store as negative number
                    next: vec![address + 3, new_address],
                };
            }

            0x30 => {
                let bit_addr = self.get_u8(address, 1) as u16;
                let code_addr = self.get_u8(address, 2) as u16;

                let new_address: u16 = if code_addr & 0x80 > 0 {
                    address.wrapping_sub((code_addr as i8 * -1) as u16) + 3
                } else {
                    address.wrapping_add(code_addr as u16) + 2
                };

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, bit_addr as u16, code_addr as u16],
                    code: format!("JNB #{:02x}, {:02x}", bit_addr, code_addr), //TODO Store as negative number
                    next: vec![address + 3, new_address],
                };
            }

            0x74 => {
                let data = self.get_u8(address, 1) as u16;

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, data],
                    code: format!("MOV A, #{:02x}", data),
                    next: vec![address + 3],
                }
            }

            0x75 => {
                let data1 = self.get_u8(address, 1) as u16;
                let data2 = self.get_u8(address, 2) as u16;

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, data1, data2],
                    code: format!("MOV {:02x}, #{:02x}", data1, data2),
                    next: vec![address + 3],
                }
            }

            0x78..=0x7F => {
                let register = opcode & 0x7;
                let data = self.get_u8(address, 1) as u16;

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, data],
                    code: format!("MOV R{}, #{:02x}", register, data),
                    next: vec![address + 2],
                }
            }

            0x80 => {
                let data = self.get_u8(address, 1);
                let new_address: u16 = if data & 0x80 > 0 {
                    address.wrapping_sub((data as i8 * -1) as u16)
                } else {
                    address.wrapping_add(data as u16)
                } + 2;

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, data as u16],
                    code: format!("SJMP {:02x}", data), //TODO Store as negative number
                    next: vec![new_address],
                };
            }

            0x90 => {
                let data = self.get_u16(address, 1) as u16;

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, data],
                    code: format!("MOV DPTR, #{:04x}", data),
                    next: vec![address + 3],
                }
            }

            0xB4 => {
                let data = self.get_u8(address, 1);
                let destu = self.get_u8(address, 2) as u16;
                let offset = (destu & 0x7F) as u16;
                let new_address: u16 = if destu & 0x80 > 0 {
                    address.wrapping_sub((destu as i8 * -1) as u16) + 3
                } else {
                    address.wrapping_add(offset) + 2
                };

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, destu as u16],
                    code: format!("CJNE A, #{:02x}, {:02x}", data, destu), //TODO Store as negative number
                    next: vec![address + 3, new_address],
                };
            }

            0xB8..=0xBF => {
                let register = opcode & 0x7;
                let data = self.get_u8(address, 1);
                let destu = self.get_u8(address, 2) as u16;
                let offset = (destu & 0x7F) as u16;
                let new_address: u16 = if destu & 0x80 > 0 {
                    address.wrapping_sub((destu as i8 * -1) as u16) + 3
                } else {
                    address.wrapping_add(offset) + 2
                };

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, destu as u16],
                    code: format!("CJNE R{}, #{:02x}, {:02x}", register, data, destu), //TODO Store as negative number
                    next: vec![address + 3, new_address],
                };
            }

            0xC2 => {
                let dest = self.get_u8(address, 1) as u16;
                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, dest],
                    code: format!("CLR {:02x}", dest),
                    next: vec![address + 2],
                }
            }

            0xD8..=0xDF => {
                let register = opcode & 0x7;;
                let destu = self.get_u8(address, 1) as u16;
                let offset = (destu & 0x7F) as u16;
                let new_address: u16 = if destu & 0x80 > 0 {
                    address.wrapping_sub((destu as i8 * -1) as u16) + 2
                } else {
                    address.wrapping_add(offset) + 2
                };

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, destu as u16],
                    code: format!("DJNZ R{}, {:02x}", register, destu), //TODO Store as negative number
                    next: vec![address + 2, new_address],
                };
            }

            0xE0 => {
                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16],
                    code: format!("MOVX A, @DPTR"),
                    next: vec![address + 1],
                }
            }

            0xE4 => {
                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16],
                    code: format!("CLR A"),
                    next: vec![address + 1],
                }
            }

            0xE8..=0xEF => {
                let register = opcode & 0x7;
                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16],
                    code: format!("MOV A, R{}", register),
                    next: vec![address + 1],
                }
            }

            0xF0 => {
                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16],
                    code: format!("MOVX @DPTR, A"),
                    next: vec![address + 1],
                }
            }

            0xF2..=0xF3 => {
                let register = opcode & 0x1;
                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16],
                    code: format!("MOVX @R{}, A", register),
                    next: vec![address + 1],
                }
            }

            0xF6..=0xF7 => {
                let register = opcode & 0x1;
                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16],
                    code: format!("MOV @R{}, A", register),
                    next: vec![address + 1],
                }
            }

            0xF8..=0xFF => {
                let register = opcode & 0x7;
                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16],
                    code: format!("MOV R{}, A", register),
                    next: vec![address + 1],
                }
            }

            _ => println!("Undefined OPCODE {:02x}", opcode)
        }

        return MCS51_Decompiler_Instruction::new();
    }
}