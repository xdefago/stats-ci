/// This example illustrates how to compute confidence intervals for proportions.
///
use rand::{self, distributions::Distribution};
use stats_ci::*;

fn main() {
    const SAMPLE_SIZE: usize = 100;
    let sample_size = if SAMPLE_SIZE % 2 == 1 {
        SAMPLE_SIZE
    } else {
        SAMPLE_SIZE + 1
    }; // ensure that the sample size is odd

    // initialize the random number generator
    let mut rng = rand::thread_rng();

    // create the distribution
    let width = 10;
    let distrib = statrs::distribution::DiscreteUniform::new(1, width).unwrap();

    // display the theoretical mean and median
    println!("Theoretical proportion (1/{width}): {}", 1. / width as f64);
    println!();

    // generate data
    let sample = (0..sample_size)
        .map(|_| distrib.sample(&mut rng) as i64)
        .collect::<Vec<_>>();

    // show the data
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

    let count_one = sample.iter().filter(|&&x| x == 1).count();
    println!(
        "Sample proportion ({}/{}): {}",
        count_one,
        sample.len(),
        count_one as f64 / sample.len() as f64
    );

    // show the sample ci (95%)
    let confidence = Confidence::new_two_sided(0.95);
    let ci = proportion::ci_if(confidence, &sample, |&x| x == 1).unwrap();
    println!(
        "Sample proportion ci ({} {}%): {}",
        confidence.kind(),
        confidence.percent(),
        ci
    );

    // show the sample ci (99%)
    let confidence = Confidence::new_lower(0.99);
    let ci = proportion::ci_if(confidence, &sample, |&x| x == 1).unwrap();
    println!(
        "Sample proportion ci ({} {}%): {}",
        confidence.kind(),
        confidence.percent(),
        ci
    );

    // show confidence that a number is even
    let confidence = Confidence::new_two_sided(0.95);
    let ci = proportion::ci_if(confidence, &sample, |&x| x % 2 == 0).unwrap();
    println!(
        "\nProportion of even numbers ({} {}%): {}",
        confidence.kind(),
        confidence.percent(),
        ci
    );
}
