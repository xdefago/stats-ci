use rand::{thread_rng, Rng};
use rayon::prelude::*;
use stats_ci::*;
use std::time::Instant;

///
/// Measures the time elapsed to compute a 95% confidence interval for a reasonably large array.
/// Measurements on a 2022 MacBook Air (M2, 24GB RAM, 1TB SSD) with rustc 1.62.2 yield a runtime of
/// nearly 50ms for an array of 14M elements (when compiled in --release).
/// Unfortunately, the runtime seems to hang (or get very slow) when the array size is increased to
/// 15M elements. Could this be due to cache issues? I'm not sure.
///
fn main() {
    type Float = f64;
    const POPULATION_SIZE: usize = 14_000_000;
    const CHUNK_SIZE: usize = 10_000;

    let mut rng = thread_rng();
    let mut source_population = vec![0 as Float; POPULATION_SIZE];
    rng.fill(&mut source_population[..]);

    println!("Sequential call:");
    let population = source_population.clone();
    let start = Instant::now();
    let ci = mean::Arithmetic::ci(Confidence::new_two_sided(0.95), population).unwrap();
    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);
    println!("CI: {} (theoretical mean: 0.5)", ci);

    println!("\nParallel call:");
    let population = source_population.clone();
    let start = Instant::now();
    let ci = population
        .par_iter()
        .map(|&x| mean::Arithmetic::from_iter([x]).unwrap())
        .reduce(|| mean::Arithmetic::new(), |s1, s2| s1 + s2)
        .ci_mean(Confidence::new_two_sided(0.95))
        .unwrap();
    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);
    println!("CI: {} (theoretical mean: 0.5)", ci);

    println!("\nParallel call (chunk size: {}):", CHUNK_SIZE);
    let population = source_population.clone();
    let start = Instant::now();
    let ci = population
        .par_chunks(CHUNK_SIZE)
        .map(|chunk| mean::Arithmetic::from_iter(chunk.iter().copied()).unwrap())
        .reduce(|| mean::Arithmetic::new(), |s1, s2| s1 + s2)
        .ci_mean(Confidence::new_two_sided(0.95))
        .unwrap();
    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);
    println!("CI: {} (theoretical mean: 0.5)", ci);
}
