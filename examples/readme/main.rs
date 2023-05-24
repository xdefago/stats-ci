///
/// This is the code used as examples in the README.md file.
/// It is used to ensure that the code snippets compile, are
/// up-to-date, and work as intended.
///

fn block_1() -> stats_ci::CIResult<()> {
    use stats_ci::*;
    let data = [
        82., 94., 68., 6., 39., 80., 10., 97., 34., 66., 62., 7., 39.,
        68., 93., 64., 10., 74., 15., 34., 4., 48., 88., 94., 17., 99.,
        81., 37., 68., 66., 40., 23., 67., 72., 63., 71., 18., 51.,
        65., 87., 12., 44., 89., 67., 28., 86., 62., 22., 90., 18.,
        50., 25., 98., 24., 61., 62., 86., 100., 96., 27., 36., 82.,
        90., 55., 26., 38., 97., 73., 16., 49., 23., 26., 55., 26., 3.,
        23., 47., 27., 58., 27., 97., 32., 29., 56., 28., 23., 37.,
        72., 62., 77., 63., 100., 40., 84., 77., 39., 71., 61., 17.,
        77.,
    ];
    // 1. create a statistics object
    let mut stats = mean::Arithmetic::new();
    // 2. add data
    stats.extend(data)?;

    // 3. define a confidence level
    let confidence = Confidence::new_two_sided(0.95);
    // 4. compute the confidence interval over the mean for some
    //    confidence level
    let ci = stats.ci_mean(confidence)?;
    // 5. get and print other statistics on the sample data
    println!("mean: {}", stats.sample_mean());
    //     mean: 53.67
    println!("std_dev: {}", stats.sample_std_dev());
    //     std_dev: 28.097613040716794
    println!(
        "ci ({} {}%): {}",
        confidence.kind(),
        confidence.percent(),
        ci
    ); // ci (two-sided 95%): [48.09482399055084, 59.24517600944916]
    println!("low: {}", ci.low_f()); // low: 48.09482399055084
    println!("high: {}", ci.high_f()); // high: 59.24517600944916

    // 6. compute other confidence intervals
    //    (almost no additional performance cost)
    println!(
        "upper one-sided 90% ci: {}",
        stats.ci_mean(Confidence::new_upper(0.9))?
    ); // upper one-sided 90% ci: [50.04495430416555,->)
    println!(
        "lower one-sided 80% ci: {}",
        stats.ci_mean(Confidence::new_lower(0.8))?
    ); // lower one-sided 80% ci: (<-,56.044998597990755]
    let ci = stats.ci_mean(Confidence::new_upper(0.975))?;
    println!("ci: {}", ci); // ci: [48.09482399055084,->)
    println!("low: {}", ci.low_f()); // low: 48.09482399055084
    println!("high: {}", ci.high_f()); // high: inf
    println!("low: {:?}", ci.low()); // high: Some(48.09482399055084)
    println!("high: {:?}", ci.high()); // high: None

    // get statistics for other means (harmonic)
    let stats = mean::Harmonic::from_iter(data)?;
    let ci = stats.ci_mean(confidence)?;
    println!("harmonic mean: {}", stats.sample_mean());
    //     harmonic mean: 30.03131315633959
    println!("ci: {}", ci);
    //     ci: [23.614092539460778, 41.23786064976718]

    // get statistics for other means (geometric)
    let stats = mean::Geometric::from_iter(data)?;
    let ci = stats.ci_mean(confidence)?;
    println!("geometric mean: {}", stats.sample_mean());
    //     geometric mean: 43.7268032829256
    println!("ci: {}", ci);
    //     ci: [37.731050052007795, 50.675327686564806]

    // incremental/intermediate statistics also work
    let mut stats = mean::Arithmetic::from_iter(data)?;
    let ci = stats.ci_mean(confidence)?;
    // a. confidence interval from the original data
    println!("incr ci: {}", ci);
    //     incr ci: [48.09482399055084, 59.24517600944916]

    // b. confidence interval after adding 10 additional data points
    for _ in 0..10 {
        stats.append(1_000.)?;
    }
    let ci = stats.ci_mean(confidence)?;
    println!("incr ci: {}", ci);
    //     incr ci: [87.80710255546494, 191.59289744453503]

    // parallel computation of the confidence interval
    use rayon::prelude::*;
    let state = data
        .clone()
        .par_iter()
        .map(|&x| mean::Arithmetic::from_iter([x]).unwrap())
        .reduce(|| mean::Arithmetic::new(), |s1, s2| s1 + s2);
    println!("parallel ci: {}", state.ci_mean(confidence)?);
    //     parallel ci: [48.09482399055084, 59.24517600944916]

    Ok(())
}

fn block_2() -> stats_ci::CIResult<()> {
    use stats_ci::*;

    let quantile = 0.5; // median

    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    let confidence = Confidence::new_two_sided(0.95);
    let ci = quantile::ci(confidence, &data, quantile)?;
    assert_eq!(ci, Interval::new(5, 12)?);

    let confidence = Confidence::new_two_sided(0.8);
    let ci = quantile::ci(confidence, &data, quantile)?;
    assert_eq!(ci, Interval::new(6, 11)?);

    let data = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L",
        "M", "N", "O",
    ];
    let confidence = Confidence::new_two_sided(0.95);
    let ci = quantile::ci(confidence, &data, quantile)?;
    println!("ci: {}", ci); // ci: [E, L]

    Ok(())
}

fn block_3() -> stats_ci::CIResult<()> {
    use stats_ci::*;
    let confidence = Confidence::new_two_sided(0.95);

    let data = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
        19, 20,
    ];
    let ci = proportion::ci_if(confidence, &data, |&x| x <= 10)?;
    println!("ci: {}", ci); // ci: [0.2992980081982124, 0.7007019918017876]
    assert!(ci.contains(&0.5));

    let population = 500;
    let successes = 421;
    let ci = proportion::ci(confidence, population, successes)?;
    println!("ci: {}", ci); // ci: [0.8074376489887337, 0.8713473021355645]
    assert!(ci.contains(&0.842));

    Ok(())
}

fn main() -> stats_ci::CIResult<()> {
    println!("block 1\n");
    block_1()?;

    println!("\nblock 2\n");
    block_2()?;

    println!("\nblock 3\n");
    block_3()?;
    Ok(())
}
