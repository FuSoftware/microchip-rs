use criterion::{black_box, criterion_group, criterion_main, Criterion};

use microchip_rs::lib::mcus::mcs51::*;

fn mcs51_benchmark(c: &mut Criterion) {
    let iterations = 1000000;
    let mut mcu = MCS51::new();
    mcu.generate_opcode_array();

    let test_data = vec![
        (0x00, "MCS51 : NOP"),
        (0x04, "MCS51 : INC, A"),
        (0x08, "MCS51 : INC, R0"),
    ];

    for instruction in test_data {
        mcu.reset();
        mcu.set_program(vec![instruction.0; iterations]);
        c.bench_function(&format!("MATCH {}", instruction.1), |b| {
            b.iter(|| mcu.next_instruction_debug_match())
        });

        mcu.reset();
        mcu.set_program(vec![instruction.0; iterations]);
        c.bench_function(&format!("TABLE {}", instruction.1), |b| {
            b.iter(|| mcu.next_instruction_debug_table())
        });
    }
}

criterion_group!(benches, mcs51_benchmark);
criterion_main!(benches);
