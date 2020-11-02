self.dispatch[0x00] = |cpu: &mut MCS51| {
    cpu.op_nop();
    cpu.opcode_additional_work("NOP", 0, 1);
};
self.dispatch[0x01] = |cpu: &mut MCS51| {
    cpu.op_ajmp();
    cpu.opcode_additional_work("AJMP", 1, 0);
};
self.dispatch[0x02] = |cpu: &mut MCS51| {
    cpu.op_ljmp(MCS51_ADDRESSING::ADDR_16);
    cpu.opcode_additional_work("LJMP", 1, 0);
};
self.dispatch[0x03] = |cpu: &mut MCS51| {
    cpu.op_rr();
    cpu.opcode_additional_work("RR", 0, 1);
};
self.dispatch[0x04] = |cpu: &mut MCS51| {
    cpu.op_inc(MCS51_ADDRESSING::ACCUMULATOR);
    cpu.opcode_additional_work("INC", 0, 1);
};
self.dispatch[0x05] = |cpu: &mut MCS51| {
    cpu.op_inc(MCS51_ADDRESSING::DIRECT(1));
    cpu.opcode_additional_work("INC", 0, 2);
};
self.dispatch[0x06] = |cpu: &mut MCS51| {
    cpu.op_inc(MCS51_ADDRESSING::INDIRECT_Ri(0));
    cpu.opcode_additional_work("INC", 0, 1);
};
self.dispatch[0x07] = |cpu: &mut MCS51| {
    cpu.op_inc(MCS51_ADDRESSING::INDIRECT_Ri(1));
    cpu.opcode_additional_work("INC", 0, 1);
};
self.dispatch[0x08] = |cpu: &mut MCS51| {
    cpu.op_inc(MCS51_ADDRESSING::REGISTER(0));
    cpu.opcode_additional_work("INC", 0, 1);
};
self.dispatch[0x09] = |cpu: &mut MCS51| {
    cpu.op_inc(MCS51_ADDRESSING::REGISTER(1));
    cpu.opcode_additional_work("INC", 0, 1);
};
self.dispatch[0x0A] = |cpu: &mut MCS51| {
    cpu.op_inc(MCS51_ADDRESSING::REGISTER(2));
    cpu.opcode_additional_work("INC", 0, 1);
};
self.dispatch[0x0B] = |cpu: &mut MCS51| {
    cpu.op_inc(MCS51_ADDRESSING::REGISTER(3));
    cpu.opcode_additional_work("INC", 0, 1);
};
self.dispatch[0x0C] = |cpu: &mut MCS51| {
    cpu.op_inc(MCS51_ADDRESSING::REGISTER(4));
    cpu.opcode_additional_work("INC", 0, 1);
};
self.dispatch[0x0D] = |cpu: &mut MCS51| {
    cpu.op_inc(MCS51_ADDRESSING::REGISTER(5));
    cpu.opcode_additional_work("INC", 0, 1);
};
self.dispatch[0x0E] = |cpu: &mut MCS51| {
    cpu.op_inc(MCS51_ADDRESSING::REGISTER(6));
    cpu.opcode_additional_work("INC", 0, 1);
};
self.dispatch[0x0F] = |cpu: &mut MCS51| {
    cpu.op_inc(MCS51_ADDRESSING::REGISTER(7));
    cpu.opcode_additional_work("INC", 0, 1);
};
self.dispatch[0x10] = |cpu: &mut MCS51| {
    cpu.op_jbc(MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    cpu.opcode_additional_work("JBC", 1, 0);
};
self.dispatch[0x11] = |cpu: &mut MCS51| {
    cpu.op_acall();
    cpu.opcode_additional_work("ACALL", 1, 0);
};
self.dispatch[0x12] = |cpu: &mut MCS51| {
    cpu.op_lcall(MCS51_ADDRESSING::ADDR_16);
    cpu.opcode_additional_work("LCALL", 1, 0);
};
self.dispatch[0x13] = |cpu: &mut MCS51| {
    cpu.op_rrc();
    cpu.opcode_additional_work("RRC", 0, 1);
};
self.dispatch[0x14] = |cpu: &mut MCS51| {
    cpu.op_dec(MCS51_ADDRESSING::ACCUMULATOR);
    cpu.opcode_additional_work("DEC", 0, 1);
};
self.dispatch[0x15] = |cpu: &mut MCS51| {
    cpu.op_dec(MCS51_ADDRESSING::DIRECT(1));
    cpu.opcode_additional_work("DEC", 0, 2);
};
self.dispatch[0x16] = |cpu: &mut MCS51| {
    cpu.op_dec(MCS51_ADDRESSING::INDIRECT_Ri(0));
    cpu.opcode_additional_work("DEC", 0, 1);
};
self.dispatch[0x17] = |cpu: &mut MCS51| {
    cpu.op_dec(MCS51_ADDRESSING::INDIRECT_Ri(1));
    cpu.opcode_additional_work("DEC", 0, 1);
};
self.dispatch[0x18] = |cpu: &mut MCS51| {
    cpu.op_dec(MCS51_ADDRESSING::REGISTER(0));
    cpu.opcode_additional_work("DEC", 0, 1);
};
self.dispatch[0x19] = |cpu: &mut MCS51| {
    cpu.op_dec(MCS51_ADDRESSING::REGISTER(1));
    cpu.opcode_additional_work("DEC", 0, 1);
};
self.dispatch[0x1A] = |cpu: &mut MCS51| {
    cpu.op_dec(MCS51_ADDRESSING::REGISTER(2));
    cpu.opcode_additional_work("DEC", 0, 1);
};
self.dispatch[0x1B] = |cpu: &mut MCS51| {
    cpu.op_dec(MCS51_ADDRESSING::REGISTER(3));
    cpu.opcode_additional_work("DEC", 0, 1);
};
self.dispatch[0x1C] = |cpu: &mut MCS51| {
    cpu.op_dec(MCS51_ADDRESSING::REGISTER(4));
    cpu.opcode_additional_work("DEC", 0, 1);
};
self.dispatch[0x1D] = |cpu: &mut MCS51| {
    cpu.op_dec(MCS51_ADDRESSING::REGISTER(5));
    cpu.opcode_additional_work("DEC", 0, 1);
};
self.dispatch[0x1E] = |cpu: &mut MCS51| {
    cpu.op_dec(MCS51_ADDRESSING::REGISTER(6));
    cpu.opcode_additional_work("DEC", 0, 1);
};
self.dispatch[0x1F] = |cpu: &mut MCS51| {
    cpu.op_dec(MCS51_ADDRESSING::REGISTER(7));
    cpu.opcode_additional_work("DEC", 0, 1);
};
self.dispatch[0x20] = |cpu: &mut MCS51| {
    cpu.op_jb(MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    cpu.opcode_additional_work("JB", 1, 0);
};
self.dispatch[0x21] = |cpu: &mut MCS51| {
    cpu.op_ajmp();
    cpu.opcode_additional_work("AJMP", 1, 0);
};
self.dispatch[0x22] = |cpu: &mut MCS51| {
    cpu.op_ret();
    cpu.opcode_additional_work("RET", 1, 0);
};
self.dispatch[0x23] = |cpu: &mut MCS51| {
    cpu.op_rl();
    cpu.opcode_additional_work("RL", 0, 1);
};
self.dispatch[0x24] = |cpu: &mut MCS51| {
    cpu.op_add(MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("ADD", 0, 2);
};
self.dispatch[0x25] = |cpu: &mut MCS51| {
    cpu.op_add(MCS51_ADDRESSING::DIRECT(1));
    cpu.opcode_additional_work("ADD", 0, 2);
};
self.dispatch[0x26] = |cpu: &mut MCS51| {
    cpu.op_add(MCS51_ADDRESSING::INDIRECT_Ri(0));
    cpu.opcode_additional_work("ADD", 0, 1);
};
self.dispatch[0x27] = |cpu: &mut MCS51| {
    cpu.op_add(MCS51_ADDRESSING::INDIRECT_Ri(1));
    cpu.opcode_additional_work("ADD", 0, 1);
};
self.dispatch[0x28] = |cpu: &mut MCS51| {
    cpu.op_add(MCS51_ADDRESSING::REGISTER(0));
    cpu.opcode_additional_work("ADD", 0, 1);
};
self.dispatch[0x29] = |cpu: &mut MCS51| {
    cpu.op_add(MCS51_ADDRESSING::REGISTER(1));
    cpu.opcode_additional_work("ADD", 0, 1);
};
self.dispatch[0x2A] = |cpu: &mut MCS51| {
    cpu.op_add(MCS51_ADDRESSING::REGISTER(2));
    cpu.opcode_additional_work("ADD", 0, 1);
};
self.dispatch[0x2B] = |cpu: &mut MCS51| {
    cpu.op_add(MCS51_ADDRESSING::REGISTER(3));
    cpu.opcode_additional_work("ADD", 0, 1);
};
self.dispatch[0x2C] = |cpu: &mut MCS51| {
    cpu.op_add(MCS51_ADDRESSING::REGISTER(4));
    cpu.opcode_additional_work("ADD", 0, 1);
};
self.dispatch[0x2D] = |cpu: &mut MCS51| {
    cpu.op_add(MCS51_ADDRESSING::REGISTER(5));
    cpu.opcode_additional_work("ADD", 0, 1);
};
self.dispatch[0x2E] = |cpu: &mut MCS51| {
    cpu.op_add(MCS51_ADDRESSING::REGISTER(6));
    cpu.opcode_additional_work("ADD", 0, 1);
};
self.dispatch[0x2F] = |cpu: &mut MCS51| {
    cpu.op_add(MCS51_ADDRESSING::REGISTER(7));
    cpu.opcode_additional_work("ADD", 0, 1);
};
self.dispatch[0x30] = |cpu: &mut MCS51| {
    cpu.op_jnb(MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    cpu.opcode_additional_work("JNB", 1, 0);
};
self.dispatch[0x31] = |cpu: &mut MCS51| {
    cpu.op_acall();
    cpu.opcode_additional_work("ACALL", 1, 0);
};
self.dispatch[0x32] = |cpu: &mut MCS51| {
    cpu.op_reti();
    cpu.opcode_additional_work("RETI", 1, 0);
};
self.dispatch[0x33] = |cpu: &mut MCS51| {
    cpu.op_rlc();
    cpu.opcode_additional_work("RLC", 1, 1);
};
self.dispatch[0x34] = |cpu: &mut MCS51| {
    cpu.op_addc(MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("ADDC", 0, 2);
};
self.dispatch[0x35] = |cpu: &mut MCS51| {
    cpu.op_addc(MCS51_ADDRESSING::DIRECT(1));
    cpu.opcode_additional_work("ADDC", 0, 2);
};
self.dispatch[0x36] = |cpu: &mut MCS51| {
    cpu.op_addc(MCS51_ADDRESSING::INDIRECT_Ri(0));
    cpu.opcode_additional_work("ADDC", 0, 1);
};
self.dispatch[0x37] = |cpu: &mut MCS51| {
    cpu.op_addc(MCS51_ADDRESSING::INDIRECT_Ri(1));
    cpu.opcode_additional_work("ADDC", 0, 1);
};
self.dispatch[0x38] = |cpu: &mut MCS51| {
    cpu.op_addc(MCS51_ADDRESSING::REGISTER(0));
    cpu.opcode_additional_work("ADDC", 0, 1);
};
self.dispatch[0x39] = |cpu: &mut MCS51| {
    cpu.op_addc(MCS51_ADDRESSING::REGISTER(1));
    cpu.opcode_additional_work("ADDC", 0, 1);
};
self.dispatch[0x3A] = |cpu: &mut MCS51| {
    cpu.op_addc(MCS51_ADDRESSING::REGISTER(2));
    cpu.opcode_additional_work("ADDC", 0, 1);
};
self.dispatch[0x3B] = |cpu: &mut MCS51| {
    cpu.op_addc(MCS51_ADDRESSING::REGISTER(3));
    cpu.opcode_additional_work("ADDC", 0, 1);
};
self.dispatch[0x3C] = |cpu: &mut MCS51| {
    cpu.op_addc(MCS51_ADDRESSING::REGISTER(4));
    cpu.opcode_additional_work("ADDC", 0, 1);
};
self.dispatch[0x3D] = |cpu: &mut MCS51| {
    cpu.op_addc(MCS51_ADDRESSING::REGISTER(5));
    cpu.opcode_additional_work("ADDC", 0, 1);
};
self.dispatch[0x3E] = |cpu: &mut MCS51| {
    cpu.op_addc(MCS51_ADDRESSING::REGISTER(6));
    cpu.opcode_additional_work("ADDC", 0, 1);
};
self.dispatch[0x3F] = |cpu: &mut MCS51| {
    cpu.op_addc(MCS51_ADDRESSING::REGISTER(7));
    cpu.opcode_additional_work("ADDC", 0, 1);
};
self.dispatch[0x40] = |cpu: &mut MCS51| {
    cpu.op_jc(MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("JC", 1, 0);
};
self.dispatch[0x41] = |cpu: &mut MCS51| {
    cpu.op_ajmp();
    cpu.opcode_additional_work("AJMP", 1, 0);
};
self.dispatch[0x42] = |cpu: &mut MCS51| {
    cpu.op_orl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::ACCUMULATOR);
    cpu.opcode_additional_work("ORL", 0, 2);
};
self.dispatch[0x43] = |cpu: &mut MCS51| {
    cpu.op_orl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::DATA(2));
    cpu.opcode_additional_work("ORL", 1, 3);
};
self.dispatch[0x44] = |cpu: &mut MCS51| {
    cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("ORL", 0, 2);
};
self.dispatch[0x45] = |cpu: &mut MCS51| {
    cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DIRECT(1));
    cpu.opcode_additional_work("ORL", 0, 2);
};
self.dispatch[0x46] = |cpu: &mut MCS51| {
    cpu.op_orl(
        MCS51_ADDRESSING::ACCUMULATOR,
        MCS51_ADDRESSING::INDIRECT_Ri(0),
    );
    cpu.opcode_additional_work("ORL", 0, 1);
};
self.dispatch[0x47] = |cpu: &mut MCS51| {
    cpu.op_orl(
        MCS51_ADDRESSING::ACCUMULATOR,
        MCS51_ADDRESSING::INDIRECT_Ri(1),
    );
    cpu.opcode_additional_work("ORL", 0, 1);
};
self.dispatch[0x48] = |cpu: &mut MCS51| {
    cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(0));
    cpu.opcode_additional_work("ORL", 0, 1);
};
self.dispatch[0x49] = |cpu: &mut MCS51| {
    cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(1));
    cpu.opcode_additional_work("ORL", 0, 1);
};
self.dispatch[0x4A] = |cpu: &mut MCS51| {
    cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(2));
    cpu.opcode_additional_work("ORL", 0, 1);
};
self.dispatch[0x4B] = |cpu: &mut MCS51| {
    cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(3));
    cpu.opcode_additional_work("ORL", 0, 1);
};
self.dispatch[0x4C] = |cpu: &mut MCS51| {
    cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(4));
    cpu.opcode_additional_work("ORL", 0, 1);
};
self.dispatch[0x4D] = |cpu: &mut MCS51| {
    cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(5));
    cpu.opcode_additional_work("ORL", 0, 1);
};
self.dispatch[0x4E] = |cpu: &mut MCS51| {
    cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(6));
    cpu.opcode_additional_work("ORL", 0, 1);
};
self.dispatch[0x4F] = |cpu: &mut MCS51| {
    cpu.op_orl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(7));
    cpu.opcode_additional_work("ORL", 0, 1);
};
self.dispatch[0x50] = |cpu: &mut MCS51| {
    cpu.op_jnc(MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("JNC", 1, 0);
};
self.dispatch[0x51] = |cpu: &mut MCS51| {
    cpu.op_acall();
    cpu.opcode_additional_work("ACALL", 1, 0);
};
self.dispatch[0x52] = |cpu: &mut MCS51| {
    cpu.op_anl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::ACCUMULATOR);
    cpu.opcode_additional_work("ANL", 0, 2);
};
self.dispatch[0x53] = |cpu: &mut MCS51| {
    cpu.op_anl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::DATA(2));
    cpu.opcode_additional_work("ANL", 1, 3);
};
self.dispatch[0x54] = |cpu: &mut MCS51| {
    cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("ANL", 0, 2);
};
self.dispatch[0x55] = |cpu: &mut MCS51| {
    cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DIRECT(1));
    cpu.opcode_additional_work("ANL", 0, 2);
};
self.dispatch[0x56] = |cpu: &mut MCS51| {
    cpu.op_anl(
        MCS51_ADDRESSING::ACCUMULATOR,
        MCS51_ADDRESSING::INDIRECT_Ri(0),
    );
    cpu.opcode_additional_work("ANL", 0, 1);
};
self.dispatch[0x57] = |cpu: &mut MCS51| {
    cpu.op_anl(
        MCS51_ADDRESSING::ACCUMULATOR,
        MCS51_ADDRESSING::INDIRECT_Ri(1),
    );
    cpu.opcode_additional_work("ANL", 0, 1);
};
self.dispatch[0x58] = |cpu: &mut MCS51| {
    cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(0));
    cpu.opcode_additional_work("ANL", 0, 1);
};
self.dispatch[0x59] = |cpu: &mut MCS51| {
    cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(1));
    cpu.opcode_additional_work("ANL", 0, 1);
};
self.dispatch[0x5A] = |cpu: &mut MCS51| {
    cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(2));
    cpu.opcode_additional_work("ANL", 0, 1);
};
self.dispatch[0x5B] = |cpu: &mut MCS51| {
    cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(3));
    cpu.opcode_additional_work("ANL", 0, 1);
};
self.dispatch[0x5C] = |cpu: &mut MCS51| {
    cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(4));
    cpu.opcode_additional_work("ANL", 0, 1);
};
self.dispatch[0x5D] = |cpu: &mut MCS51| {
    cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(5));
    cpu.opcode_additional_work("ANL", 0, 1);
};
self.dispatch[0x5E] = |cpu: &mut MCS51| {
    cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(6));
    cpu.opcode_additional_work("ANL", 0, 1);
};
self.dispatch[0x5F] = |cpu: &mut MCS51| {
    cpu.op_anl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(7));
    cpu.opcode_additional_work("ANL", 0, 1);
};
self.dispatch[0x60] = |cpu: &mut MCS51| {
    cpu.op_jz(MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("JZ", 1, 0);
};
self.dispatch[0x61] = |cpu: &mut MCS51| {
    cpu.op_ajmp();
    cpu.opcode_additional_work("AJMP", 1, 0);
};
self.dispatch[0x62] = |cpu: &mut MCS51| {
    cpu.op_xrl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::ACCUMULATOR);
    cpu.opcode_additional_work("XRL", 0, 2);
};
self.dispatch[0x63] = |cpu: &mut MCS51| {
    cpu.op_orl(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::DATA(2));
    cpu.opcode_additional_work("XRL", 1, 3);
};
self.dispatch[0x64] = |cpu: &mut MCS51| {
    cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("XRL", 0, 2);
};
self.dispatch[0x65] = |cpu: &mut MCS51| {
    cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DIRECT(1));
    cpu.opcode_additional_work("XRL", 0, 2);
};
self.dispatch[0x66] = |cpu: &mut MCS51| {
    cpu.op_xrl(
        MCS51_ADDRESSING::ACCUMULATOR,
        MCS51_ADDRESSING::INDIRECT_Ri(0),
    );
    cpu.opcode_additional_work("XRL", 0, 1);
};
self.dispatch[0x67] = |cpu: &mut MCS51| {
    cpu.op_xrl(
        MCS51_ADDRESSING::ACCUMULATOR,
        MCS51_ADDRESSING::INDIRECT_Ri(1),
    );
    cpu.opcode_additional_work("XRL", 0, 1);
};
self.dispatch[0x68] = |cpu: &mut MCS51| {
    cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(0));
    cpu.opcode_additional_work("XRL", 0, 1);
};
self.dispatch[0x69] = |cpu: &mut MCS51| {
    cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(1));
    cpu.opcode_additional_work("XRL", 0, 1);
};
self.dispatch[0x6A] = |cpu: &mut MCS51| {
    cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(2));
    cpu.opcode_additional_work("XRL", 0, 1);
};
self.dispatch[0x6B] = |cpu: &mut MCS51| {
    cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(3));
    cpu.opcode_additional_work("XRL", 0, 1);
};
self.dispatch[0x6C] = |cpu: &mut MCS51| {
    cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(4));
    cpu.opcode_additional_work("XRL", 0, 1);
};
self.dispatch[0x6D] = |cpu: &mut MCS51| {
    cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(5));
    cpu.opcode_additional_work("XRL", 0, 1);
};
self.dispatch[0x6E] = |cpu: &mut MCS51| {
    cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(6));
    cpu.opcode_additional_work("XRL", 0, 1);
};
self.dispatch[0x6F] = |cpu: &mut MCS51| {
    cpu.op_xrl(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::REGISTER(7));
    cpu.opcode_additional_work("XRL", 0, 1);
};
self.dispatch[0x70] = |cpu: &mut MCS51| {
    cpu.op_jnz(MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("JNZ", 1, 0);
};
self.dispatch[0x71] = |cpu: &mut MCS51| {
    cpu.op_acall();
    cpu.opcode_additional_work("ACALL", 1, 0);
};
self.dispatch[0x72] = |cpu: &mut MCS51| {
    cpu.op_orl_c(MCS51_ADDRESSING::DATA(1), false);
    cpu.opcode_additional_work("ORL", 1, 2);
};
self.dispatch[0x73] = |cpu: &mut MCS51| {
    cpu.op_jmp();
    cpu.opcode_additional_work("JMP", 1, 1);
};
self.dispatch[0x74] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("MOV", 0, 2);
};
self.dispatch[0x75] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::DATA(2));
    cpu.opcode_additional_work("MOV", 0, 3);
};
self.dispatch[0x76] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::INDIRECT_Ri(0), MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("MOV", 0, 2);
};
self.dispatch[0x77] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::INDIRECT_Ri(1), MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("MOV", 0, 2);
};
self.dispatch[0x78] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(0), MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("MOV", 0, 2);
};
self.dispatch[0x79] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(1), MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("MOV", 0, 2);
};
self.dispatch[0x7A] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(2), MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("MOV", 0, 2);
};
self.dispatch[0x7B] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(3), MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("MOV", 0, 2);
};
self.dispatch[0x7C] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(4), MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("MOV", 0, 2);
};
self.dispatch[0x7D] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(5), MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("MOV", 0, 2);
};
self.dispatch[0x7E] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(6), MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("MOV", 0, 2);
};
self.dispatch[0x7F] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(7), MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("MOV", 0, 2);
};
self.dispatch[0x80] = |cpu: &mut MCS51| {
    cpu.op_sjmp(MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("SJMP", 1, 1);
};
self.dispatch[0x81] = |cpu: &mut MCS51| {
    cpu.op_ajmp();
    cpu.opcode_additional_work("AJMP", 1, 1);
};
self.dispatch[0x82] = |cpu: &mut MCS51| {
    cpu.op_anl_c(MCS51_ADDRESSING::DATA(1), false);
    cpu.opcode_additional_work("ANL", 1, 2);
};
self.dispatch[0x83] = |cpu: &mut MCS51| {
    cpu.op_movc_pc();
    cpu.opcode_additional_work("MOVC", 1, 1);
};
self.dispatch[0x84] = |cpu: &mut MCS51| {
    cpu.op_div();
    cpu.opcode_additional_work("DIV", 3, 1);
};
self.dispatch[0x85] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::DIRECT(2));
    cpu.opcode_additional_work("MOV", 1, 3);
};
self.dispatch[0x86] = |cpu: &mut MCS51| {
    cpu.op_mov(
        MCS51_ADDRESSING::DIRECT(1),
        MCS51_ADDRESSING::INDIRECT_Ri(0),
    );
    cpu.opcode_additional_work("MOV", 1, 2);
};
self.dispatch[0x87] = |cpu: &mut MCS51| {
    cpu.op_mov(
        MCS51_ADDRESSING::DIRECT(1),
        MCS51_ADDRESSING::INDIRECT_Ri(1),
    );
    cpu.opcode_additional_work("MOV", 1, 2);
};
self.dispatch[0x88] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(0));
    cpu.opcode_additional_work("MOV", 1, 2);
};
self.dispatch[0x89] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(1));
    cpu.opcode_additional_work("MOV", 1, 2);
};
self.dispatch[0x8A] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(2));
    cpu.opcode_additional_work("MOV", 1, 2);
};
self.dispatch[0x8B] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(3));
    cpu.opcode_additional_work("MOV", 1, 2);
};
self.dispatch[0x8C] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(4));
    cpu.opcode_additional_work("MOV", 1, 2);
};
self.dispatch[0x8D] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(5));
    cpu.opcode_additional_work("MOV", 1, 2);
};
self.dispatch[0x8E] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(6));
    cpu.opcode_additional_work("MOV", 1, 2);
};
self.dispatch[0x8F] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::REGISTER(7));
    cpu.opcode_additional_work("MOV", 1, 2);
};
self.dispatch[0x90] = |cpu: &mut MCS51| {
    cpu.op_mov_dptr(MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("MOV", 1, 3);
};
self.dispatch[0x91] = |cpu: &mut MCS51| {
    cpu.op_acall();
    cpu.opcode_additional_work("ACALL", 1, 1);
};
self.dispatch[0x92] = |cpu: &mut MCS51| {
    cpu.op_mov_bit_c(MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("MOV", 1, 2);
};
self.dispatch[0x93] = |cpu: &mut MCS51| {
    cpu.op_movc_dptr();
    cpu.opcode_additional_work("MOVC", 1, 1);
};
self.dispatch[0x94] = |cpu: &mut MCS51| {
    cpu.op_subb(MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("SUBB", 0, 2);
};
self.dispatch[0x95] = |cpu: &mut MCS51| {
    cpu.op_subb(MCS51_ADDRESSING::DIRECT(1));
    cpu.opcode_additional_work("SUBB", 0, 2);
};
self.dispatch[0x96] = |cpu: &mut MCS51| {
    cpu.op_subb(MCS51_ADDRESSING::INDIRECT_Ri(0));
    cpu.opcode_additional_work("SUBB", 0, 1);
};
self.dispatch[0x97] = |cpu: &mut MCS51| {
    cpu.op_subb(MCS51_ADDRESSING::INDIRECT_Ri(1));
    cpu.opcode_additional_work("SUBB", 0, 1);
};
self.dispatch[0x98] = |cpu: &mut MCS51| {
    cpu.op_subb(MCS51_ADDRESSING::REGISTER(0));
    cpu.opcode_additional_work("SUBB", 0, 1);
};
self.dispatch[0x99] = |cpu: &mut MCS51| {
    cpu.op_subb(MCS51_ADDRESSING::REGISTER(1));
    cpu.opcode_additional_work("SUBB", 0, 1);
};
self.dispatch[0x9A] = |cpu: &mut MCS51| {
    cpu.op_subb(MCS51_ADDRESSING::REGISTER(2));
    cpu.opcode_additional_work("SUBB", 0, 1);
};
self.dispatch[0x9B] = |cpu: &mut MCS51| {
    cpu.op_subb(MCS51_ADDRESSING::REGISTER(3));
    cpu.opcode_additional_work("SUBB", 0, 1);
};
self.dispatch[0x9C] = |cpu: &mut MCS51| {
    cpu.op_subb(MCS51_ADDRESSING::REGISTER(4));
    cpu.opcode_additional_work("SUBB", 0, 1);
};
self.dispatch[0x9D] = |cpu: &mut MCS51| {
    cpu.op_subb(MCS51_ADDRESSING::REGISTER(5));
    cpu.opcode_additional_work("SUBB", 0, 1);
};
self.dispatch[0x9E] = |cpu: &mut MCS51| {
    cpu.op_subb(MCS51_ADDRESSING::REGISTER(6));
    cpu.opcode_additional_work("SUBB", 0, 1);
};
self.dispatch[0x9F] = |cpu: &mut MCS51| {
    cpu.op_subb(MCS51_ADDRESSING::REGISTER(7));
    cpu.opcode_additional_work("SUBB", 0, 1);
};
self.dispatch[0xA0] = |cpu: &mut MCS51| {
    cpu.op_orl_c(MCS51_ADDRESSING::DATA(1), true);
    cpu.opcode_additional_work("ORLC", 1, 2);
};
self.dispatch[0xA1] = |cpu: &mut MCS51| {
    cpu.op_ajmp();
    cpu.opcode_additional_work("AJMP", 1, 1);
};
self.dispatch[0xA2] = |cpu: &mut MCS51| {
    cpu.op_mov_c_bit(MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("MOV", 0, 2);
};
self.dispatch[0xA3] = |cpu: &mut MCS51| {
    cpu.op_inc_dptr();
    cpu.opcode_additional_work("INC DPTR", 1, 1);
};
self.dispatch[0xA4] = |cpu: &mut MCS51| {
    cpu.op_mul();
    cpu.opcode_additional_work("MUL", 3, 1)
};
self.dispatch[0xA5] = |cpu: &mut MCS51| { cpu.opcode_additional_work("RESERVED", 0, 0); };
self.dispatch[0xA6] = |cpu: &mut MCS51| {
    cpu.op_mov(
        MCS51_ADDRESSING::INDIRECT_Ri(0),
        MCS51_ADDRESSING::DIRECT(1),
    );
    cpu.opcode_additional_work("MOV", 1, 2)
};
self.dispatch[0xA7] = |cpu: &mut MCS51| {
    cpu.op_mov(
        MCS51_ADDRESSING::INDIRECT_Ri(1),
        MCS51_ADDRESSING::DIRECT(1),
    );
    cpu.opcode_additional_work("MOV", 1, 2)
};
self.dispatch[0xA8] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(0), MCS51_ADDRESSING::DIRECT(1));
    cpu.opcode_additional_work("MOV", 1, 2)
};
self.dispatch[0xA9] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(1), MCS51_ADDRESSING::DIRECT(1));
    cpu.opcode_additional_work("MOV", 1, 2)
};
self.dispatch[0xAA] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(2), MCS51_ADDRESSING::DIRECT(1));
    cpu.opcode_additional_work("MOV", 1, 2)
};
self.dispatch[0xAB] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(3), MCS51_ADDRESSING::DIRECT(1));
    cpu.opcode_additional_work("MOV", 1, 2)
};
self.dispatch[0xAC] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(4), MCS51_ADDRESSING::DIRECT(1));
    cpu.opcode_additional_work("MOV", 1, 2)
};
self.dispatch[0xAD] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(5), MCS51_ADDRESSING::DIRECT(1));
    cpu.opcode_additional_work("MOV", 1, 2)
};
self.dispatch[0xAE] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(6), MCS51_ADDRESSING::DIRECT(1));
    cpu.opcode_additional_work("MOV", 1, 2)
};
self.dispatch[0xAF] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(7), MCS51_ADDRESSING::DIRECT(1));
    cpu.opcode_additional_work("MOV", 1, 2)
};
self.dispatch[0xB0] = |cpu: &mut MCS51| {
    cpu.op_anl_c(MCS51_ADDRESSING::DIRECT(1), true);
    cpu.opcode_additional_work("ANL", 1, 2)
};
self.dispatch[0xB1] = |cpu: &mut MCS51| {};
self.dispatch[0xB2] = |cpu: &mut MCS51| {};
self.dispatch[0xB3] = |cpu: &mut MCS51| {};
self.dispatch[0xB4] = |cpu: &mut MCS51| {
    cpu.op_cjne(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    cpu.opcode_additional_work("CJNE", 2, 0)
};
self.dispatch[0xB5] = |cpu: &mut MCS51| {
    cpu.op_cjne(MCS51_ADDRESSING::ACCUMULATOR, MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::DATA(2));
    cpu.opcode_additional_work("CJNE", 2, 0)
};
self.dispatch[0xB6] = |cpu: &mut MCS51| {
    cpu.op_cjne(MCS51_ADDRESSING::INDIRECT_Ri(0), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    cpu.opcode_additional_work("CJNE", 2, 0)
};
self.dispatch[0xB7] = |cpu: &mut MCS51| {
    cpu.op_cjne(MCS51_ADDRESSING::INDIRECT_Ri(1), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    cpu.opcode_additional_work("CJNE", 2, 0)
};
self.dispatch[0xB8] = |cpu: &mut MCS51| {
    cpu.op_cjne(MCS51_ADDRESSING::REGISTER(0), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    cpu.opcode_additional_work("CJNE", 2, 0)
};
self.dispatch[0xB9] = |cpu: &mut MCS51| {
    cpu.op_cjne(MCS51_ADDRESSING::REGISTER(1), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    cpu.opcode_additional_work("CJNE", 2, 0)
};
self.dispatch[0xBA] = |cpu: &mut MCS51| {
    cpu.op_cjne(MCS51_ADDRESSING::REGISTER(2), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    cpu.opcode_additional_work("CJNE", 2, 0)
};
self.dispatch[0xBB] = |cpu: &mut MCS51| {
    cpu.op_cjne(MCS51_ADDRESSING::REGISTER(3), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    cpu.opcode_additional_work("CJNE", 2, 0)
};
self.dispatch[0xBC] = |cpu: &mut MCS51| {
    cpu.op_cjne(MCS51_ADDRESSING::REGISTER(4), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    cpu.opcode_additional_work("CJNE", 2, 0)
};
self.dispatch[0xBD] = |cpu: &mut MCS51| {
    cpu.op_cjne(MCS51_ADDRESSING::REGISTER(5), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    cpu.opcode_additional_work("CJNE", 2, 0)
};
self.dispatch[0xBE] = |cpu: &mut MCS51| {
    cpu.op_cjne(MCS51_ADDRESSING::REGISTER(6), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    cpu.opcode_additional_work("CJNE", 2, 0)
};
self.dispatch[0xBF] = |cpu: &mut MCS51| {
    cpu.op_cjne(MCS51_ADDRESSING::REGISTER(7), MCS51_ADDRESSING::DATA(1), MCS51_ADDRESSING::DATA(2));
    cpu.opcode_additional_work("CJNE", 2, 0)
};
self.dispatch[0xC0] = |cpu: &mut MCS51| {};
self.dispatch[0xC1] = |cpu: &mut MCS51| {};
self.dispatch[0xC2] = |cpu: &mut MCS51| {
    cpu.op_clr(MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("CLR", 2, 2)
};
self.dispatch[0xC3] = |cpu: &mut MCS51| {
    cpu.set_carry_flag(false);
    cpu.opcode_additional_work("CLR C", 1, 1)
};
self.dispatch[0xC4] = |cpu: &mut MCS51| {};
self.dispatch[0xC5] = |cpu: &mut MCS51| {};
self.dispatch[0xC6] = |cpu: &mut MCS51| {};
self.dispatch[0xC7] = |cpu: &mut MCS51| {};
self.dispatch[0xC8] = |cpu: &mut MCS51| {};
self.dispatch[0xC9] = |cpu: &mut MCS51| {};
self.dispatch[0xCA] = |cpu: &mut MCS51| {};
self.dispatch[0xCB] = |cpu: &mut MCS51| {};
self.dispatch[0xCC] = |cpu: &mut MCS51| {};
self.dispatch[0xCD] = |cpu: &mut MCS51| {};
self.dispatch[0xCE] = |cpu: &mut MCS51| {};
self.dispatch[0xCF] = |cpu: &mut MCS51| {};
self.dispatch[0xD0] = |cpu: &mut MCS51| {};
self.dispatch[0xD1] = |cpu: &mut MCS51| {};
self.dispatch[0xD2] = |cpu: &mut MCS51| {};
self.dispatch[0xD3] = |cpu: &mut MCS51| {};
self.dispatch[0xD4] = |cpu: &mut MCS51| {};
self.dispatch[0xD5] = |cpu: &mut MCS51| {};
self.dispatch[0xD6] = |cpu: &mut MCS51| {};
self.dispatch[0xD7] = |cpu: &mut MCS51| {};
self.dispatch[0xD8] = |cpu: &mut MCS51| {
    cpu.op_djnz(MCS51_ADDRESSING::REGISTER(0), MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("DJNZ", 0, 0)
};
self.dispatch[0xD9] = |cpu: &mut MCS51| {
    cpu.op_djnz(MCS51_ADDRESSING::REGISTER(1), MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("DJNZ", 0, 0)
};
self.dispatch[0xDA] = |cpu: &mut MCS51| {
    cpu.op_djnz(MCS51_ADDRESSING::REGISTER(2), MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("DJNZ", 0, 0)
};
self.dispatch[0xDB] = |cpu: &mut MCS51| {
    cpu.op_djnz(MCS51_ADDRESSING::REGISTER(3), MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("DJNZ", 0, 0)
};
self.dispatch[0xDC] = |cpu: &mut MCS51| {
    cpu.op_djnz(MCS51_ADDRESSING::REGISTER(4), MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("DJNZ", 0, 0)
};
self.dispatch[0xDD] = |cpu: &mut MCS51| {
    cpu.op_djnz(MCS51_ADDRESSING::REGISTER(5), MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("DJNZ", 0, 0)
};
self.dispatch[0xDE] = |cpu: &mut MCS51| {
    cpu.op_djnz(MCS51_ADDRESSING::REGISTER(6), MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("DJNZ", 0, 0)
};
self.dispatch[0xDF] = |cpu: &mut MCS51| {
    cpu.op_djnz(MCS51_ADDRESSING::REGISTER(7), MCS51_ADDRESSING::DATA(1));
    cpu.opcode_additional_work("DJNZ", 0, 0)
};
self.dispatch[0xE0] = |cpu: &mut MCS51| {};
self.dispatch[0xE1] = |cpu: &mut MCS51| {};
self.dispatch[0xE2] = |cpu: &mut MCS51| {};
self.dispatch[0xE3] = |cpu: &mut MCS51| {};
self.dispatch[0xE4] = |cpu: &mut MCS51| {
    cpu.op_clr(MCS51_ADDRESSING::ACCUMULATOR);
    cpu.opcode_additional_work("CLR A", 1, 1)
};
self.dispatch[0xE5] = |cpu: &mut MCS51| {};
self.dispatch[0xE6] = |cpu: &mut MCS51| {};
self.dispatch[0xE7] = |cpu: &mut MCS51| {};
self.dispatch[0xE8] = |cpu: &mut MCS51| {};
self.dispatch[0xE9] = |cpu: &mut MCS51| {};
self.dispatch[0xEA] = |cpu: &mut MCS51| {};
self.dispatch[0xEB] = |cpu: &mut MCS51| {};
self.dispatch[0xEC] = |cpu: &mut MCS51| {};
self.dispatch[0xED] = |cpu: &mut MCS51| {};
self.dispatch[0xEE] = |cpu: &mut MCS51| {};
self.dispatch[0xEF] = |cpu: &mut MCS51| {};
self.dispatch[0xF0] = |cpu: &mut MCS51| {};
self.dispatch[0xF1] = |cpu: &mut MCS51| {};
self.dispatch[0xF2] = |cpu: &mut MCS51| {};
self.dispatch[0xF3] = |cpu: &mut MCS51| {};
self.dispatch[0xF4] = |cpu: &mut MCS51| {};
self.dispatch[0xF5] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::DIRECT(1), MCS51_ADDRESSING::ACCUMULATOR);
    cpu.opcode_additional_work("MOV", 0, 2);
};
self.dispatch[0xF6] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::INDIRECT_Ri(0), MCS51_ADDRESSING::ACCUMULATOR);
    cpu.opcode_additional_work("MOV", 0, 1);
};
self.dispatch[0xF7] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::INDIRECT_Ri(1), MCS51_ADDRESSING::ACCUMULATOR);
    cpu.opcode_additional_work("MOV", 0, 1);
};
self.dispatch[0xF8] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(0), MCS51_ADDRESSING::ACCUMULATOR);
    cpu.opcode_additional_work("MOV", 0, 1);
};
self.dispatch[0xF9] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(1), MCS51_ADDRESSING::ACCUMULATOR);
    cpu.opcode_additional_work("MOV", 0, 1);
};
self.dispatch[0xFA] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(2), MCS51_ADDRESSING::ACCUMULATOR);
    cpu.opcode_additional_work("MOV", 0, 1);
};
self.dispatch[0xFB] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(3), MCS51_ADDRESSING::ACCUMULATOR);
    cpu.opcode_additional_work("MOV", 0, 1);
};
self.dispatch[0xFC] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(4), MCS51_ADDRESSING::ACCUMULATOR);
    cpu.opcode_additional_work("MOV", 0, 1);
};
self.dispatch[0xFD] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(5), MCS51_ADDRESSING::ACCUMULATOR);
    cpu.opcode_additional_work("MOV", 0, 1);
};
self.dispatch[0xFE] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(6), MCS51_ADDRESSING::ACCUMULATOR);
    cpu.opcode_additional_work("MOV", 0, 1);
};
self.dispatch[0xFF] = |cpu: &mut MCS51| {
    cpu.op_mov(MCS51_ADDRESSING::REGISTER(7), MCS51_ADDRESSING::ACCUMULATOR);
    cpu.opcode_additional_work("MOV", 0, 1);
};