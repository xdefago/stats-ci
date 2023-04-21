use assert_approx_eq::assert_approx_eq;
use stats_ci::*;


use rand_chacha::ChaCha8Rng;
use rand_seeder::Seeder;

const SEED_STRING: &str =
    "Seed to the number generator so that the test is deterministically reproducible!";

#[test]
fn test_accuracy_proportion() {
    let tolerance = 0.03;
    let sample_size = 500;
    let repetitions = 1000;
    let confidences = vec![
        Confidence::new_two_sided(0.8),
        Confidence::new_two_sided(0.9),
        Confidence::new_two_sided(0.95),
        Confidence::new_two_sided(0.99),
    ];
    let targets = vec![0.15, 0.2, 0.4, 0.5, 0.9];

    for target in targets {
        let distrib = rand::distributions::Bernoulli::new(target).unwrap();

        for confidence in &confidences {
            let hit_rate =
                hit_rate(&distrib, target, sample_size, repetitions, *confidence).unwrap();
            println!(
                "hit rate: {:.1}% (Δ: {:.1}%)  [trial target: {}, confidence: {:?}]",
                hit_rate * 100.,
                (confidence.level() - hit_rate).abs() * 100.,
                target, confidence
            );
            assert_approx_eq!(hit_rate, confidence.level(), tolerance);
        }
    }
}

fn hit_rate<D>(
    distrib: &D,
    target: f64,
    sample_size: usize,
    repetitions: usize,
    confidence: Confidence,
) -> Result<f64, Box<dyn std::error::Error>>
where
    D: rand::distributions::Distribution<bool>,
{
    let mut rng: ChaCha8Rng = Seeder::from(SEED_STRING).make_rng();
    let mut hits = 0;
    for _ in 0..repetitions {
        let sample = (0..sample_size)
            .map(|_| distrib.sample(&mut rng))
            .collect::<Vec<_>>();
        let ci = proportion::ci_true(confidence, sample)?;

        if ci.contains(&target) {
            hits += 1;
        }
    }
    Ok(hits as f64 / repetitions as f64)
}
