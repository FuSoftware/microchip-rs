use criterion::{black_box, criterion_group, criterion_main, Criterion};

use microchip_rs::lib::mcus::mcs51::*;

fn mcs51_inc_loop(b: u8) {
    let mut mcu = MCS51::new();
    mcu.reset();
    mcu.set_program(vec![
        0x04, // Increment Accumulator
        0x04, // Increment Accumulator
        0x04, // Increment Accumulator
        0x09, // Increment Register 1
        0x09, // Increment Register 1
        0x09, // Increment Register 1
        0x74, b, // Store 0xFE in accumulator
        0x79, b*2, // Store 0xFD in R1
        0x02, 0x00, 0x00 // Jump to beginning
    ]);

    for _i in 0..1000000 {
        mcu.clock();
    }

}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("mcs51_inc_loop", |b| b.iter(|| mcs51_inc_loop(black_box(30))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);