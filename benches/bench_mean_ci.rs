use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;
use stats_ci::*;

fn bench_mean_arithmetic(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let confidence = Confidence::new_two_sided(0.95);

    let mut group = c.benchmark_group("mean::Arithmetic::ci (f32 vs. f64)");

    for size in [100_000] {
        let data = (0..size).map(|_| rng.gen::<f64>()).collect::<Vec<_>>();

        group.bench_with_input(BenchmarkId::new("clone", size), &data, |b, data| {
            b.iter(|| {
                let _ = data.clone();
            })
        });

        group.bench_with_input(BenchmarkId::new("<f64>", size), &data, |b, data| {
            b.iter(|| mean::Arithmetic::<f64>::ci(confidence, data))
        });

        let data = data.iter().map(|&x| x as f32).collect::<Vec<_>>();
        group.bench_with_input(BenchmarkId::new("<f32>", size), &data, |b, data| {
            b.iter(|| mean::Arithmetic::<f32>::ci(confidence, data))
        });
    }
    group.finish();
}

fn bench_mean_category(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let confidence = Confidence::new_two_sided(0.95);

    let mut group = c.benchmark_group("mean::*::ci (Arithmetic, Harmonic, Geometric)");

    for size in [100_000] {
        let data = (0..size).map(|_| rng.gen::<f64>()).collect::<Vec<_>>();

        group.bench_with_input(BenchmarkId::new("Arithmetic", size), &data, |b, data| {
            b.iter(|| mean::Arithmetic::ci(confidence, data))
        });

        group.bench_with_input(BenchmarkId::new("Harmonic", size), &data, |b, data| {
            b.iter(|| mean::Harmonic::ci(confidence, data))
        });

        group.bench_with_input(BenchmarkId::new("Geometric", size), &data, |b, data| {
            b.iter(|| mean::Geometric::ci(confidence, data))
        });
    }
    group.finish();
}

fn bench_mean_rayon(c: &mut Criterion) {
    use rayon::prelude::*;

    let mut rng = rand::thread_rng();
    let confidence = Confidence::new_two_sided(0.95);

    let mut group = c.benchmark_group("sequential vs. parallel (rayon)");

    for size in [1_000_000] {
        let data = (0..size).map(|_| rng.gen::<f64>()).collect::<Vec<_>>();

        group.bench_with_input(BenchmarkId::new("Sequential", size), &data, |b, data| {
            b.iter(|| mean::Arithmetic::from_iter(data)?.ci_mean(confidence))
        });

        group.bench_with_input(
            BenchmarkId::new("Parallel (rayon/chunks)", size),
            &data,
            |b, data| {
                b.iter(|| {
                    let stats = data
                        .clone()
                        .par_chunks(1000)
                        .map(|chunk| {
                            let mut stats = mean::Arithmetic::new();
                            for x in chunk {
                                stats.append(*x).unwrap();
                            }
                            stats
                        })
                        .reduce(|| mean::Arithmetic::new(), |s1, s2| s1 + s2);

                    stats.ci_mean(confidence)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Parallel (rayon/par_iter)", size),
            &data,
            |b, data| {
                b.iter(|| {
                    let stats = data
                        .clone()
                        .par_iter()
                        .map(|&x| mean::Arithmetic::from_iter(&[x]).unwrap())
                        .reduce(|| mean::Arithmetic::new(), |s1, s2| s1 + s2);

                    stats.ci_mean(confidence)
                })
            },
        );
    }
    group.finish();
}

fn bench_mean(c: &mut Criterion) {
    bench_mean_arithmetic(c);
    bench_mean_category(c);
    bench_mean_rayon(c);
}

criterion_group!(benches, bench_mean);
criterion_main!(benches);
