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
}
