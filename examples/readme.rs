///
/// This is the code used as examples in the README.md file.
/// It is used to ensure that the code snippets compile, are up-to-date, and work as intended.
///

fn block_1() -> stats_ci::CIResult<()> {
    use stats_ci::*;
    let data = [
        82., 94., 68., 6., 39., 80., 10., 97., 34., 66., 62., 7., 39., 68., 93., 64., 10., 74.,
        15., 34., 4., 48., 88., 94., 17., 99., 81., 37., 68., 66., 40., 23., 67., 72., 63., 71.,
        18., 51., 65., 87., 12., 44., 89., 67., 28., 86., 62., 22., 90., 18., 50., 25., 98., 24.,
        61., 62., 86., 100., 96., 27., 36., 82., 90., 55., 26., 38., 97., 73., 16., 49., 23., 26.,
        55., 26., 3., 23., 47., 27., 58., 27., 97., 32., 29., 56., 28., 23., 37., 72., 62., 77.,
        63., 100., 40., 84., 77., 39., 71., 61., 17., 77.,
    ];
    let confidence = Confidence::new_two_sided(0.95);
    let ci = mean::Arithmetic::ci(confidence, data)?;
    // mean: 53.67
    // stddev: 28.097613040716798
    println!("ci: {}", ci); // ci: [48.09482399055084, 59.24517600944916]
    println!("low: {}", ci.low_f()); // low: 48.09482399055084
    println!("high: {}", ci.high_f()); // high: 59.24517600944916

    let ci = mean::Harmonic::ci(confidence, data)?;
    // harmonic mean: 30.031313156339586
    println!("ci: {}", ci); // ci: [23.614092539460778, 41.23786064976718]

    let ci = mean::Geometric::ci(confidence, data)?;
    // geometric mean: 43.7268032829256
    println!("ci: {}", ci); // ci: [37.731050052007795, 50.675327686564806]

    let confidence = Confidence::new_upper(0.975);
    let ci = mean::Arithmetic::ci(confidence, data)?;
    println!("ci: {}", ci); // ci: [48.09482399055084,->)
    println!("low: {}", ci.low_f()); // low: 48.09482399055084
    println!("high: {}", ci.high_f()); // high: inf
    println!("low: {:?}", ci.low()); // high: Some(48.09482399055084)
    println!("high: {:?}", ci.high()); // high: None

    // incremental statistics also work
    let mut stats = mean::Arithmetic::new();
    stats.extend(data)?;
    let ci = stats.ci_mean(confidence)?;
    println!("incr ci: {}", ci); // incr ci: [48.09482399055084,->)
    for _ in 0..10 {
        stats.append(1_000.)?;
    }
    let ci = stats.ci_mean(confidence)?;
    println!("incr ci (97.5%): {}", ci); // incr ci (97.5%): [87.80710255546494,->)
    let ci = stats.ci_mean(Confidence::new_two_sided(0.8))?;
    println!("incr ci (80%): {}", ci); // incr ci (80%): [105.9411358250259, 173.45886417497408]

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
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O",
    ];
    let confidence = Confidence::new_two_sided(0.95);
    let ci = quantile::ci(confidence, &data, quantile)?;
    assert_eq!(ci, Interval::new("E", "L")?);

    Ok(())
}

fn block_3() -> stats_ci::CIResult<()> {
    use stats_ci::*;
    let confidence = Confidence::new_two_sided(0.95);

    let data = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
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
    block_1()?;
    block_2()?;
    block_3()?;
    Ok(())
}
