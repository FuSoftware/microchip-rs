mod lib;
use lib::mcus::pic16f628a::*;
use std::time::{Duration, Instant};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_register_operations() {
        let mut mcu = PIC16F628A::new();
        mcu.reset();

        mcu.set_bank(0);
        assert_eq!(
            *mcu.get_register(PIC16F628A_REGISTERS::STATUS).unwrap(),
            0b00011000
        );
        mcu.set_bank(1);
        assert_eq!(
            *mcu.get_register(PIC16F628A_REGISTERS::STATUS).unwrap(),
            0b00111000
        );
        mcu.set_bank(0);

        let porta = mcu.get_register_mut(PIC16F628A_REGISTERS::PORTA).unwrap();
        *porta = 0x0F;
        let porta2 = mcu.get_memory_address_mut(0x05).unwrap();
        assert_eq!(*porta2, 0x0F);
        assert_eq!(
            *mcu.get_register(PIC16F628A_REGISTERS::PORTA).unwrap(),
            0x0F
        );
    }
}

fn main() {
    let mut mcu = PIC16F628A::new();
    mcu.reset();

    mcu.set_memory_address(0x7E, 0x0F);

    let opcode = 0b00110011111110;

    println!(
        "REG: {:08b}, C: {}",
        mcu.get_memory_address(0x7E).unwrap(),
        mcu.get_carry_flag()
    );
    let mut now = Instant::now();
    mcu.run_opcode_old(opcode);
    println!("{}", now.elapsed().as_nanos());
    println!(
        "REG: {:08b}, C: {}",
        mcu.get_memory_address(0x7E).unwrap(),
        mcu.get_carry_flag()
    );
    now = Instant::now();
    mcu.run_opcode(opcode);
    println!("{}", now.elapsed().as_nanos());
    println!(
        "REG: {:08b}, C: {}",
        mcu.get_memory_address(0x7E).unwrap(),
        mcu.get_carry_flag()
    );
}