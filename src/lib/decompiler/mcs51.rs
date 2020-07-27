use std::fmt;

pub struct MCS51_Decompiler_Instruction {
    address: u16,
    instruction: Vec<u16>,
    code: String,
    next: Vec<u16>,
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
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} -> {:?}", self.code, self.next)
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
        let hi_byte = *self.program.get(address as usize + offset as usize).unwrap() as u16;
        let lo_byte = *self.program.get(address as usize + offset as usize + 1).unwrap() as u16;
        return (hi_byte << 8) + lo_byte;
    }

    pub fn get_u8(&mut self, address: u16, offset: u16) -> u8 {
        let addr = (address + offset) as usize;
        return *self.program.get(addr).unwrap();
    }

    pub fn get_instruction(&mut self, address: u16) -> MCS51_Decompiler_Instruction {
        let opcode = *self.program.get(address as usize).unwrap();

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

            _ => println!("Undefined OPCODE {}", opcode)
        }

        return MCS51_Decompiler_Instruction::new();
    }
}