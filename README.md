[![MIT license](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![Apache 2.0 license](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)
[![Docs](https://img.shields.io/docsrs/stats-ci)](https://docs.rs/stats-ci)
[![Tests](https://github.com/xdefago/stats-ci/actions/workflows/tests.yml/badge.svg)](https://github.com/xdefago/stats-ci/actions/workflows/tests.yml)
[![Downloads](https://img.shields.io/crates/d/stats-ci)](https://crates.io/crates/stats-ci)
[![Latest crates.io](https://img.shields.io/crates/v/stats-ci)](https://crates.io/crates/stats-ci)

# stats-ci


NB: As probably obvious from the `0.0.x` version number, this crate is not currently in a finished state and any commit can possibly introduce breaking changes. At this point, I am making no particular efforts to preserve backward compatibility. Therefore, please use at your own risks at least until version `0.1` or above. 

## Description

Stats-ci provides some basic functions to compute confidence intervals of sample data.
This includes the following:
* confidence intervals around the mean for numerical data,
* confidence intervals around a quantile (e.g., median) for arbitrary ordered data,
* confidence intervals for proportions.

Not included yet but planned are:
* confidence intervals for difference of sample data.
* confidence intervals for regression parameters.

## Motivation

The motivation behind creating this crate came both from the recurring need of confidence intervals in personal projects and also out of frustration from having to look up the formulas each time. I reckoned that I might not be alone in this situation and that such a crate could prove useful to some.

## Disclaimer

I am far from being a statistician and I will gladly welcome any advice or corrections.
I only made a feeble attempt at numerical statibility (e.g., kahan sum, log-sum-exp).
In any case, please be circumspect about the results obtained from this crate.

## Usage

Add the most recent release to your `Cargo.toml` _(check the latest version number on [crates.io](https://crates.io/crates/stats-ci))_:

```rust
[dependencies]
stats-ci = "0.0.5"
```

The crate has an optional feature `serde` which, if enabled, adds the crate [`serde`](https://crates.io/crates/serde) as a dependency and provides serialization and deserialization for both [`Confidence`](https://docs.rs/stats-ci/latest/stats_ci/enum.Confidence.html) and [`Interval`](https://docs.rs/stats-ci/latest/stats_ci/enum.Interval.html).

```rust
stats-ci = { version = "0.0.5", features = ["serde"] }
```


## Examples

### C.I. for the Mean

The crate provides functions to compute confidence intervals for the mean of floating-point (`f32` or `f64`) data.
The functions are generic and can be used with any type that implements the `Float` trait from the crate [`num-traits`](https://crates.io/crates/num-traits).
 
The crate provides three functions to compute confidence intervals for the mean of floating-point data:
* `mean::Arithmetic::ci` computes the confidence interval for the arithmetic mean.
* `mean::Geometric::ci` computes the confidence interval for the geometric mean
* `mean::Harmonic::ci` computes the confidence interval for the harmonic mean

```rust
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
    println!("ci: {}", ci); // ci: [41.64960001858482, 65.69039998141518]
    println!("low: {:?}", ci.low_f()); // low: 41.64960001858482
    println!("high: {:?}", ci.high_f()); // high: 65.69039998141518

    let ci = mean::Harmonic::ci(confidence, data)?;
    // harmonic mean: 30.031313156339586
    println!("ci: {}", ci); // ci: [22.470391261623906, 45.26085671573706]

    let ci = mean::Geometric::ci(confidence, data)?;
    // geometric mean: 43.7268032829256
    println!("ci: {}", ci); // ci: [20.367948541739317, 93.87461488452854]

    let confidence = Confidence::new_upper(0.975);
    let ci = mean::Arithmetic::ci(confidence, data)?;
    println!("ci: {}", ci); // ci: [41.64960001858482,->)
    println!("low: {:?}", ci.low_f()); // low: 41.64960001858482
    println!("high: {:?}", ci.high_f()); // high: inf
    println!("low: {:?}", ci.low()); // high: Some(41.64960001858482)
    println!("high: {:?}", ci.high()); // high: None
```

## C.I. for Quantiles

Depending on the type of data and measurements, it is sometimes inappropriate to compute the mean of the data because that value makes little sense.
For instance, consider a communication system and suppose that we want to test if at least 95% of messages are delivered within 1 second with 90% confidence.
Then, the value of interest is the one-sided confidence interval of the 95th percentile (quantile=.95, condidence level=0.9). 

In a different context, if the data is an ordered sequence of strings, it could make sense to compute an interval around the median of the data, but the mean cannot be computed.

```rust
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
```

## C.I. for Proportions

Confidence intervals for proportions are often used in the context of A/B testing or when measuring the success/failure rate of a system.
It is also useful when running Monte-Carlo simulations to estimate the winning chances of a player in a game.
 
This crate uses the Wilson score interval to compute the confidence interval for a proportion,
which is more stable than the standard normal approximation but results in slightly more conservative intervals.

```rust
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
```

## Contributing

I will gladly and carefully consider any constructive comments that you have to offer.
In particular, I will be considering constructive feedback both on the interface and the calculations
with the following priorities correctness, code readability, genericity, efficiency.

Currently, the following are on my TODO list:

* [feature] confidence intervals for difference of sample data.
* [feature] confidence intervals for regression parameters.
* [stats] review/fix statistical tests
* [API] return results more consistently
* [API] remove `unwrap()` and reduce panicking code

## References

* Raj Jain. [The Art of Computer Systems Performance Analysis: Techniques for Experimental Design, Measurement, Simulation, and Modeling,](https://www.cse.wustl.edu/~jain/books/perfbook.htm) John Wiley & Sons, 1991.
* [Wikipedia - Confidence interval](https://en.wikipedia.org/wiki/Confidence_interval)
* [Wikipedia - Binomial proportion confidence interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval)
* [Wikipedia article on normal approximation interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Normal_approximation_interval)
* Francis J. DiTraglia. [Blog post: The Normal Approximation Confidence Interval for a Proportion](https://www.econometrics.blog/post/the-normal-approximation-confidence-interval-for-a-proportion/)


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
