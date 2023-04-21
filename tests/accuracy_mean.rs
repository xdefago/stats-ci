use assert_approx_eq::assert_approx_eq;
use num_traits::Float;
use statrs::distribution::*;
use stats_ci::*;

use rand_chacha::ChaCha8Rng;
use rand_seeder::Seeder;

const SEED_STRING: &str =
    "Seed to the number generator so that the test is deterministically reproducible!";

#[test]
fn test_arithmetic_mean() {
    let sample_size = 100;
    let repetitions = 1000;
    let confidences = vec![
        Confidence::new_two_sided(0.8),
        Confidence::new_two_sided(0.9),
        Confidence::new_two_sided(0.95),
        Confidence::new_two_sided(0.99),
    ];

    let distrib = Normal::new(0., 1.).unwrap();
    test_arithmetic(&distrib, sample_size, repetitions, &confidences, 0.01).unwrap();

    let distrib = Uniform::new(0., 1.).unwrap();
    test_arithmetic(&distrib, sample_size, repetitions, &confidences, 0.015).unwrap();

    let distrib = Exp::new(1.).unwrap();
    test_arithmetic(&distrib, sample_size, repetitions, &confidences, 0.015).unwrap();

    let distrib = LogNormal::new(0., 1.).unwrap();
    test_arithmetic(&distrib, sample_size, repetitions, &confidences, 0.04).unwrap();
}

fn test_arithmetic<T: Float, D>(
    distrib: &D,
    sample_size: usize,
    repetitions: usize,
    confidences: &[Confidence],
    tolerance: f64,
) -> Result<(), Box<dyn std::error::Error>>
where
    D: rand::distributions::Distribution<T> + statrs::statistics::Distribution<T>,
{
    let mean = distrib.mean().ok_or("Can't compute mean")?;
    test_accuracy::<mean::Arithmetic, _, _>(
        distrib,
        mean,
        sample_size,
        repetitions,
        confidences,
        tolerance,
    )?;
    Ok(())
}

fn test_accuracy<C, D, T: Float>(
    distrib: &D,
    mean: T,
    sample_size: usize,
    repetitions: usize,
    confidences: &[Confidence],
    tolerance: f64,
) -> Result<(), Box<dyn std::error::Error>>
where
    C: mean::MeanCI<T>,
    D: rand::distributions::Distribution<T> + statrs::statistics::Distribution<T>,
{
    for &confidence in confidences {
        let hit_rate = hit_rate::<C, _, _>(distrib, mean, sample_size, repetitions, confidence)?;
        println!(
            "hit rate: {:.1}% (Δ: {:.1}%)  [{}, {:?}]",
            hit_rate * 100.,
            (confidence.level() - hit_rate).abs() * 100.,
            std::any::type_name::<D>(),
            confidence
        );
        assert_approx_eq!(hit_rate, confidence.level(), tolerance);
    }
    Ok(())
}

fn hit_rate<C, D, T: Float>(
    distribution: &D,
    mean: T,
    sample_size: usize,
    repetitions: usize,
    confidence: Confidence,
) -> Result<f64, Box<dyn std::error::Error>>
where
    C: mean::MeanCI<T>,
    D: rand::distributions::Distribution<T> + statrs::statistics::Distribution<T>,
{
    let mut rng: ChaCha8Rng = Seeder::from(SEED_STRING).make_rng();
    let mut success = 0;
    for _ in 0..repetitions {
        let sample: Vec<T> = (0..sample_size)
            .map(|_| distribution.sample(&mut rng))
            .collect();
        let ci = <C>::ci(confidence, sample)?;
        if ci.contains(&mean) {
            success += 1;
        }
    }
    Ok(success as f64 / repetitions as f64)
}
