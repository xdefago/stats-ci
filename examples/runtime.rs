use rand::{thread_rng, Rng};
use stats_ci::*;
use std::time::Instant;

///
/// Measures the time elapsed to compute a 95% confidence interval for a reasonably large array.
/// Measurements on a 2022 MacBook Air (M2, 24GB RAM, 1TB SSD) with rustc 1.62.2 yield a runtime of
/// about 50ms for an array of 12.5M elements (when compiled in --release).
/// Unfortunately, the runtime seems to hang (or get very slow) when the array size is increased to
/// 15M elements. This could be due to cache issues? I'm not sure.
///
fn main() {
    type Float = f64;
    const POPULATION_SIZE: usize = 12_500_000;
    let mut rng = thread_rng();
    let mut population = vec![0 as Float; POPULATION_SIZE];
    rng.fill(&mut population[..]);

    let start = Instant::now();
    let ci = mean::Arithmetic::ci(Confidence::new_two_sided(0.95), population).unwrap();
    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);
    println!("CI: {:?}", ci);
}
