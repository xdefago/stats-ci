/// This example illustrates how to compute the confidence interval for the mean and the median of a sample.
///
use rand::{self, distributions::Distribution};
use statrs::statistics::Distribution as StatrsDistribution;
use statrs::statistics::Median;
use stats_ci::*;

fn main() {
    const SAMPLE_SIZE: usize = 200000;
    let sample_size = if SAMPLE_SIZE % 2 == 1 {
        SAMPLE_SIZE
    } else {
        SAMPLE_SIZE + 1
    }; // ensure that the sample size is odd

    // initialize the random number generator
    let mut rng = rand::thread_rng();

    // create the distribution
    let lambda = 0.1;
    let distrib = statrs::distribution::Exp::new(lambda).unwrap();

    // display the theoretical mean and median
    println!("Theoretical mean (1/lambda): {}", distrib.mean().unwrap());
    println!("Theoretical median (ln(2)/lambda): {}", distrib.median());
    println!();

    // generate data
    let mut sample = (0..sample_size)
        .map(|_| distrib.sample(&mut rng))
        .collect::<Vec<_>>();

    // show the data
    sample.sort_unstable_by(|x, y| x.partial_cmp(y).unwrap());
    if SAMPLE_SIZE < 100 {
        println!(
            "Sample: [{}]",
            sample
                .iter()
                .map(|x| format!("{:.2}", x))
                .collect::<Vec<_>>()
                .join(", ")
        );
    } else {
        println!(
            "Sample: [{} ... {}]",
            sample
                .iter()
                .take(20)
                .map(|x| format!("{:.2}", x))
                .collect::<Vec<_>>()
                .join(", "),
            sample[sample.len() - 1]
        );
    }

    // compute the sample stats
    let sample_median = sample[sample.len() / 2];
    let stats = mean::Arithmetic::from_iter(sample.clone()).unwrap();

    // show the sample stats
    println!("Sample mean: {}", stats.sample_mean());
    println!("Sample median: {}", sample_median);
    println!("Sample standard deviation: {}", stats.sample_std_dev());
    println!("Sample standard error: {}", stats.sample_sem());
    println!();

    // compute the confidence interval for the mean
    let confidence = Confidence::new_two_sided(0.99);

    println!(
        "{} {}% ci for mean: {}",
        confidence.kind(),
        confidence.percent(),
        stats.ci_mean(confidence).unwrap()
    );
    println!(
        "{} {}% ci for median: {}",
        confidence.kind(),
        confidence.percent(),
        quantile::ci_sorted_unchecked(confidence, &sample, 0.5).unwrap()
    );
    println!();

    let confidence = Confidence::new_two_sided(0.95);
    println!(
        "{} {}% ci for mean: {}",
        confidence.kind(),
        confidence.percent(),
        stats.ci_mean(confidence).unwrap()
    );
    println!(
        "{} {}% ci for median: {}",
        confidence.kind(),
        confidence.percent(),
        quantile::ci_sorted_unchecked(confidence, &sample, 0.5).unwrap()
    );
    println!();

    let confidence = Confidence::new_lower(0.95);
    println!(
        "{} {}% ci for mean: {}",
        confidence.kind(),
        confidence.percent(),
        stats.ci_mean(confidence).unwrap()
    );
    println!(
        "{} {}% ci for median: {}",
        confidence.kind(),
        confidence.percent(),
        quantile::ci_sorted_unchecked(confidence, &sample, 0.5).unwrap()
    );
    println!(
        "{} {}% ci for 90th percentile: {}",
        confidence.kind(),
        confidence.percent(),
        quantile::ci_sorted_unchecked(confidence, &sample, 0.9).unwrap()
    );
}
