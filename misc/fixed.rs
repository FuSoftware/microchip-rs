0x00 => {
    self.op_nop();
    self.opcode_additional_work("NOP", 0, 1);
},
0x01 => {
    self.op_ajmp();
    self.opcode_additional_work("AJMP", 1, 0);
},
0x02 => {
    self.op_ljmp(MCS51_ADDRESSING::ADDR_16);
    self.opcode_additional_work("LJMP", 1, 0);
},
0x03 => {
    self.op_rr();
    self.opcode_additional_work("RR", 0, 1);
},
0x04 => {
    self.op_inc(MCS51_ADDRESSING::ACCUMULATOR);
    self.opcode_additional_work("INC", 0, 1);
},
0x05 => {
    self.op_inc(MCS51_ADDRESSING::DIRECT(1));
    self.opcode_additional_work("INC", 0, 2);
},
0x06 => {
    self.op_inc(MCS51_ADDRESSING::INDIRECT_Ri(0));
    self.opcode_additional_work("INC", 0, 1);
},
0x07 => {
    self.op_inc(MCS51_ADDRESSING::INDIRECT_Ri(1));
    self.opcode_additional_work("INC", 0, 1);
},
0x08 => {
    self.op_inc(MCS51_ADDRESSING::REGISTER(0));
    self.opcode_additional_work("INC", 0, 1);
},
0x09 => {
    self.op_inc(MCS51_ADDRESSING::REGISTER(1));
    self.opcode_additional_work("INC", 0, 1);
},
0x0A => {
    self.op_inc(MCS51_ADDRESSING::REGISTER(2));
    self.opcode_additional_work("INC", 0, 1);
},
0x0B => {
    self.op_inc(MCS51_ADDRESSING::REGISTER(3));
    self.opcode_additional_work("INC", 0, 1);
},
0x0C => {
    self.op_inc(MCS51_ADDRESSING::REGISTER(4));
    self.opcode_additional_work("INC", 0, 1);
},
0x0D => {
    self.op_inc(MCS51_ADDRESSING::REGISTER(5));
    self.opcode_additional_work("INC", 0, 1);
},
0x0E => {
    self.op_inc(MCS51_ADDRESSING::REGISTER(6));
    self.opcode_additional_work("INC", 0, 1);
},
0x0F => {
    self.op_inc(MCS51_ADDRESSING::REGISTER(7));
    self.opcode_additional_work("INC", 0, 1);
},
0x10 => {
    self.op_jbc(MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    self.opcode_additional_work("JBC", 1, 0);
},
0x11 => {
    self.op_acall();
    self.opcode_additional_work("ACALL", 1, 0);
},
0x12 => {
    self.op_lcall(MCS51_ADDRESSING::ADDR_16);
    self.opcode_additional_work("LCALL", 1, 0);
},
0x13 => {
    self.op_rrc();
    self.opcode_additional_work("RRC", 0, 1);
},
0x14 => {
    self.op_dec(MCS51_ADDRESSING::ACCUMULATOR);
    self.opcode_additional_work("DEC", 0, 1);
},
0x15 => {
    self.op_dec(MCS51_ADDRESSING::DIRECT(1));
    self.opcode_additional_work("DEC", 0, 2);
},
0x16 => {
    self.op_dec(MCS51_ADDRESSING::INDIRECT_Ri(0));
    self.opcode_additional_work("DEC", 0, 1);
},
0x17 => {
    self.op_dec(MCS51_ADDRESSING::INDIRECT_Ri(1));
    self.opcode_additional_work("DEC", 0, 1);
},
0x18 => {
    self.op_dec(MCS51_ADDRESSING::REGISTER(0));
    self.opcode_additional_work("DEC", 0, 1);
},
0x19 => {
    self.op_dec(MCS51_ADDRESSING::REGISTER(1));
    self.opcode_additional_work("DEC", 0, 1);
},
0x1A => {
    self.op_dec(MCS51_ADDRESSING::REGISTER(2));
    self.opcode_additional_work("DEC", 0, 1);
},
0x1B => {
    self.op_dec(MCS51_ADDRESSING::REGISTER(3));
    self.opcode_additional_work("DEC", 0, 1);
},
0x1C => {
    self.op_dec(MCS51_ADDRESSING::REGISTER(4));
    self.opcode_additional_work("DEC", 0, 1);
},
0x1D => {
    self.op_dec(MCS51_ADDRESSING::REGISTER(5));
    self.opcode_additional_work("DEC", 0, 1);
},
0x1E => {
    self.op_dec(MCS51_ADDRESSING::REGISTER(6));
    self.opcode_additional_work("DEC", 0, 1);
},
0x1F => {
    self.op_dec(MCS51_ADDRESSING::REGISTER(7));
    self.opcode_additional_work("DEC", 0, 1);
},
0x20 => {
    self.op_jb(MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    self.opcode_additional_work("JB", 1, 0);
},
0x21 => {
    self.op_ajmp();
    self.opcode_additional_work("AJMP", 1, 0);
},
0x22 => {
    self.op_ret();
    self.opcode_additional_work("RET", 1, 0);
},
0x23 => {
    self.op_rl();
    self.opcode_additional_work("RL", 0, 1);
},
0x24 => {
    self.op_add(MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("ADD", 0, 2);
},
0x25 => {
    self.op_add(MCS51_ADDRESSING::DIRECT(1));
    self.opcode_additional_work("ADD", 0, 2);
},
0x26 => {
    self.op_add(MCS51_ADDRESSING::INDIRECT_Ri(0));
    self.opcode_additional_work("ADD", 0, 1);
},
0x27 => {
    self.op_add(MCS51_ADDRESSING::INDIRECT_Ri(1));
    self.opcode_additional_work("ADD", 0, 1);
},
0x28 => {
    self.op_add(MCS51_ADDRESSING::REGISTER(0));
    self.opcode_additional_work("ADD", 0, 1);
},
0x29 => {
    self.op_add(MCS51_ADDRESSING::REGISTER(1));
    self.opcode_additional_work("ADD", 0, 1);
},
0x2A => {
    self.op_add(MCS51_ADDRESSING::REGISTER(2));
    self.opcode_additional_work("ADD", 0, 1);
},
0x2B => {
    self.op_add(MCS51_ADDRESSING::REGISTER(3));
    self.opcode_additional_work("ADD", 0, 1);
},
0x2C => {
    self.op_add(MCS51_ADDRESSING::REGISTER(4));
    self.opcode_additional_work("ADD", 0, 1);
},
0x2D => {
    self.op_add(MCS51_ADDRESSING::REGISTER(5));
    self.opcode_additional_work("ADD", 0, 1);
},
0x2E => {
    self.op_add(MCS51_ADDRESSING::REGISTER(6));
    self.opcode_additional_work("ADD", 0, 1);
},
0x2F => {
    self.op_add(MCS51_ADDRESSING::REGISTER(7));
    self.opcode_additional_work("ADD", 0, 1);
},
0x30 => {
    self.op_jnb(MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    self.opcode_additional_work("JNB", 1, 0);
},
0x31 => {
    self.op_acall();
    self.opcode_additional_work("ACALL", 1, 0);
},
0x32 => {
    self.op_reti();
    self.opcode_additional_work("RETI", 1, 0);
},
0x33 => {
    self.op_rlc();
    self.opcode_additional_work("RLC", 1, 1);
},
0x34 => {
    self.op_addc(MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("ADDC", 0, 2);
},
0x35 => {
    self.op_addc(MCS51_ADDRESSING::DIRECT(1));
    self.opcode_additional_work("ADDC", 0, 2);
},
0x36 => {
    self.op_addc(MCS51_ADDRESSING::INDIRECT_Ri(0));
    self.opcode_additional_work("ADDC", 0, 1);
},
0x37 => {
    self.op_addc(MCS51_ADDRESSING::INDIRECT_Ri(1));
    self.opcode_additional_work("ADDC", 0, 1);
},
0x38 => {
    self.op_addc(MCS51_ADDRESSING::REGISTER(0));
    self.opcode_additional_work("ADDC", 0, 1);
},
0x39 => {
    self.op_addc(MCS51_ADDRESSING::REGISTER(1));
    self.opcode_additional_work("ADDC", 0, 1);
},
0x3A => {
    self.op_addc(MCS51_ADDRESSING::REGISTER(2));
    self.opcode_additional_work("ADDC", 0, 1);
},
0x3B => {
    self.op_addc(MCS51_ADDRESSING::REGISTER(3));
    self.opcode_additional_work("ADDC", 0, 1);
},
0x3C => {
    self.op_addc(MCS51_ADDRESSING::REGISTER(4));
    self.opcode_additional_work("ADDC", 0, 1);
},
0x3D => {
    self.op_addc(MCS51_ADDRESSING::REGISTER(5));
    self.opcode_additional_work("ADDC", 0, 1);
},
0x3E => {
    self.op_addc(MCS51_ADDRESSING::REGISTER(6));
    self.opcode_additional_work("ADDC", 0, 1);
},
0x3F => {
    self.op_addc(MCS51_ADDRESSING::REGISTER(7));
    self.opcode_additional_work("ADDC", 0, 1);
},
0x40 => {
    self.op_jc(MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("JC", 1, 0);
},
0x41 => {
    self.op_ajmp();
    self.opcode_additional_work("AJMP", 1, 0);
},
0x42 => {
    self.op_orl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::ACCUMULATOR);
    self.opcode_additional_work("ORL", 0, 2);
},
0x43 => {
    self.op_orl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::DATA(2));
    self.opcode_additional_work("ORL", 1, 3);
},
0x44 => {
    self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("ORL", 0, 2);
},
0x45 => {
    self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DIRECT(1));
    self.opcode_additional_work("ORL", 0, 2);
},
0x46 => {
    self.op_orl(
        MCS51_ADDRESSING::ACCUMULATOR,
        MCS51_ADDRESSING::INDIRECT_Ri(0),
    );
    self.opcode_additional_work("ORL", 0, 1);
},
0x47 => {
    self.op_orl(
        MCS51_ADDRESSING::ACCUMULATOR,
        MCS51_ADDRESSING::INDIRECT_Ri(1),
    );
    self.opcode_additional_work("ORL", 0, 1);
},
0x48 => {
    self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(0));
    self.opcode_additional_work("ORL", 0, 1);
},
0x49 => {
    self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(1));
    self.opcode_additional_work("ORL", 0, 1);
},
0x4A => {
    self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(2));
    self.opcode_additional_work("ORL", 0, 1);
},
0x4B => {
    self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(3));
    self.opcode_additional_work("ORL", 0, 1);
},
0x4C => {
    self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(4));
    self.opcode_additional_work("ORL", 0, 1);
},
0x4D => {
    self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(5));
    self.opcode_additional_work("ORL", 0, 1);
},
0x4E => {
    self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(6));
    self.opcode_additional_work("ORL", 0, 1);
},
0x4F => {
    self.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(7));
    self.opcode_additional_work("ORL", 0, 1);
},
0x50 => {
    self.op_jnc(MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("JNC", 1, 0);
},
0x51 => {
    self.op_acall();
    self.opcode_additional_work("ACALL", 1, 0);
},
0x52 => {
    self.op_anl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::ACCUMULATOR);
    self.opcode_additional_work("ANL", 0, 2);
},
0x53 => {
    self.op_anl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::DATA(2));
    self.opcode_additional_work("ANL", 1, 3);
},
0x54 => {
    self.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("ANL", 0, 2);
},
0x55 => {
    self.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DIRECT(1));
    self.opcode_additional_work("ANL", 0, 2);
},
0x56 => {
    self.op_anl(
        MCS51_ADDRESSING::ACCUMULATOR,
        MCS51_ADDRESSING::INDIRECT_Ri(0),
    );
    self.opcode_additional_work("ANL", 0, 1);
},
0x57 => {
    self.op_anl(
        MCS51_ADDRESSING::ACCUMULATOR,
        MCS51_ADDRESSING::INDIRECT_Ri(1),
    );
    self.opcode_additional_work("ANL", 0, 1);
},
0x58 => {
    self.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(0));
    self.opcode_additional_work("ANL", 0, 1);
},
0x59 => {
    self.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(1));
    self.opcode_additional_work("ANL", 0, 1);
},
0x5A => {
    self.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(2));
    self.opcode_additional_work("ANL", 0, 1);
},
0x5B => {
    self.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(3));
    self.opcode_additional_work("ANL", 0, 1);
},
0x5C => {
    self.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(4));
    self.opcode_additional_work("ANL", 0, 1);
},
0x5D => {
    self.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(5));
    self.opcode_additional_work("ANL", 0, 1);
},
0x5E => {
    self.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(6));
    self.opcode_additional_work("ANL", 0, 1);
},
0x5F => {
    self.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(7));
    self.opcode_additional_work("ANL", 0, 1);
},
0x60 => {
    self.op_jz(MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("JZ", 1, 0);
},
0x61 => {
    self.op_ajmp();
    self.opcode_additional_work("AJMP", 1, 0);
},
0x62 => {
    self.op_xrl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::ACCUMULATOR);
    self.opcode_additional_work("XRL", 0, 2);
},
0x63 => {
    self.op_orl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::DATA(2));
    self.opcode_additional_work("XRL", 1, 3);
},
0x64 => {
    self.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("XRL", 0, 2);
},
0x65 => {
    self.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DIRECT(1));
    self.opcode_additional_work("XRL", 0, 2);
},
0x66 => {
    self.op_xrl(
        MCS51_ADDRESSING::ACCUMULATOR,
        MCS51_ADDRESSING::INDIRECT_Ri(0),
    );
    self.opcode_additional_work("XRL", 0, 1);
},
0x67 => {
    self.op_xrl(
        MCS51_ADDRESSING::ACCUMULATOR,
        MCS51_ADDRESSING::INDIRECT_Ri(1),
    );
    self.opcode_additional_work("XRL", 0, 1);
},
0x68 => {
    self.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(0));
    self.opcode_additional_work("XRL", 0, 1);
},
0x69 => {
    self.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(1));
    self.opcode_additional_work("XRL", 0, 1);
},
0x6A => {
    self.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(2));
    self.opcode_additional_work("XRL", 0, 1);
},
0x6B => {
    self.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(3));
    self.opcode_additional_work("XRL", 0, 1);
},
0x6C => {
    self.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(4));
    self.opcode_additional_work("XRL", 0, 1);
},
0x6D => {
    self.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(5));
    self.opcode_additional_work("XRL", 0, 1);
},
0x6E => {
    self.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(6));
    self.opcode_additional_work("XRL", 0, 1);
},
0x6F => {
    self.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(7));
    self.opcode_additional_work("XRL", 0, 1);
},
0x70 => {
    self.op_jnz(MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("JNZ", 1, 0);
},
0x71 => {
    self.op_acall();
    self.opcode_additional_work("ACALL", 1, 0);
},
0x72 => {
    self.op_orl_c(MCS51_ADDRESSING::DATA(1), false);
    self.opcode_additional_work("ORL", 1, 2);
},
0x73 => {
    self.op_jmp();
    self.opcode_additional_work("JMP", 1, 1);
},
0x74 => {
    self.op_mov(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("MOV", 0, 2);
},
0x75 => {
    self.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::DATA(2));
    self.opcode_additional_work("MOV", 0, 3);
},
0x76 => {
    self.op_mov(MCS51_ADDRESSING::INDIRECT_Ri(0), MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("MOV", 0, 2);
},
0x77 => {
    self.op_mov(MCS51_ADDRESSING::INDIRECT_Ri(1), MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("MOV", 0, 2);
},
0x78 => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(0), MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("MOV", 0, 2);
},
0x79 => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(1), MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("MOV", 0, 2);
},
0x7A => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(2), MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("MOV", 0, 2);
},
0x7B => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(3), MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("MOV", 0, 2);
},
0x7C => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(4), MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("MOV", 0, 2);
},
0x7D => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(5), MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("MOV", 0, 2);
},
0x7E => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(6), MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("MOV", 0, 2);
},
0x7F => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(7), MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("MOV", 0, 2);
},
0x80 => {
    self.op_sjmp(MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("SJMP", 1, 1);
},
0x81 => {
    self.op_ajmp();
    self.opcode_additional_work("AJMP", 1, 1);
},
0x82 => {
    self.op_anl_c(MCS51_ADDRESSING::DATA(1), false);
    self.opcode_additional_work("ANL", 1, 2);
},
0x83 => {
    self.op_movc_pc();
    self.opcode_additional_work("MOVC", 1, 1);
},
0x84 => {
    self.op_div();
    self.opcode_additional_work("DIV", 3, 1);
},
0x85 => {
    self.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::DIRECT(2));
    self.opcode_additional_work("MOV", 1, 3);
},
0x86 => {
    self.op_mov(
        MCS51_ADDRESSING::DIRECT(1),
        MCS51_ADDRESSING::INDIRECT_Ri(0),
    );
    self.opcode_additional_work("MOV", 1, 2);
},
0x87 => {
    self.op_mov(
        MCS51_ADDRESSING::DIRECT(1),
        MCS51_ADDRESSING::INDIRECT_Ri(1),
    );
    self.opcode_additional_work("MOV", 1, 2);
},
0x88 => {
    self.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(0));
    self.opcode_additional_work("MOV", 1, 2);
},
0x89 => {
    self.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(1));
    self.opcode_additional_work("MOV", 1, 2);
},
0x8A => {
    self.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(2));
    self.opcode_additional_work("MOV", 1, 2);
},
0x8B => {
    self.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(3));
    self.opcode_additional_work("MOV", 1, 2);
},
0x8C => {
    self.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(4));
    self.opcode_additional_work("MOV", 1, 2);
},
0x8D => {
    self.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(5));
    self.opcode_additional_work("MOV", 1, 2);
},
0x8E => {
    self.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(6));
    self.opcode_additional_work("MOV", 1, 2);
},
0x8F => {
    self.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(7));
    self.opcode_additional_work("MOV", 1, 2);
},
0x90 => {
    self.op_mov_dptr(MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("MOV", 1, 3);
},
0x91 => {
    self.op_acall();
    self.opcode_additional_work("ACALL", 1, 1);
},
0x92 => {
    self.op_mov_bit_c(MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("MOV", 1, 2);
},
0x93 => {
    self.op_movc_dptr();
    self.opcode_additional_work("MOVC", 1, 1);
},
0x94 => {
    self.op_subb(MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("SUBB", 0, 2);
},
0x95 => {
    self.op_subb(MCS51_ADDRESSING::DIRECT(1));
    self.opcode_additional_work("SUBB", 0, 2);
},
0x96 => {
    self.op_subb(MCS51_ADDRESSING::INDIRECT_Ri(0));
    self.opcode_additional_work("SUBB", 0, 1);
},
0x97 => {
    self.op_subb(MCS51_ADDRESSING::INDIRECT_Ri(1));
    self.opcode_additional_work("SUBB", 0, 1);
},
0x98 => {
    self.op_subb(MCS51_ADDRESSING::REGISTER(0));
    self.opcode_additional_work("SUBB", 0, 1);
},
0x99 => {
    self.op_subb(MCS51_ADDRESSING::REGISTER(1));
    self.opcode_additional_work("SUBB", 0, 1);
},
0x9A => {
    self.op_subb(MCS51_ADDRESSING::REGISTER(2));
    self.opcode_additional_work("SUBB", 0, 1);
},
0x9B => {
    self.op_subb(MCS51_ADDRESSING::REGISTER(3));
    self.opcode_additional_work("SUBB", 0, 1);
},
0x9C => {
    self.op_subb(MCS51_ADDRESSING::REGISTER(4));
    self.opcode_additional_work("SUBB", 0, 1);
},
0x9D => {
    self.op_subb(MCS51_ADDRESSING::REGISTER(5));
    self.opcode_additional_work("SUBB", 0, 1);
},
0x9E => {
    self.op_subb(MCS51_ADDRESSING::REGISTER(6));
    self.opcode_additional_work("SUBB", 0, 1);
},
0x9F => {
    self.op_subb(MCS51_ADDRESSING::REGISTER(7));
    self.opcode_additional_work("SUBB", 0, 1);
},
0xA0 => {
    self.op_orl_c(MCS51_ADDRESSING::DATA(1), true);
    self.opcode_additional_work("ORLC", 1, 2);
},
0xA1 => {
    self.op_ajmp();
    self.opcode_additional_work("AJMP", 1, 1);
},
0xA2 => {
    self.op_mov_c_bit(MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("MOV", 0, 2);
},
0xA3 => {
    self.op_inc_dptr();
    self.opcode_additional_work("INC DPTR", 1, 1);
},
0xA4 => {
    self.op_mul();
    self.opcode_additional_work("MUL", 3, 1)
},
0xA5 => { self.opcode_additional_work("RESERVED", 0, 0); },
0xA6 => {
    self.op_mov(
        MCS51_ADDRESSING::INDIRECT_Ri(0),
        MCS51_ADDRESSING::DIRECT(1),
    );
    self.opcode_additional_work("MOV", 1, 2)
},
0xA7 => {
    self.op_mov(
        MCS51_ADDRESSING::INDIRECT_Ri(1),
        MCS51_ADDRESSING::DIRECT(1),
    );
    self.opcode_additional_work("MOV", 1, 2)
},
0xA8 => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(0), MCS51_ADDRESSING::DIRECT(1));
    self.opcode_additional_work("MOV", 1, 2)
},
0xA9 => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(1), MCS51_ADDRESSING::DIRECT(1));
    self.opcode_additional_work("MOV", 1, 2)
},
0xAA => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(2), MCS51_ADDRESSING::DIRECT(1));
    self.opcode_additional_work("MOV", 1, 2)
},
0xAB => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(3), MCS51_ADDRESSING::DIRECT(1));
    self.opcode_additional_work("MOV", 1, 2)
},
0xAC => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(4), MCS51_ADDRESSING::DIRECT(1));
    self.opcode_additional_work("MOV", 1, 2)
},
0xAD => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(5), MCS51_ADDRESSING::DIRECT(1));
    self.opcode_additional_work("MOV", 1, 2)
},
0xAE => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(6), MCS51_ADDRESSING::DIRECT(1));
    self.opcode_additional_work("MOV", 1, 2)
},
0xAF => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(7), MCS51_ADDRESSING::DIRECT(1));
    self.opcode_additional_work("MOV", 1, 2)
},
0xB0 => {
    self.op_anl_c(MCS51_ADDRESSING::DIRECT(1), true);
    self.opcode_additional_work("ANL", 1, 2)
},
0xB1 => {},
0xB2 => {},
0xB3 => {},
0xB4 => {
    self.op_cjne(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    self.opcode_additional_work("CJNE", 2, 0)
},
0xB5 => {
    self.op_cjne(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::DATA(2));
    self.opcode_additional_work("CJNE", 2, 0)
},
0xB6 => {
    self.op_cjne(MCS51_ADDRESSING::INDIRECT_Ri(0), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    self.opcode_additional_work("CJNE", 2, 0)
},
0xB7 => {
    self.op_cjne(MCS51_ADDRESSING::INDIRECT_Ri(1), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    self.opcode_additional_work("CJNE", 2, 0)
},
0xB8 => {
    self.op_cjne(MCS51_ADDRESSING::REGISTER(0), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    self.opcode_additional_work("CJNE", 2, 0)
},
0xB9 => {
    self.op_cjne(MCS51_ADDRESSING::REGISTER(1), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    self.opcode_additional_work("CJNE", 2, 0)
},
0xBA => {
    self.op_cjne(MCS51_ADDRESSING::REGISTER(2), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    self.opcode_additional_work("CJNE", 2, 0)
},
0xBB => {
    self.op_cjne(MCS51_ADDRESSING::REGISTER(3), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    self.opcode_additional_work("CJNE", 2, 0)
},
0xBC => {
    self.op_cjne(MCS51_ADDRESSING::REGISTER(4), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    self.opcode_additional_work("CJNE", 2, 0)
},
0xBD => {
    self.op_cjne(MCS51_ADDRESSING::REGISTER(5), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    self.opcode_additional_work("CJNE", 2, 0)
},
0xBE => {
    self.op_cjne(MCS51_ADDRESSING::REGISTER(6), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    self.opcode_additional_work("CJNE", 2, 0)
},
0xBF => {
    self.op_cjne(MCS51_ADDRESSING::REGISTER(7), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    self.opcode_additional_work("CJNE", 2, 0)
},
0xC0 => {},
0xC1 => {},
0xC2 => {
    self.op_clr(MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("CLR", 2, 2)
},
0xC3 => {
    self.set_carry_flag(false);
    self.opcode_additional_work("CLR C", 1, 1)
},
0xC4 => {},
0xC5 => {},
0xC6 => {},
0xC7 => {},
0xC8 => {},
0xC9 => {},
0xCA => {},
0xCB => {},
0xCC => {},
0xCD => {},
0xCE => {},
0xCF => {},
0xD0 => {},
0xD1 => {},
0xD2 => {},
0xD3 => {},
0xD4 => {},
0xD5 => {},
0xD6 => {},
0xD7 => {},
0xD8 => {
    self.op_djnz(MCS51_ADDRESSING::REGISTER(0), MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("DJNZ", 0, 0)
},
0xD9 => {
    self.op_djnz(MCS51_ADDRESSING::REGISTER(1), MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("DJNZ", 0, 0)
},
0xDA => {
    self.op_djnz(MCS51_ADDRESSING::REGISTER(2), MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("DJNZ", 0, 0)
},
0xDB => {
    self.op_djnz(MCS51_ADDRESSING::REGISTER(3), MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("DJNZ", 0, 0)
},
0xDC => {
    self.op_djnz(MCS51_ADDRESSING::REGISTER(4), MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("DJNZ", 0, 0)
},
0xDD => {
    self.op_djnz(MCS51_ADDRESSING::REGISTER(5), MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("DJNZ", 0, 0)
},
0xDE => {
    self.op_djnz(MCS51_ADDRESSING::REGISTER(6), MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("DJNZ", 0, 0)
},
0xDF => {
    self.op_djnz(MCS51_ADDRESSING::REGISTER(7), MCS51_ADDRESSING::DATA(1));
    self.opcode_additional_work("DJNZ", 0, 0)
},
0xE0 => {},
0xE1 => {},
0xE2 => {},
0xE3 => {},
0xE4 => {
    self.op_clr(MCS51_ADDRESSING::ACCUMULATOR);
    self.opcode_additional_work("CLR A", 1, 1)
},
0xE5 => {},
0xE6 => {},
0xE7 => {},
0xE8 => {},
0xE9 => {},
0xEA => {},
0xEB => {},
0xEC => {},
0xED => {},
0xEE => {},
0xEF => {},
0xF0 => {},
0xF1 => {},
0xF2 => {},
0xF3 => {},
0xF4 => {},
0xF5 => {
    self.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::ACCUMULATOR);
    self.opcode_additional_work("MOV", 0, 2);
},
0xF6 => {
    self.op_mov(MCS51_ADDRESSING::INDIRECT_Ri(0), MCS51_ADDRESSING::ACCUMULATOR);
    self.opcode_additional_work("MOV", 0, 1);
},
0xF7 => {
    self.op_mov(MCS51_ADDRESSING::INDIRECT_Ri(1), MCS51_ADDRESSING::ACCUMULATOR);
    self.opcode_additional_work("MOV", 0, 1);
},
0xF8 => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(0), MCS51_ADDRESSING::ACCUMULATOR);
    self.opcode_additional_work("MOV", 0, 1);
},
0xF9 => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(1), MCS51_ADDRESSING::ACCUMULATOR);
    self.opcode_additional_work("MOV", 0, 1);
},
0xFA => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(2), MCS51_ADDRESSING::ACCUMULATOR);
    self.opcode_additional_work("MOV", 0, 1);
},
0xFB => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(3), MCS51_ADDRESSING::ACCUMULATOR);
    self.opcode_additional_work("MOV", 0, 1);
},
0xFC => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(4), MCS51_ADDRESSING::ACCUMULATOR);
    self.opcode_additional_work("MOV", 0, 1);
},
0xFD => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(5), MCS51_ADDRESSING::ACCUMULATOR);
    self.opcode_additional_work("MOV", 0, 1);
},
0xFE => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(6), MCS51_ADDRESSING::ACCUMULATOR);
    self.opcode_additional_work("MOV", 0, 1);
},
0xFF => {
    self.op_mov(MCS51_ADDRESSING::REGISTER(7), MCS51_ADDRESSING::ACCUMULATOR);
    self.opcode_additional_work("MOV", 0, 1);
},