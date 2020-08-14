mod lib;
use lib::decompiler::mcs51::*;
use lib::mcus::mcs51::*;
use lib::mcus::pic16f628a::*;
use std::fs;
use std::fs::File;
use std::io::Read;
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
        mcu.next_instruction();
        assert_eq!(mcu.get_accumulator(), 1);
        mcu.next_instruction();
        mcu.next_instruction();
        assert_eq!(mcu.get_accumulator(), 3);
        mcu.next_instruction();
        mcu.next_instruction();
        assert_eq!(mcu.read_register(1), 2);
        mcu.next_instruction();
        mcu.next_instruction();
        assert_eq!(mcu.get_accumulator(), 1);
        mcu.next_instruction();
        assert_eq!(mcu.read_register(1), 1);

        //Switch bank
        mcu.write(0xD0, 0b00001000);
        mcu.next_instruction();
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
            mcu.next_instruction();
        }

        mcu.next_instruction();
        assert_eq!(mcu.get_accumulator(), 0xFE);

        mcu.next_instruction();
        assert_eq!(mcu.read_register(1), 0xFD);
    }

    #[test]
    fn add_operations_mcs51() {
        let mut mcu = MCS51::new();
        mcu.reset();
        mcu.set_program(vec![
            0x74, 0xC3, // Store 0xC3 in Accumulator
            0x79, 0xAA, // Store 0xAA in R1
            0x29, //Add R1 to accumulator
        ]);
        mcu.next_instruction();
        mcu.next_instruction();
        mcu.next_instruction();
        assert_eq!(mcu.read_register(1), 0xAA);
        assert_eq!(mcu.get_accumulator(), 0x6D);
        assert_eq!(mcu.get_aux_carry_flag(), false);
        assert_eq!(mcu.get_carry_flag(), true);
        assert_eq!(mcu.get_overflow_flag(), true);
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

fn test_emulator_mcs51() {
    let mut mcu = MCS51::new();
    mcu.generate_opcode_array();

    /*
    mcu.set_program(vec![
        0x04, // Increment Accumulator
        0x04, // Increment Accumulator
        0x04, // Increment Accumulator
        0x04, // Increment Accumulator
        0x04, // Increment Accumulator
        0x04, // Increment Accumulator
        0x04, // Increment Accumulator
        0x04, // Increment Accumulator
        0x04, // Increment Accumulator
        0x04, // Increment Accumulator
        0x02, 0x00, 0x00 // Jump to beginning
    ]);
    */

    /*
    mcu.set_program(vec![
        0x08, // Increment R1
        0x08, // Increment R1
        0x08, // Increment R1
        0x08, // Increment R1
        0x08, // Increment R1
        0x08, // Increment R1
        0x08, // Increment R1
        0x08, // Increment R1
        0x08, // Increment R1
        0x08, // Increment R1
        0x02, 0x00, 0x00, // Jump to beginning
    ]);
    */

    
    mcu.set_program(vec![
        0x00, // NOP
        0x00, // NOP
        0x00, // NOP
        0x00, // NOP
        0x00, // NOP
        0x00, // NOP
        0x00, // NOP
        0x00, // NOP
        0x00, // NOP
        0x00, // NOP
        0x02, 0x00, 0x00 // Jump to beginning
    ]);
    

    
    for _j in 0..10 {
        mcu.reset();

        let iterations = 1000000000;
        let now = Instant::now();
        for _i in 0..iterations {
            mcu.next_instruction();
        }

        let time_us = now.elapsed().as_micros();
        let time_ns = now.elapsed().as_nanos();
        let time_us_inst = time_us as f64 / iterations as f64;
        let time_ns_inst = time_ns as f64 / iterations as f64;
        println!(
            "({:.3}ns/inst) ({:.3} GHz) ({:.3} MHz)",
            time_ns_inst,
            1.0 / time_ns_inst,
            1.0 / time_us_inst
        );
    }
    
    /*
    for _i in 0..20 {
        mcu.next_instruction();
        println!("{}",mcu.pc);
    }
    */
}

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).expect("no file found");
    let metadata = std::fs::metadata(filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn main() {
    let a: u8 = 128;
    println!("{}", a as i8);

    //test_emulator_mcs51();
    /*
    let mut dec = MCS51_Decompiler::new();
    dec.program = get_file_as_byte_vec(r#"D:\Perso\Prog\rust\microchip-rs\data\1594462804_raw.bin"#);

    /*
    let mut next: u16 = 0;
    let mut code = String::new();
    for _i in 0..512 {
        let v = dec.get_instruction(next);
        code.push_str(format!("{}", v).as_str());
        code.push('\n');
        next = v.next[0];
    }

    fs::write("data/code.asm", code).expect("Unable to write file");
    */

    dec.decompile(0);
    dec.write_to_file("data/code.asm");
    */
}
