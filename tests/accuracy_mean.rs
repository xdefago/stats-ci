use num_traits::Float;
use statrs::distribution::*;
use stats_ci::*;

use rand_chacha::ChaCha8Rng;
use rand_seeder::Seeder;

mod common;

const SEED_STRING: &str =
    "Seed to the number generator so that the test is deterministically reproducible!";

#[test]
fn test_arithmetic_mean() {
    let tolerance = 0.01;
    let sample_size = 500;
    let repetitions = 500;
    let confidences = vec![
        // two-sided
        Confidence::new_two_sided(0.8),
        Confidence::new_two_sided(0.9),
        Confidence::new_two_sided(0.95),
        Confidence::new_two_sided(0.975),
        Confidence::new_two_sided(0.99),
        // upper one-sided
        Confidence::new_upper(0.8),
        Confidence::new_upper(0.9),
        Confidence::new_upper(0.95),
        Confidence::new_upper(0.975),
        Confidence::new_upper(0.99),
        // lower one-sided
        Confidence::new_lower(0.8),
        Confidence::new_lower(0.9),
        Confidence::new_lower(0.95),
        Confidence::new_lower(0.975),
        Confidence::new_lower(0.99),
    ];

    let distrib = Normal::new(0., 1.).unwrap();
    test_arithmetic(&distrib, sample_size, repetitions, &confidences, tolerance).unwrap();

    let distrib = Uniform::new(0., 1.).unwrap();
    test_arithmetic(&distrib, sample_size, repetitions, &confidences, tolerance).unwrap();

    let distrib = Exp::new(1.).unwrap();
    test_arithmetic(&distrib, sample_size, repetitions, &confidences, tolerance).unwrap();

    let distrib = LogNormal::new(0., 1.).unwrap();
    test_arithmetic(&distrib, sample_size, repetitions, &confidences, tolerance).unwrap();
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
    test_accuracy::<mean::Arithmetic<T>, _, _>(
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
        let color = common::highlight_color(hit_rate, confidence.level(), tolerance);
        println!(
            "{}  [{}, {:?}]",
            color.paint(format!(
                "hit rate: {:.1}% (Δ: {:.1}%)",
                hit_rate * 100.,
                (confidence.level() - hit_rate).abs() * 100.,
            )),
            std::any::type_name::<D>(),
            confidence
        );
        //assert!(hit_rate >= confidence.level() - 2. * tolerance);
    }
    println!();
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
