use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stats_ci::utils::KahanSum;

fn bench_add_assign() {
    let repetitions = 1_000_000;
    let mut sum = KahanSum::new(0.0_f32);
    (1..=repetitions).for_each(|_| {
        sum += black_box(0.1);
    });
}

fn bench_kahan(c: &mut Criterion) {
    c.bench_function("KahanSum::add_assign", |b| b.iter(|| bench_add_assign()));
}

criterion_group!(benches, bench_kahan);
criterion_main!(benches);
