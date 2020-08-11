use criterion::{black_box, criterion_group, criterion_main, Criterion};

use microchip_rs::lib::mcus::mcs51::*;

fn criterion_benchmark(c: &mut Criterion) {
    let iterations = 1000000;
    let mut mcu = MCS51::new();
    
    mcu.reset();
    mcu.set_program(vec![0x00; iterations]);
    c.bench_function("MCS51 : NOP", |b| b.iter(|| mcu.next_instruction_debug()));

    mcu.reset();
    mcu.set_program(vec![0x04; iterations]);
    //c.bench_function("MCS51 : INC, A", |b| b.iter(|| mcu.next_instruction_debug()));

    mcu.reset();
    mcu.set_program(vec![0x08; iterations]);
    //c.bench_function("MCS51 : INC, R0", |b| b.iter(|| mcu.next_instruction_debug()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);