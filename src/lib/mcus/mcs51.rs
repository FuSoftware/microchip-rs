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
    ADDR_16,
    ADDR_11,
    RELATIVE,
}

pub struct MCS51 {
    pc: u16,
    program: Vec<u8>,
    pub user_registers: [u8; 8],
    pub special_function_registers: [u8; MCS51_REGISTERS::REGISTER_COUNT as usize],
    pub ram: [u8; 255],
    pub stack: Vec<u8>,
    pub additional_cycles: u8,
    pub instructions: [(&'static str, u8, u16, fn(&mut MCS51)); 255],
}

impl MCS51 {
    pub fn new() -> MCS51 {
        let mut mcs51 = MCS51 {
            pc: 0,
            user_registers: [0; 8],
            ram: [0; 255],
            program: vec![],
            stack: vec![],
            special_function_registers: [0; MCS51_REGISTERS::REGISTER_COUNT as usize],
            additional_cycles: 0,
            instructions: [("NOT IMPLEMENTED", 0, 0, |_cpu| {}); 255],
        };

        mcs51.instructions[0x00] = ("NOP", 1, 0, |cpu| cpu.op_nop());
        mcs51.instructions[0x01] = ("AJMP", 1, 0, |cpu| cpu.op_ajmp(MCS51_ADDRESSING::ADDR_11));
        mcs51.instructions[0x02] = ("LJMP", 1, 0, |cpu| cpu.op_ljmp(MCS51_ADDRESSING::ADDR_16));
        mcs51.instructions[0x03] = ("RR", 0, 0, |cpu| cpu.op_rr());
        mcs51.instructions[0x04] = ("INC", 0, 0, |cpu| cpu.op_inc(MCS51_ADDRESSING::ACCUMULATOR));
        mcs51.instructions[0x05] = ("INC", 0, 1, |cpu| cpu.op_inc(MCS51_ADDRESSING::DIRECT(1)));
        mcs51.instructions[0x06] = ("INC", 0, 0, |cpu| {
            cpu.op_inc(MCS51_ADDRESSING::INDIRECT_Ri(0))
        });
        mcs51.instructions[0x07] = ("INC", 0, 0, |cpu| {
            cpu.op_inc(MCS51_ADDRESSING::INDIRECT_Ri(1))
        });
        mcs51.instructions[0x08] = ("INC", 0, 0, |cpu| cpu.op_inc(MCS51_ADDRESSING::REGISTER(0)));
        mcs51.instructions[0x09] = ("INC", 0, 0, |cpu| cpu.op_inc(MCS51_ADDRESSING::REGISTER(1)));
        mcs51.instructions[0x0A] = ("INC", 0, 0, |cpu| cpu.op_inc(MCS51_ADDRESSING::REGISTER(2)));
        mcs51.instructions[0x0B] = ("INC", 0, 0, |cpu| cpu.op_inc(MCS51_ADDRESSING::REGISTER(3)));
        mcs51.instructions[0x0C] = ("INC", 0, 0, |cpu| cpu.op_inc(MCS51_ADDRESSING::REGISTER(4)));
        mcs51.instructions[0x0D] = ("INC", 0, 0, |cpu| cpu.op_inc(MCS51_ADDRESSING::REGISTER(5)));
        mcs51.instructions[0x0E] = ("INC", 0, 0, |cpu| cpu.op_inc(MCS51_ADDRESSING::REGISTER(6)));
        mcs51.instructions[0x0F] = ("INC", 0, 0, |cpu| cpu.op_inc(MCS51_ADDRESSING::REGISTER(7)));
        mcs51.instructions[0x10] = ("JBC", 1, 0, |cpu| {
            cpu.op_jbc(MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2))
        });
        mcs51.instructions[0x11] = ("ACALL", 1, 0, |cpu| cpu.op_acall(MCS51_ADDRESSING::ADDR_11));
        mcs51.instructions[0x12] = ("LCALL", 1, 0, |cpu| cpu.op_lcall(MCS51_ADDRESSING::ADDR_16));
        mcs51.instructions[0x13] = ("RRC", 0, 0, |cpu| cpu.op_rrc());
        mcs51.instructions[0x14] = ("DEC", 0, 0, |cpu| cpu.op_dec(MCS51_ADDRESSING::ACCUMULATOR));
        mcs51.instructions[0x15] = ("DEC", 0, 1, |cpu| cpu.op_dec(MCS51_ADDRESSING::DIRECT(1)));
        mcs51.instructions[0x16] = ("DEC", 0, 0, |cpu| {
            cpu.op_dec(MCS51_ADDRESSING::INDIRECT_Ri(0))
        });
        mcs51.instructions[0x17] = ("DEC", 0, 0, |cpu| {
            cpu.op_dec(MCS51_ADDRESSING::INDIRECT_Ri(1))
        });
        mcs51.instructions[0x18] = ("DEC", 0, 0, |cpu| cpu.op_dec(MCS51_ADDRESSING::REGISTER(0)));
        mcs51.instructions[0x19] = ("DEC", 0, 0, |cpu| cpu.op_dec(MCS51_ADDRESSING::REGISTER(1)));
        mcs51.instructions[0x1A] = ("DEC", 0, 0, |cpu| cpu.op_dec(MCS51_ADDRESSING::REGISTER(2)));
        mcs51.instructions[0x1B] = ("DEC", 0, 0, |cpu| cpu.op_dec(MCS51_ADDRESSING::REGISTER(3)));
        mcs51.instructions[0x1C] = ("DEC", 0, 0, |cpu| cpu.op_dec(MCS51_ADDRESSING::REGISTER(4)));
        mcs51.instructions[0x1D] = ("DEC", 0, 0, |cpu| cpu.op_dec(MCS51_ADDRESSING::REGISTER(5)));
        mcs51.instructions[0x1E] = ("DEC", 0, 0, |cpu| cpu.op_dec(MCS51_ADDRESSING::REGISTER(6)));
        mcs51.instructions[0x1F] = ("DEC", 0, 0, |cpu| cpu.op_dec(MCS51_ADDRESSING::REGISTER(7)));
        mcs51.instructions[0x20] = ("JB", 1, 0, |cpu| {
            cpu.op_jb(MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2))
        });
        mcs51.instructions[0x21] = ("AJMP", 1, 0, |cpu| cpu.op_ajmp(MCS51_ADDRESSING::ADDR_11));
        mcs51.instructions[0x22] = ("RET", 1, 0, |cpu| cpu.op_ret());
        mcs51.instructions[0x23] = ("RL", 0, 0, |cpu| cpu.op_rl());
        mcs51.instructions[0x24] = ("ADD", 0, 1, |cpu| cpu.op_add(MCS51_ADDRESSING::DATA(1)));
        mcs51.instructions[0x25] = ("ADD", 0, 1, |cpu| cpu.op_add(MCS51_ADDRESSING::DIRECT(1)));
        mcs51.instructions[0x26] = ("ADD", 0, 0, |cpu| {
            cpu.op_add(MCS51_ADDRESSING::INDIRECT_Ri(0))
        });
        mcs51.instructions[0x27] = ("ADD", 0, 0, |cpu| {
            cpu.op_add(MCS51_ADDRESSING::INDIRECT_Ri(1))
        });
        mcs51.instructions[0x28] = ("ADD", 0, 0, |cpu| cpu.op_add(MCS51_ADDRESSING::REGISTER(0)));
        mcs51.instructions[0x29] = ("ADD", 0, 0, |cpu| cpu.op_add(MCS51_ADDRESSING::REGISTER(1)));
        mcs51.instructions[0x2A] = ("ADD", 0, 0, |cpu| cpu.op_add(MCS51_ADDRESSING::REGISTER(2)));
        mcs51.instructions[0x2B] = ("ADD", 0, 0, |cpu| cpu.op_add(MCS51_ADDRESSING::REGISTER(3)));
        mcs51.instructions[0x2C] = ("ADD", 0, 0, |cpu| cpu.op_add(MCS51_ADDRESSING::REGISTER(4)));
        mcs51.instructions[0x2D] = ("ADD", 0, 0, |cpu| cpu.op_add(MCS51_ADDRESSING::REGISTER(5)));
        mcs51.instructions[0x2E] = ("ADD", 0, 0, |cpu| cpu.op_add(MCS51_ADDRESSING::REGISTER(6)));
        mcs51.instructions[0x2F] = ("ADD", 0, 0, |cpu| cpu.op_add(MCS51_ADDRESSING::REGISTER(7)));
        mcs51.instructions[0x30] = ("JNB", 1, 0, |cpu| {
            cpu.op_jnb(MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2))
        });
        mcs51.instructions[0x31] = ("ACALL", 1, 0, |cpu| cpu.op_acall(MCS51_ADDRESSING::ADDR_11));
        mcs51.instructions[0x32] = ("RETI", 1, 0, |cpu| cpu.op_reti());
        mcs51.instructions[0x33] = ("RLC", 1, 0, |cpu| cpu.op_rlc());
        mcs51.instructions[0x34] = ("ADDC", 0, 1, |cpu| cpu.op_addc(MCS51_ADDRESSING::DATA(1)));
        mcs51.instructions[0x35] = ("ADDC", 0, 1, |cpu| cpu.op_addc(MCS51_ADDRESSING::DIRECT(1)));
        mcs51.instructions[0x36] = ("ADDC", 0, 0, |cpu| {
            cpu.op_addc(MCS51_ADDRESSING::INDIRECT_Ri(0))
        });
        mcs51.instructions[0x37] = ("ADDC", 0, 0, |cpu| {
            cpu.op_addc(MCS51_ADDRESSING::INDIRECT_Ri(1))
        });
        mcs51.instructions[0x38] = ("ADDC", 0, 0, |cpu| {
            cpu.op_addc(MCS51_ADDRESSING::REGISTER(0))
        });
        mcs51.instructions[0x39] = ("ADDC", 0, 0, |cpu| {
            cpu.op_addc(MCS51_ADDRESSING::REGISTER(1))
        });
        mcs51.instructions[0x3A] = ("ADDC", 0, 0, |cpu| {
            cpu.op_addc(MCS51_ADDRESSING::REGISTER(2))
        });
        mcs51.instructions[0x3B] = ("ADDC", 0, 0, |cpu| {
            cpu.op_addc(MCS51_ADDRESSING::REGISTER(3))
        });
        mcs51.instructions[0x3C] = ("ADDC", 0, 0, |cpu| {
            cpu.op_addc(MCS51_ADDRESSING::REGISTER(4))
        });
        mcs51.instructions[0x3D] = ("ADDC", 0, 0, |cpu| {
            cpu.op_addc(MCS51_ADDRESSING::REGISTER(5))
        });
        mcs51.instructions[0x3E] = ("ADDC", 0, 0, |cpu| {
            cpu.op_addc(MCS51_ADDRESSING::REGISTER(6))
        });
        mcs51.instructions[0x3F] = ("ADDC", 0, 0, |cpu| {
            cpu.op_addc(MCS51_ADDRESSING::REGISTER(7))
        });
        mcs51.instructions[0x40] = ("JC", 1, 0, |cpu| cpu.op_jc(MCS51_ADDRESSING::DATA(1)));
        mcs51.instructions[0x41] = ("AJMP", 1, 0, |cpu| cpu.op_ajmp(MCS51_ADDRESSING::ADDR_11));
        mcs51.instructions[0x42] = ("ORL", 0, 1, |cpu| {
            cpu.op_orl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::ACCUMULATOR)
        });
        mcs51.instructions[0x43] = ("ORL", 1, 2, |cpu| {
            cpu.op_orl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::DATA(2))
        });
        mcs51.instructions[0x44] = ("ORL", 0, 1, |cpu| {
            cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DATA(1))
        });
        mcs51.instructions[0x45] = ("ORL", 0, 1, |cpu| {
            cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DIRECT(1))
        });
        mcs51.instructions[0x46] = ("ORL", 0, 0, |cpu| {
            cpu.op_orl(
                MCS51_ADDRESSING::ACCUMULATOR,
                MCS51_ADDRESSING::INDIRECT_Ri(0),
            )
        });
        mcs51.instructions[0x47] = ("ORL", 0, 0, |cpu| {
            cpu.op_orl(
                MCS51_ADDRESSING::ACCUMULATOR,
                MCS51_ADDRESSING::INDIRECT_Ri(1),
            )
        });
        mcs51.instructions[0x48] = ("ORL", 0, 0, |cpu| {
            cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(0))
        });
        mcs51.instructions[0x49] = ("ORL", 0, 0, |cpu| {
            cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(1))
        });
        mcs51.instructions[0x4A] = ("ORL", 0, 0, |cpu| {
            cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(2))
        });
        mcs51.instructions[0x4B] = ("ORL", 0, 0, |cpu| {
            cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(3))
        });
        mcs51.instructions[0x4C] = ("ORL", 0, 0, |cpu| {
            cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(4))
        });
        mcs51.instructions[0x4D] = ("ORL", 0, 0, |cpu| {
            cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(5))
        });
        mcs51.instructions[0x4E] = ("ORL", 0, 0, |cpu| {
            cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(6))
        });
        mcs51.instructions[0x4F] = ("ORL", 0, 0, |cpu| {
            cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(7))
        });
        mcs51.instructions[0x50] = ("JNC", 1, 0, |cpu| {
            cpu.op_jnc(MCS51_ADDRESSING::DATA(1))
        });
        mcs51.instructions[0x51] = ("ACALL", 1, 0, |cpu| {
            cpu.op_acall(MCS51_ADDRESSING::ADDR_11)
        });
        mcs51.instructions[0x52] = ("ANL", 0, 1, |cpu| {
            cpu.op_anl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::ACCUMULATOR)
        });
        mcs51.instructions[0x53] = ("ANL", 1, 2, |cpu| {
            cpu.op_anl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::DATA(2))
        });
        mcs51.instructions[0x54] = ("ANL", 0, 1, |cpu| {
            cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DATA(1))
        });
        mcs51.instructions[0x55] = ("ANL", 0, 1, |cpu| {
            cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DIRECT(1))
        });
        mcs51.instructions[0x56] = ("ANL", 0, 0, |cpu| {
            cpu.op_anl(
                MCS51_ADDRESSING::ACCUMULATOR,
                MCS51_ADDRESSING::INDIRECT_Ri(0),
            )
        });
        mcs51.instructions[0x57] = ("ANL", 0, 0, |cpu| {
            cpu.op_anl(
                MCS51_ADDRESSING::ACCUMULATOR,
                MCS51_ADDRESSING::INDIRECT_Ri(1),
            )
        });
        mcs51.instructions[0x58] = ("ANL", 0, 0, |cpu| {
            cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(0))
        });
        mcs51.instructions[0x59] = ("ANL", 0, 0, |cpu| {
            cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(1))
        });
        mcs51.instructions[0x5A] = ("ANL", 0, 0, |cpu| {
            cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(2))
        });
        mcs51.instructions[0x5B] = ("ANL", 0, 0, |cpu| {
            cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(3))
        });
        mcs51.instructions[0x5C] = ("ANL", 0, 0, |cpu| {
            cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(4))
        });
        mcs51.instructions[0x5D] = ("ANL", 0, 0, |cpu| {
            cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(5))
        });
        mcs51.instructions[0x5E] = ("ANL", 0, 0, |cpu| {
            cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(6))
        });
        mcs51.instructions[0x5F] = ("ANL", 0, 0, |cpu| {
            cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(7))
        });
        mcs51.instructions[0x60] = ("JZ", 1, 0, |cpu| cpu.op_jz(MCS51_ADDRESSING::DATA(1)));
        mcs51.instructions[0x61] = ("AJMP", 1, 0, |cpu| cpu.op_ajmp(MCS51_ADDRESSING::ADDR_11));
        mcs51.instructions[0x62] = ("XRL", 0, 1, |cpu| {
            cpu.op_xrl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::ACCUMULATOR)
        });
        mcs51.instructions[0x43] = ("XRL", 1, 2, |cpu| {
            cpu.op_orl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::DATA(2))
        });
        mcs51.instructions[0x44] = ("XRL", 0, 1, |cpu| {
            cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DATA(1))
        });
        mcs51.instructions[0x45] = ("XRL", 0, 1, |cpu| {
            cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DIRECT(1))
        });
        mcs51.instructions[0x46] = ("XRL", 0, 0, |cpu| {
            cpu.op_xrl(
                MCS51_ADDRESSING::ACCUMULATOR,
                MCS51_ADDRESSING::INDIRECT_Ri(0),
            )
        });
        mcs51.instructions[0x47] = ("XRL", 0, 0, |cpu| {
            cpu.op_xrl(
                MCS51_ADDRESSING::ACCUMULATOR,
                MCS51_ADDRESSING::INDIRECT_Ri(1),
            )
        });
        mcs51.instructions[0x68] = ("XRL", 0, 0, |cpu| {
            cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(0))
        });
        mcs51.instructions[0x69] = ("XRL", 0, 0, |cpu| {
            cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(1))
        });
        mcs51.instructions[0x6A] = ("XRL", 0, 0, |cpu| {
            cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(2))
        });
        mcs51.instructions[0x6B] = ("XRL", 0, 0, |cpu| {
            cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(3))
        });
        mcs51.instructions[0x6C] = ("XRL", 0, 0, |cpu| {
            cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(4))
        });
        mcs51.instructions[0x6D] = ("XRL", 0, 0, |cpu| {
            cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(5))
        });
        mcs51.instructions[0x6E] = ("XRL", 0, 0, |cpu| {
            cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(6))
        });
        mcs51.instructions[0x6F] = ("XRL", 0, 0, |cpu| {
            cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(7))
        });
        mcs51.instructions[0x70] = ("JNZ", 1, 0, |cpu| {
            cpu.op_jnz(MCS51_ADDRESSING::DATA(1))
        });
        mcs51.instructions[0x71] = ("ACALL", 1, 0, |cpu| {
            cpu.op_acall(MCS51_ADDRESSING::ADDR_11)
        });
        mcs51.instructions[0x72] = ("ORL", 1, 1, |cpu| {
            cpu.op_orl_c(MCS51_ADDRESSING::DATA(1), false)
        });
        mcs51.instructions[0x73] = ("JMP", 1, 0, |cpu| {
            cpu.op_jmp()
        });
        mcs51.instructions[0x74] = ("MOV", 0, 1, |cpu| {
            cpu.op_mov(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DATA(1))
        });
        mcs51.instructions[0x75] = ("MOV", 0, 1, |cpu| {
            cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::DATA(1))
        });
        mcs51.instructions[0x76] = ("MOV", 0, 1, |cpu| {
            cpu.op_mov(MCS51_ADDRESSING::INDIRECT_Ri(0), MCS51_ADDRESSING::DATA(1))
        });
        mcs51.instructions[0x77] = ("MOV", 0, 1, |cpu| {
            cpu.op_mov(MCS51_ADDRESSING::INDIRECT_Ri(1), MCS51_ADDRESSING::DATA(1))
        });
        mcs51.instructions[0x78] = ("MOV", 0, 1, |cpu| {
            cpu.op_mov(MCS51_ADDRESSING::REGISTER(0), MCS51_ADDRESSING::DATA(1))
        });
        mcs51.instructions[0x79] = ("MOV", 0, 1, |cpu| {
            cpu.op_mov(MCS51_ADDRESSING::REGISTER(1), MCS51_ADDRESSING::DATA(1))
        });
        mcs51.instructions[0x7A] = ("MOV", 0, 1, |cpu| {
            cpu.op_mov(MCS51_ADDRESSING::REGISTER(2), MCS51_ADDRESSING::DATA(1))
        });
        mcs51.instructions[0x7B] = ("MOV", 0, 1, |cpu| {
            cpu.op_mov(MCS51_ADDRESSING::REGISTER(3), MCS51_ADDRESSING::DATA(1))
        });
        mcs51.instructions[0x7C] = ("MOV", 0, 1, |cpu| {
            cpu.op_mov(MCS51_ADDRESSING::REGISTER(4), MCS51_ADDRESSING::DATA(1))
        });
        mcs51.instructions[0x7D] = ("MOV", 0, 1, |cpu| {
            cpu.op_mov(MCS51_ADDRESSING::REGISTER(5), MCS51_ADDRESSING::DATA(1))
        });
        mcs51.instructions[0x7E] = ("MOV", 0, 1, |cpu| {
            cpu.op_mov(MCS51_ADDRESSING::REGISTER(6), MCS51_ADDRESSING::DATA(1))
        });
        mcs51.instructions[0x7F] = ("MOV", 0, 1, |cpu| {
            cpu.op_mov(MCS51_ADDRESSING::REGISTER(7), MCS51_ADDRESSING::DATA(1))
        });
        mcs51.instructions[0x80] = ("SJMP", 1, 0, |cpu| 
            cpu.op_sjmp(MCS51_ADDRESSING::DATA(1))
        );
        mcs51.instructions[0x81] = ("AJMP", 1, 0, |cpu| 
            cpu.op_ajmp(MCS51_ADDRESSING::ADDR_11)
        );
        mcs51.instructions[0x82] = ("ANL", 1, 1, |cpu| 
            cpu.op_anl_c(MCS51_ADDRESSING::DATA(1), false)
        );
        mcs51.instructions[0x83] = ("MOVC", 1, 0, |cpu| 
            cpu.op_movc_pc()
        );
        mcs51.instructions[0x84] = ("DIV", 3, 0, |cpu| 
            cpu.op_div()
        );
        mcs51.instructions[0x85] = ("MOV", 1, 2, |cpu| 
            cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::DIRECT(2))
        );
        mcs51.instructions[0x86] = ("MOV", 1, 1, |cpu| 
            cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::INDIRECT_Ri(0))
        );
        mcs51.instructions[0x87] = ("MOV", 1, 1, |cpu| 
            cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::INDIRECT_Ri(1))
        );
        mcs51.instructions[0x88] = ("MOV", 1, 1, |cpu| 
            cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(0))
        );
        mcs51.instructions[0x89] = ("MOV", 1, 1, |cpu| 
            cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(1))
        );
        mcs51.instructions[0x8A] = ("MOV", 1, 1, |cpu| 
            cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(2))
        );
        mcs51.instructions[0x8B] = ("MOV", 1, 1, |cpu| 
            cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(3))
        );
        mcs51.instructions[0x8C] = ("MOV", 1, 1, |cpu| 
            cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(4))
        );
        mcs51.instructions[0x8D] = ("MOV", 1, 1, |cpu| 
            cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(5))
        );
        mcs51.instructions[0x8E] = ("MOV", 1, 1, |cpu| 
            cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(6))
        );
        mcs51.instructions[0x8F] = ("MOV", 1, 1, |cpu| 
            cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(7))
        );
        mcs51.instructions[0x90] = ("MOV", 1, 2, |cpu| 
            cpu.op_mov_dptr(MCS51_ADDRESSING::DATA(1))
        );
        mcs51.instructions[0x91] = ("ACALL", 1, 0, |cpu| {
            cpu.op_acall(MCS51_ADDRESSING::ADDR_11)
        });
        mcs51.instructions[0x92] = ("MOV", 1, 1, |cpu| {
            cpu.op_mov_bit_c(MCS51_ADDRESSING::DATA(1))
        });
        mcs51.instructions[0x93] = ("MOVC", 1, 0, |cpu| {
            cpu.op_movc_dptr()
        });
        /*
        mcs51.instructions[0x94] = ("SUBB", 1, 0, |cpu| {
            cpu.op_subb(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DATA(1))
        });
        */

        mcs51
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
            self.special_function_registers[register as usize] =
                self.special_function_registers[register as usize].wrapping_sub(value);
        } else {
            self.special_function_registers[register as usize] =
                self.special_function_registers[register as usize].wrapping_add(value);
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
        let psw = self.read_sfr(MCS51_REGISTERS::PSW).unwrap();
        let bank = *psw >> 3 & 0b11;
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

    pub fn read_code_byte(&mut self, addr: usize) -> u8 {
        return self.program[addr];
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

    pub fn set_dptr(&mut self, value: u16) {
        self.write_sfr(MCS51_REGISTERS::DPH, (value >> 8) as u8);
        self.write_sfr(MCS51_REGISTERS::DPL, value as u8 & 0xFF);
    }

    pub fn get_dptr(&mut self) -> u16 {
        let dph = *self.read_sfr(MCS51_REGISTERS::DPH).unwrap();
        let dpl = *self.read_sfr(MCS51_REGISTERS::DPL).unwrap();

        return ((dph as u16) << 8) + dpl as u16;
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
        self.additional_cycles = 0;
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
        if self.additional_cycles > 0 {
            self.additional_cycles -= 1;
        } else {
            self.next_instruction();
        }
    }

    pub fn next_instruction(&mut self) {
        let opcode = self.program[self.pc as usize];
        let operation = self.opcode_dispatch(opcode);
        operation.3(self);
        self.additional_cycles = operation.1;
        self.pc = self.pc + 1 + operation.2;
    }

    pub fn set_u8(&mut self, addressing: MCS51_ADDRESSING, value: u8) {
        match addressing {
            MCS51_ADDRESSING::ACCUMULATOR => self.write_sfr(MCS51_REGISTERS::ACC, value),
            MCS51_ADDRESSING::REGISTER(reg) => self.write_register(reg, value),
            MCS51_ADDRESSING::DIRECT(offset) => self.write(*self.program.get(self.pc as usize + offset as usize).unwrap(),value),
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
            MCS51_ADDRESSING::DIRECT(offset) => Some(*self.read(*self.program.get(self.pc as usize + offset as usize).unwrap()).unwrap()),
            MCS51_ADDRESSING::INDIRECT_Ri(reg) => {
                Some(*self.read(self.read_register(reg)).unwrap())
            }
            MCS51_ADDRESSING::DATA(offset) => Some(*self.program.get(self.pc as usize + offset as usize).unwrap()),
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
                let offset: usize = self.pc as usize + 1;
                /*
                let hi_byte = *self.program.get(offset).unwrap();
                let lo_byte = *self.program.get(offset as usize + 1).unwrap();
                let addr: u16 = (hi_byte as u16) << 8 + lo_byte as u16;
                */
                
                let mut data: [u8; 2] = [0; 2];
                data.copy_from_slice(&self.program[offset..offset+2]);
                let addr = u16::from_be_bytes(data);

                return Some(addr);
            }
            MCS51_ADDRESSING::DATA(off) => {
                let offset: usize = self.pc as usize + off as usize;

                let mut data: [u8; 2] = [0; 2];
                data.copy_from_slice(&self.program[offset..offset+2]);
                let dat = u16::from_be_bytes(data);
                return Some(dat);
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

    pub fn get_next_opcode(&mut self) -> &(&'static str, u8, u16, fn(&mut MCS51)) {
        return self.instructions.get(self.pc as usize).unwrap();
    }

    pub fn opcode_dispatch(&self, opcode: u8) -> (&'static str, u8, u16, fn(&mut MCS51)) {
        return self.instructions[opcode as usize];
    }

    pub fn op_mov_c_bit(&mut self, bit_addr: MCS51_ADDRESSING) {
        let bit = self.read_bit(self.get_u8(bit_addr).unwrap());
        self.set_carry_flag(bit);
    }

    pub fn op_mov_bit_c(&mut self, bit_addr: MCS51_ADDRESSING) {
        let cf = self.get_carry_flag();
        self.write_bit(self.get_u8(bit_addr).unwrap(), cf);
    }

    pub fn op_mov_dptr(&mut self, data16: MCS51_ADDRESSING) {
        let data = self.get_u16(data16).unwrap();
        self.set_dptr(data);
    }

    pub fn op_div(&mut self) {
        let b = *self.read_sfr(MCS51_REGISTERS::B).unwrap();
        self.set_carry_flag(false);

        if b == 0 {
            self.set_overflow_flag(true);
        } else {
            self.set_overflow_flag(false);
            let a = self.get_accumulator();
            let result = a / b;
            let remainder = a % b;

            self.set_accumulator(result);
            self.write_sfr(MCS51_REGISTERS::B, remainder);
        }        
    }

    pub fn op_movc_pc(&mut self) {
        let pc = self.pc + 1;
        let acc = self.get_accumulator() as u16;
        let value = self.read_code_byte((pc + acc) as usize);
        self.set_accumulator(value);

    }

    pub fn op_movc_dptr(&mut self) {
        let acc = self.get_accumulator() as u16;
        let dptr = self.get_dptr();
        let value = self.read_code_byte((dptr + acc) as usize);
        self.set_accumulator(value);
    }

    pub fn op_jmp(&mut self) {
        let acc = self.get_accumulator() as u16;
        let dptr = self.get_dptr();
        let new_pc = dptr.wrapping_add(acc);

        self.pc = new_pc;
    }

    pub fn op_jnz(&mut self, code_addr: MCS51_ADDRESSING) {
        let acc = self.get_accumulator();
        self.pc = self.pc + 2;

        if acc != 0 {
            let code = self.get_u8(code_addr).unwrap();
            self.write_pc_rel(code as u16, code & 0x80 != 0)
        }
    }

    pub fn op_jz(&mut self, code_addr: MCS51_ADDRESSING) {
        let acc = self.get_accumulator();
        self.pc = self.pc + 2;

        if acc == 0 {
            let code = self.get_u8(code_addr).unwrap();
            self.write_pc_rel(code as u16, code & 0x80 != 0)
        }
    }

    pub fn op_jnc(&mut self, code_addr: MCS51_ADDRESSING) {
        let cf = self.get_carry_flag();
        self.pc = self.pc + 2;

        if !cf {
            let code = self.get_u8(code_addr).unwrap();
            self.write_pc_rel(code as u16, code & 0x80 != 0)
        }
    }

    pub fn op_anl_c(&mut self, addr: MCS51_ADDRESSING, complement: bool) {
        let bit = self.read_bit(self.get_u8(addr).unwrap());
        let mut cf = self.get_carry_flag();

        if complement {
            cf = cf & !bit;
        } else {
            cf = cf & bit;
        }

        self.set_carry_flag(cf);
    }

    pub fn op_anl(&mut self, dest: MCS51_ADDRESSING, src: MCS51_ADDRESSING) {
        let op1 = self.get_u8(src).unwrap();
        let op2 = self.get_u8(dest).unwrap();

        let result = op1 & op2;

        self.set_u8(dest, result);
    }

    pub fn op_xrl(&mut self, dest: MCS51_ADDRESSING, src: MCS51_ADDRESSING) {
        let op1 = self.get_u8(src).unwrap();
        let op2 = self.get_u8(dest).unwrap();

        let result = op1 ^ op2;

        self.set_u8(dest, result);
    }

    pub fn op_orl_c(&mut self, addr: MCS51_ADDRESSING, complement: bool) {
        let bit = self.read_bit(self.get_u8(addr).unwrap());
        let mut cf = self.get_carry_flag();

        if complement {
            cf = cf | !bit;
        } else {
            cf = cf | bit;
        }

        self.set_carry_flag(cf);
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
        self.stack.push(((self.pc >> 8) & 0xFF) as u8);
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
        self.push_stack((self.pc & 0xFF) as u8);
        self.push_stack(((self.pc >> 2) & 0xFF) as u8);
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
        let pc_hi = self.pop_stack();
        let pc_lo = self.pop_stack();
        self.pc = (pc_hi as u16) << 8 + pc_lo as u16;
    }

    // Decrement
    pub fn op_dec(&mut self, operand: MCS51_ADDRESSING) {
        let op = self.get_u8(operand).unwrap();
        self.set_u8(operand, op.wrapping_sub(1)); 
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
        self.set_u8(operand, op.wrapping_add(1));        
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

    pub fn op_sjmp(&mut self, addr: MCS51_ADDRESSING) {
        let addr_rel = self.get_i8(addr).unwrap();
        self.pc = self.pc + 2;

        if addr_rel < 0 {
            self.pc -= addr_rel.abs() as u16;
        } else {
            self.pc += addr_rel as u16;
        }
    }

    pub fn op_nop(&mut self) {}
}
