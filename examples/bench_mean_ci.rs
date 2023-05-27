use stats_ci::mean::StatisticsOps;
use stats_ci::*;

use rand::Rng;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_seeder::Seeder;

const SIZE: usize = 1_000_000;

const SEED_STRINGS: [&str; 35] = [
    "Stats-ci provides some basic functions to compute confidence intervals of sample data.",
    "This includes the following:",
    "confidence intervals around the mean for numerical data,",
    "confidence intervals around a quantile (e.g., median) for arbitrary ordered data,",
    "confidence intervals for proportions.",
    "Not included yet but planned are:",
    "confidence intervals for difference of sample data.",
    "confidence intervals for regression parameters.",
    "The motivation behind creating this crate came both",
    "from the recurring need of confidence intervals in personal projects",
    "and also out of frustration from having to look up the formulas each time.",
    "I reckoned that I might not be alone in this situation",
    "and that such a crate could prove useful to some.",
    "I am far from being a statistician and I will gladly welcome any advice or corrections.",
    "I only made a feeble attempt at numerical stability (e.g., kahan sum, log-sum-exp).",
    "In any case, please be circumspect about the results obtained from this crate.",
    "The crate has an optional feature serde which,",
    "if enabled, adds the crate serde as a dependency",
    "and provides serialization and deserialization for both Confidence and Interval.",
    "Unless you explicitly state otherwise,",
    "any contribution intentionally submitted for inclusion in the work by you,",
    "as defined in the Apache-2.0 license,",
    "shall be dual licensed as above, without any additional terms or conditions.",
    "mean::Arithmetic::ci computes the confidence interval for the arithmetic mean.",
    "mean::Geometric::ci computes the confidence interval for the geometric mean",
    "mean::Harmonic::ci computes the confidence interval for the harmonic mean",
    "Depending on the type of data and measurements, it is sometimes inappropriate",
    "to compute the mean of the data because that value makes little sense.",
    "For instance, consider a communication system and suppose that we want to test",
    " if at least 95% of messages are delivered within 1 second with 90% confidence.",
    "Then, the value of interest is the one-sided confidence interval of",
    "the 95th percentile (quantile=.95, confidence level=0.9).",
    "In a different context, if the data is an ordered sequence of strings,",
    "it could make sense to compute an interval around the median of the data,",
    "but the mean cannot be computed.",
];

fn bench_test<A, B>(title: &str)
where
    A: StatisticsOps<f64>,
    B: StatisticsOps<f64>,
{
    println!("Benchmarking {}...", title);
    let seeds = SEED_STRINGS
        .into_iter()
        .map(|s| Seeder::from(s).make_seed())
        .collect::<Vec<_>>();
    let confidence = Confidence::new_two_sided(0.95);

    let mut time_a = Vec::with_capacity(seeds.len());
    let mut time_b = Vec::with_capacity(seeds.len());
    print!("Progress: ");
    for seed in seeds {
        print!(".");
        let mut rng = ChaCha8Rng::from_seed(seed);
        let data_a = (0..SIZE).map(|_| rng.gen::<f64>()).collect::<Vec<_>>();
        let data_b = data_a.clone();

        let start = std::time::Instant::now();
        let stats = <A as StatisticsOps<f64>>::from_iter(data_a).unwrap();
        let _ = stats.ci_mean(confidence);
        let elapsed = start.elapsed();
        time_a.push(elapsed);

        let start = std::time::Instant::now();
        let stats = <B as StatisticsOps<f64>>::from_iter(data_b).unwrap();
        let _ = stats.ci_mean(confidence);
        let elapsed = start.elapsed();
        time_b.push(elapsed);
    }
    println!();

    let micros_a = time_a
        .iter()
        .map(|t| t.as_micros() as f64)
        .collect::<Vec<_>>();
    let micros_b = time_b
        .iter()
        .map(|t| t.as_micros() as f64)
        .collect::<Vec<_>>();

    let a_ci = mean::Arithmetic::ci(confidence, micros_a.clone()).unwrap();
    let b_ci = mean::Arithmetic::ci(confidence, micros_b.clone()).unwrap();
    let paired = comparison::Paired::ci(confidence, &micros_a, &micros_b).unwrap();
    println!("A   :   {} µs", a_ci);
    println!("B   :   {} µs", b_ci);
    println!("diff:   {} µs", paired);
    let relative_ci = b_ci.relative_to(&a_ci) * 100.;
    println!("relative: {} %", relative_ci);
    println!(
        "relative to A: {:.1} % ~ {:.1} %",
        relative_ci.low_f(),
        relative_ci.high_f()
    );
    println!();
}

fn main() {
    bench_test::<mean::Arithmetic<_>, mean::Geometric<f64>>("A: Arithmetic / B: Geometric");
    bench_test::<mean::Arithmetic<_>, mean::Harmonic<f64>>("A: Arithmetic / B: Harmonic");
    bench_test::<mean::Harmonic<_>, mean::Geometric<f64>>("A: Harmonic / B: Geometric");
}
