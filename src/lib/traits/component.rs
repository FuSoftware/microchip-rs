pub trait IOComponent {
    fn get_pin(&self, pin: usize) -> bool;
    fn set_pin(&mut self, pin: usize, val: bool);

    fn get_port_u8(&self, port: usize) -> u8;
    fn set_port_u8(&mut self, port: usize, val: u8);

    fn get_port_u16(&self, port: usize) -> u16;
    fn set_port_u16(&mut self, port: usize, val: u16);
}

pub trait MCU<T> {
    fn clock(&mut self);
    fn next_instruction(&mut self);
    fn run_opcode(&mut self, opcode: T);
    fn set_program(&mut self, program: Vec<u8>);
    fn setup(&mut self);
    fn reset(&mut self);
    fn run(&mut self);
}