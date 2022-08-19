enum MCS51_Compiler_Argument {
    LABEL(String),
    DATA(u8),
    DATA16(u16),
    REGISTER(u8),
    SFR(String)
}

pub struct MCS51_Compiler_Instruction {
    mnemonic: String,
    arguments: Vec<MCS51_Compiler_Argument>,
}

impl MCS51_Compiler_Instruction {
    pub fn from_string() -> MCS51_Compiler_Instruction {
        return MCS51_Compiler_Instruction {
            mnemonic: "".to_owned(),
            arguments: vec![]
        }
    }
}

pub struct MCS51_Compiler {

}