mod lib;
use lib::mcus::mcs51::*;
use lib::mcus::pic16f628a::*;
use std::time::{Duration, Instant};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn register_operations_16f628a() {
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

    #[test]
    fn register_operations_mcs51() {
        let mut mcu = MCS51::new();
        mcu.reset();
        assert_eq!(mcu.get_accumulator(), 0);
        mcu.set_program(vec![
            0x04, // Increment Accumulator
            0x04, // Increment Accumulator
            0x04, // Increment Accumulator
            0x09, // Increment Register 1
            0x09, // Increment Register 1
            0x14, // Decrement Accumulator
            0x14, // Decrement Accumulator
            0x19, // Decrement Register 1
            0x09, // Increment Register 1
        ]);
        mcu.clock();
        assert_eq!(mcu.get_accumulator(), 1);
        mcu.clock();
        mcu.clock();
        assert_eq!(mcu.get_accumulator(), 3);
        mcu.clock();
        mcu.clock();
        assert_eq!(mcu.read_register(1), 2);
        mcu.clock();
        mcu.clock();
        assert_eq!(mcu.get_accumulator(), 1);
        mcu.clock();
        assert_eq!(mcu.read_register(1), 1);

        //Switch bank
        mcu.write(0xD0, 0b00001000);
        mcu.clock();
        assert_eq!(mcu.read_register(1), 1);
    }

    #[test]
    fn banking_operations_mcs51() {
        let mut mcu = MCS51::new();
        mcu.reset();
        mcu.write(0xD0, 0b00000000);
        assert_eq!(mcu.get_current_register_bank(), 0);
        mcu.write(0xD0, 0b00001000);
        assert_eq!(mcu.get_current_register_bank(), 1);
        mcu.write(0xD0, 0b00010000);
        assert_eq!(mcu.get_current_register_bank(), 2);
        mcu.write(0xD0, 0b00011000);
        assert_eq!(mcu.get_current_register_bank(), 3);
    }

    #[test]
    fn bit_operations_mcs51() {
        let mut mcu = MCS51::new();
        mcu.reset();
        assert_eq!(mcu.read_bit(0x60), false);
        assert_eq!(mcu.read_bit(0x61), false);
        assert_eq!(mcu.read_bit(0x62), false);

        mcu.write_bit(0x60, true);
        assert_eq!(mcu.get_accumulator(), 1);
        assert_eq!(*mcu.read(0x2c).unwrap(), 1);

        mcu.write_bit(0x61, true);
        assert_eq!(mcu.get_accumulator(), 3);
        assert_eq!(*mcu.read(0x2c).unwrap(), 3);
        assert_eq!(mcu.read_bit(0x61), true);

        mcu.write_bit(0x62, true);
        assert_eq!(mcu.get_accumulator(), 7);
        assert_eq!(*mcu.read(0x2c).unwrap(), 7);
        assert_eq!(mcu.read_bit(0x62), true);
    }

    #[test]
    fn bit_mov_operations_mcs51() {
        let mut mcu = MCS51::new();
        mcu.reset();
        mcu.set_program(vec![
            0x04, // Increment Accumulator
            0x04, // Increment Accumulator
            0x04, // Increment Accumulator
            0x09, // Increment Register 1
            0x09, // Increment Register 1
            0x09, // Increment Register 1
            0x74, 0xFE, // Store 0xFE in accumulator
            0x79, 0xFD, // Store 0xFD in R1
        ]);
        for _i in 0..6 {
            mcu.clock();
        }

        mcu.clock();
        assert_eq!(mcu.get_accumulator(), 0xFE);

        mcu.clock();
        assert_eq!(mcu.read_register(1), 0xFD);
    }
}

pub fn test1() {
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

fn main() {
    let a: i8 = -5;
    let b: u8 = a as u8;

    let c: u8 = 254;
    let d: i8 = c as i8;
    println!("{} {}", b, d);
}
