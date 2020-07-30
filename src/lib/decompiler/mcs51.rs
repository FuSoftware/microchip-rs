use std::fs;
use std::fmt;
use std::collections::BTreeMap;
use std::collections::VecDeque;

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
        write!(f, "{:04x} : {} -> {:02x?}", self.address, self.code, self.next)
        //write!(f, "{}", self.code)
    }
}

pub struct MCS51_Decompiler {
    pub program: Vec<u8>,
    pub instructions: BTreeMap<u16, MCS51_Decompiler_Instruction>,
}

impl MCS51_Decompiler {
    pub fn new() -> MCS51_Decompiler {
        MCS51_Decompiler {
            program: Vec::new(),
            instructions: BTreeMap::new()
        }
    }

    pub fn write_to_file(&self) {
        let mut code = String::new();
        for inst in &self.instructions {
            code.push_str(format!("{}", inst.1).as_str());
            code.push('\n');
        }

        fs::write("data/code.asm", code).expect("Unable to write file");
    }

    pub fn decompile(&mut self, start: u16) {
        let mut next_addresses: VecDeque<u16> = VecDeque::new();
        next_addresses.push_back(start);

        while !next_addresses.is_empty() {
            let addr = next_addresses.pop_front().unwrap();
            if !self.instructions.contains_key(&addr) {
                let v = self.get_instruction(addr); 
                println!("{}", v);            
                for new_addr in &v.next {
                    next_addresses.push_front(*new_addr);
                }
                self.instructions.insert(addr, v);
            }
        }

        println!("Decompiled {} instructions", self.instructions.len());
    }

    pub fn sfr_name(address: u8) -> String {
        return match address {
            0x80 => "P0".to_owned(),
            0x81 => "SP".to_owned(),
            0x82 => "DPL".to_owned(),
            0x83 => "DPH".to_owned(),
            0x87 => "PCON".to_owned(),
            0x88 => "TCON".to_owned(),
            0x89 => "TMOD".to_owned(),
            0x8A => "TL0".to_owned(),
            0x8B => "TL1".to_owned(),
            0x8C => "TH0".to_owned(),
            0x8D => "TH1".to_owned(),
            0x90 => "P1".to_owned(),
            0x98 => "SCON".to_owned(),
            0x99 => "SBUF".to_owned(),
            0xA0 => "P2".to_owned(),
            0xA8 => "IE".to_owned(),
            0xB0 => "P3".to_owned(),
            0xB8 => "IP".to_owned(),
            0xC8 => "T2CON".to_owned(),
            0xCA => "RCAP2L".to_owned(),
            0xCB => "RCAP2H".to_owned(),
            0xCC => "TL2".to_owned(),
            0xCD => "TH2".to_owned(),
            0xD0 => "PSW".to_owned(),
            0xE0 => "ACC".to_owned(),
            0xF0 => "B".to_owned(),
            _ => format!("{:02x}", address),
        };
    }

    pub fn bit_address_name(address: u8) -> String {
        return match address {
            0x80..=0x87 => format!("P0.{}", address - 0x80),
            0x88..=0x8F => format!("TCON.{}", address - 0x88),
            0x90..=0x97 => format!("P1.{}", address - 0x90),
            0x98..=0x9F => format!("SCON.{}", address - 0x98),
            0xA0..=0xA7 => format!("P2.{}", address - 0xA0),
            0xA8..=0xAF => format!("IE.{}", address - 0xA8),
            0xB0..=0xB7 => format!("P3.{}", address - 0xB0),
            0xB8..=0xBF => format!("IP.{}", address - 0xB8),
            0xC8..=0xCF => format!("T2CON.{}", address - 0xC8),
            0xD0..=0xD7 => format!("PSW.{}", address - 0xD0),
            0xE0..=0xE7 => format!("ACC.{}", address - 0xE0),
            0xF0..=0xF7 => format!("B.{}", address - 0xF0),
            _ => format!("{:02x}", address),
        }
    }

    pub fn get_u16(&self, address: u16, offset: u16) -> u16 {
        let hi_byte = self.program[address as usize + offset as usize] as u16;
        let lo_byte = self.program[address as usize + offset as usize + 1] as u16;
        return (hi_byte << 8) + lo_byte;
    }

    pub fn get_u8(&self, address: u16, offset: u16) -> u8 {
        let addr = (address + offset) as usize;
        return *self.program.get(addr).unwrap();
    }

    pub fn get_opcode(&self, address: u16) -> u8 {
        return self.program[address as usize];
    }

    pub fn one_byte_instruction(&self, address: u16, opcode:u8, label: &str) -> MCS51_Decompiler_Instruction {
        return MCS51_Decompiler_Instruction {
            address: address,
            instruction: vec![opcode as u16],
            code: label.to_owned(),
            next: vec![address + 1],
        };
    }

    pub fn two_byte_instruction(&self, address: u16, opcode:u8, immediate: bool, prepend: &str, append: &str) -> MCS51_Decompiler_Instruction {
        let val = self.get_u8(address, 1) as u16;

        let code = if immediate {
            format!("{}#{:02x}{}", prepend, val, append)
        } else {
            format!("{}{}{}", prepend, MCS51_Decompiler::sfr_name(val as u8), append)
        };

        return MCS51_Decompiler_Instruction {
            address: address,
            instruction: vec![opcode as u16, val],
            code: code,
            next: vec![address + 2],
        }
    }

    pub fn jump_instruction(&self, address: u16, opcode:u8, label: &str) -> MCS51_Decompiler_Instruction {
        let code_addr = self.get_u8(address, 1) as u16;

        let new_address: u16 = if code_addr & 0x80 > 0 {
            address.wrapping_sub((code_addr as i8 * -1) as u16) + 2
        } else {
            address.wrapping_add(code_addr as u16) + 2
        };

        return MCS51_Decompiler_Instruction {
            address: address,
            instruction: vec![opcode as u16, code_addr],
            code: format!("{} {:02x}", label, code_addr), //TODO Store as negative number
            next: vec![address + 2, new_address],
        };
    }

    pub fn get_rel_address(address: u16, val_i8: u16, instruction_length: u16) -> u16 {
        let new_address: u16 = if val_i8 & 0x80 > 0 {
            address.wrapping_sub((val_i8 as i8 * -1) as u16) + instruction_length
        } else {
            address.wrapping_add(val_i8) + instruction_length
        };

        return new_address
    }

    pub fn get_instruction(&mut self, address: u16) -> MCS51_Decompiler_Instruction {
        let opcode = self.program[address as usize];

        match opcode {
            0x00 => {
                return self.one_byte_instruction(address, opcode, "NOP");
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

            0x05 => {
                let dest = self.get_u8(address, 1) as u16;

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, dest],
                    code: format!("INC {:02x}", dest),
                    next: vec![address + 2],
                }
            }

            0x06..=0x07 => {
                return self.one_byte_instruction(address, opcode, &format!("INC @R{}", opcode & 0x1));
            }

            0x08..=0x0F => {
                return self.one_byte_instruction(address, opcode, &format!("INC R{}", opcode & 0x7));
            }

            0x10 => {
                let bit_addr = self.get_u8(address, 1) as u16;
                let code_addr = self.get_u8(address, 2) as u16;

                let new_address: u16 = MCS51_Decompiler::get_rel_address(address, code_addr, 3);
                let dest_name = MCS51_Decompiler::bit_address_name(bit_addr as u8);

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, bit_addr as u16, code_addr as u16],
                    code: format!("JBC {}, {:02x}", dest_name, code_addr), //TODO Store as negative number
                    next: vec![address + 3, new_address],
                };
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

            0x13 => {
                return self.one_byte_instruction(address, opcode, "RRC A");
            }

            0x14 => {
                return self.one_byte_instruction(address, opcode, "DEC A");
            }

            0x15 => {
                let data = self.get_u8(address, 1) as u16;

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, data],
                    code: format!("DEC {}", MCS51_Decompiler::sfr_name(data as u8)),
                    next: vec![address + 2],
                }
            }

            0x16..=0x17 => {
                return self.one_byte_instruction(address, opcode, &format!("DEC A, @R{}", opcode & 0x1));
            }

            0x18..=0x1F => {
                return self.one_byte_instruction(address, opcode, &format!("DEC A, R{}", opcode & 0x7));
            }

            0x20 => {
                let bit_addr = self.get_u8(address, 1) as u16;
                let code_addr = self.get_u8(address, 2) as u16;

                let new_address: u16 = if code_addr & 0x80 > 0 {
                    address.wrapping_sub((code_addr as i8 * -1) as u16) + 3
                } else {
                    address.wrapping_add(code_addr as u16) + 3
                };

                let dest_name = MCS51_Decompiler::bit_address_name(bit_addr as u8);

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, bit_addr as u16, code_addr as u16],
                    code: format!("JB {}, {:02x}", dest_name, code_addr), //TODO Store as negative number
                    next: vec![address + 3, new_address],
                };
            }

            0x22 => {
                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16],
                    code: "RET".to_owned(),
                    next: vec![],
                };
            }

            0x24 => {
                return self.two_byte_instruction(address, opcode, true, "ADD A, ", "");
            }

            0x25 => {
                return self.two_byte_instruction(address, opcode, false, "ADD A, ", "");
            }

            0x26..=0x27 => {
                return self.one_byte_instruction(address, opcode, &format!("ADD A, @R{}", opcode & 0x1));
            }

            0x28..=0x2F => {
                return self.one_byte_instruction(address, opcode, &format!("ADD A, R{}", opcode & 0x7));
            }

            0x30 => {
                let bit_addr = self.get_u8(address, 1) as u16;
                let code_addr = self.get_u8(address, 2) as u16;

                let new_address: u16 = if code_addr & 0x80 > 0 {
                    address.wrapping_sub((code_addr as i8 * -1) as u16) + 3
                } else {
                    address.wrapping_add(code_addr as u16) + 3
                };

                let dest_name = MCS51_Decompiler::bit_address_name(bit_addr as u8);

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, bit_addr as u16, code_addr as u16],
                    code: format!("JNB {}, {:02x}", dest_name, code_addr), //TODO Store as negative number
                    next: vec![address + 3, new_address],
                };
            }

            0x33 => {
                return self.one_byte_instruction(address, opcode, "RLC A");
            }

            0x34 => {
                return self.two_byte_instruction(address, opcode, true, "ADDC A, ", "");
            }

            0x35 => {
                return self.two_byte_instruction(address, opcode, false, "ADDC A, ", "");
            }

            0x40 => {
                return self.jump_instruction(address, opcode, "JC");
            }

            0x42 => {
                return self.two_byte_instruction(address, opcode, false, "ORL ", ", A");
            }

            0x44 => {
                return self.two_byte_instruction(address, opcode, true, "ORL A, ", "");
            }

            0x45 => {
                return self.two_byte_instruction(address, opcode, false, "ORL A, ", "");
            }

            0x46..=0x47 => {
                return self.one_byte_instruction(address, opcode, &format!("ORL A, @R{}", opcode & 0x1));
            }

            0x48..=0x4F => {
                return self.one_byte_instruction(address, opcode,&format!("ORL A, R{}", opcode & 0x7));
            }

            0x50 => {
                return self.jump_instruction(address, opcode, "JNC");
            }

            0x53 => {
                let dest = self.get_u8(address, 1) as u16;
                let src = self.get_u8(address, 2) as u16;

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, dest, src],
                    code: format!("ANL {:02x}, #{:02x}", dest, src), //TODO Store as negative number
                    next: vec![address + 3],
                };
            }

            0x54 => {
                return self.two_byte_instruction(address, opcode, true, "ANL A, ", "");
            }

            0x55 => {
                return self.two_byte_instruction(address, opcode, false, "ANL A, ", "");
            }

            0x60 => {
                return self.jump_instruction(address, opcode, "JZ");
            }

            0x65 => {
                return self.two_byte_instruction(address, opcode, false, "XRL A, ", "");
            }

            0x70 => {
                return self.jump_instruction(address, opcode, "JNZ");
            }

            0x74 => {
                return self.two_byte_instruction(address, opcode, true, "MOV A, ", "");
            }

            0x75 => {
                let data1 = self.get_u8(address, 1) as u16;
                let data2 = self.get_u8(address, 2) as u16;

                let address_label = MCS51_Decompiler::sfr_name(data1 as u8);

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, data1, data2],
                    code: format!("MOV {}, #{:02x}", address_label, data2),
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
                let data = self.get_u8(address, 1) as u16;
                let new_address = MCS51_Decompiler::get_rel_address(address, data, 2);

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, data as u16],
                    code: format!("SJMP {:02x}", data), //TODO Store as negative number
                    next: vec![new_address],
                };
            }

            0x85 => {
                let dest = self.get_u8(address, 1) as u16;
                let src = self.get_u8(address, 2) as u16;
                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, dest, src],
                    code: format!("MOV {}, {}", MCS51_Decompiler::sfr_name(dest as u8),  MCS51_Decompiler::sfr_name(src as u8)),
                    next: vec![address + 3],
                }
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

            0x93 => {
                return self.one_byte_instruction(address, opcode, "MOVC A, @A + DPTR");
            }

            0x94 => {
                return self.two_byte_instruction(address, opcode, true, "SUBB A, ", "");
            }

            0x95 => {
                return self.two_byte_instruction(address, opcode, false, "SUBB A, ", "");
            }

            0x98..=0x9F => {
                return self.one_byte_instruction(address, opcode, &format!("SUBB A, R{}", opcode & 0x7));
            }

            0xA3 => {
                return self.one_byte_instruction(address, opcode, "INC DPTR");
            }

            0xA4 => {
                return self.one_byte_instruction(address, opcode, "MUL AB");
            }

            0xA8..=0xAF => {
                let register = opcode & 0x7;
                let data = self.get_u8(address, 1) as u16;

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, data],
                    code: format!("MOV R{}, {:02x}", register, data),
                    next: vec![address + 2],
                };
            }

            0xB2 => {
                let data_addr = self.get_u8(address, 1) as u16;
                let bit_dest = MCS51_Decompiler::bit_address_name(data_addr as u8);

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, data_addr],
                    code: format!("CPL {}", bit_dest),
                    next: vec![address + 2],
                };
            }

            0xB4 => {
                let data = self.get_u8(address, 1) as u16;
                let destu = self.get_u8(address, 2) as u16;
                let new_address = MCS51_Decompiler::get_rel_address(address, destu, 3);

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, data, destu as u16],
                    code: format!("CJNE A, #{:02x}, {:02x}", data, destu), //TODO Store as negative number
                    next: vec![address + 3, new_address],
                };
            }

            0xB5 => {
                let data_addr = self.get_u8(address, 1) as u16;
                let destu = self.get_u8(address, 2) as u16;
                let new_address = MCS51_Decompiler::get_rel_address(address, destu, 3);

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, data_addr, destu],
                    code: format!("CJNE A, {:02x}, {:02x}", data_addr, destu), //TODO Store as negative number
                    next: vec![address + 3, new_address],
                };
            }

            0xB8..=0xBF => {
                let register = opcode & 0x7;
                let data = self.get_u8(address, 1) as u16;
                let destu = self.get_u8(address, 2) as u16;
                let new_address = MCS51_Decompiler::get_rel_address(address, destu, 3);

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, data, destu],
                    code: format!("CJNE R{}, #{:02x}, {:02x}", register, data, destu), //TODO Store as negative number
                    next: vec![address + 3, new_address],
                };
            }

            0xC0 => {
                return self.two_byte_instruction(address, opcode, false, "PUSH ", "");
            }

            0xC2 => {
                return self.two_byte_instruction(address, opcode, false, "CLR ", "");
            }

            0xC3 => {
                return self.one_byte_instruction(address, opcode, "CLR C");
            }

            0xC4 => {
                return self.one_byte_instruction(address, opcode, "SWAP");
            }

            0xC5 => {
                return self.two_byte_instruction(address, opcode, false, "XCH A, ", "");
            }

            0xD0 => {
                return self.two_byte_instruction(address, opcode, false, "POP ", "");
            }

            0xD2 => {
                let dest = self.get_u8(address, 1) as u16;
                let dest_name = MCS51_Decompiler::bit_address_name(dest as u8);
                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, dest],
                    code: format!("SETB {}", dest_name),
                    next: vec![address + 2],
                }
            }

            0xD3 => {
                return self.one_byte_instruction(address, opcode, "SETB C");
            }

            0xD4 => {
                return self.one_byte_instruction(address, opcode, "DA A");
            }

            0xD5 => {
                let data_addr = self.get_u8(address, 1) as u16;
                let destu = self.get_u8(address, 2) as u16;
                let new_address = MCS51_Decompiler::get_rel_address(address, destu, 3);

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, data_addr as u16, destu as u16],
                    code: format!("DJNZ {:02x}, {:02x}", data_addr, destu), //TODO Store as negative number
                    next: vec![address + 3, new_address],
                };
            }

            0xD8..=0xDF => {
                let register = opcode & 0x7;
                let destu = self.get_u8(address, 1) as u16;
                let new_address = MCS51_Decompiler::get_rel_address(address, destu, 2);

                return MCS51_Decompiler_Instruction {
                    address: address,
                    instruction: vec![opcode as u16, destu as u16],
                    code: format!("DJNZ R{}, {:02x}", register, destu), //TODO Store as negative number
                    next: vec![address + 2, new_address],
                };
            }

            0xE0 => {
                return self.one_byte_instruction(address, opcode, "MOVX A, @DPTR");
            }

            0xE2..=0xE3 => {
                return self.one_byte_instruction(address, opcode, &format!("MOV A, @R{}", opcode & 0x1));
            }

            0xE4 => {
                return self.one_byte_instruction(address, opcode, "CLR A");
            }

            0xE5 => {
                return self.two_byte_instruction(address, opcode, false, "MOV A, ", "");
            }

            0xE8..=0xEF => {
                return self.one_byte_instruction(address, opcode, &format!("MOV A, R{}", opcode & 0x7));
            }

            0xF0 => {
                return self.one_byte_instruction(address, opcode, "MOVX @DPTR, A");
            }

            0xF2..=0xF3 => {
                return self.one_byte_instruction(address, opcode, &format!("MOVX @R{}, A", opcode & 0x1));
            }

            0xF4 => {
                return self.one_byte_instruction(address, opcode, "CPL A");
            }

            0xF5 => {
                return self.two_byte_instruction(address, opcode, false, "MOV ", ", A");
            }

            0xF6..=0xF7 => {
                return self.one_byte_instruction(address, opcode, &format!("MOV @R{}, A", opcode & 0x1));
            }

            0xF8..=0xFF => {
                return self.one_byte_instruction(address, opcode, &format!("MOV R{}, A", opcode & 0x7));
            }

            _ => println!("Undefined OPCODE {:02x} at address {:04x}", opcode, address)
        }

        return MCS51_Decompiler_Instruction::new();
    }
}