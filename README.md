[![MIT license](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![Apache 2.0 license](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)
[![Docs](https://img.shields.io/docsrs/stats-ci)](https://docs.rs/stats-ci)
[![Tests](https://github.com/xdefago/stats-ci/actions/workflows/tests.yml/badge.svg)](https://github.com/xdefago/stats-ci/actions/workflows/tests.yml)
[![Downloads](https://img.shields.io/crates/d/stats-ci)](https://crates.io/crates/stats-ci)
[![Latest crates.io](https://img.shields.io/crates/v/stats-ci)](https://crates.io/crates/stats-ci)

A library to help compute confidence intervals in various situations. 

This crate provides convenient means to help compute confidence intervals in multiple cases.
It is meant to be used when analyzing experimental data and measurements.
Experimental data is subject to experimental error which must be taken into account during analysis.
Among several statistical methods, confidence intervals provide information that it easy to interpret.

The motivation comes from a personal need and was that no
crate seem to provide an easy and comprehensive solution to computing such intervals.
One exception is the crate [`criterion`](https://crates.io/crates/criterion) which computes
confidence intervals for its measurements but does not export such functionality.

This crate provides the means to easily and efficiently compute confidence intervals of sample data
in situations as follows:
* [`mean`] confidence intervals around the mean (arithmetic, harmonic, geometric) for numerical data,
* [`quantile`] confidence intervals around a quantile (e.g., median) for arbitrary ordered data,
* [`proportion`] confidence intervals for proportions.
* [`comparison`] confidence intervals for comparisons (paired or unpaired observations).

This crate does not (yet) support the following:
* confidence intervals for regression parameters.
* confidence intervals for other statistics (e.g., variance, etc.)
* Chi square test

This crate's documentation provides several simple examples of how to use each feature.

# Usage

This crate is on [crates.io](https://crates.io/crates/stats-ci) and can be used by simply adding `stats-ci` to the dependencies in your project's `Cargo.toml` _(check the latest version number on [crates.io](https://crates.io/crates/stats-ci) and replace `{ latest version }` below)_:

```toml
[dependencies]
stats-ci = "{ latest version }"
```

The crate is still somewhat unstable and breaking changes can possibly occur from a minor version to the next.


# Confidence intervals (overview)

Statistics are based on the idea that experimental data is a random sampling over some theoretical
(yet unknown) probability distribution. Hypothetically, if the sampled data could consist of an 
infinite number of measurements, the data would exactly describe that theoretical distribution.
In realistic situations, however, only a very limited number of measurements are available.
This raises the question as how to estimate the parameters of the theoretical distribution (e.g., the mean) based on that limited number of samples, and with what accuracy.

In this context, confidence intervals provide a statistical answer to both questions.
Roughly, the idea is that one chooses a desired __level of confidence__ (or how unlikely are the conclusions to be bogus) and the estimated parameter is no longer represented as a single value, but as an interval. The width of that interval depends on the confidence level, the variability of the data, and the number of samples. The way to interpret it is that the theoretical value of the parameter (e.g., mean, median) can be any value from that interval. A level of confidence of 95% means that, if repeating the entire experiment under the exact same circumstances, about 95% of the confidence intervals obtained in each experiment will include the theoretical value of the parameter.

Consider a simple experiment yielding the following random data sampled from some unknown distribution:
```rust
let data = [
    10.6, 6.6, 26.7, 0.4, 5.7, 0.3, 1.1, 5.0, 8.4, 1.4, 15.1, 0.3,
    20.4, 1.2, 28.4, 10.7, 0.4, 10.1, 4.5, 7.1, 4.3, 37.4, 0.9, 10.1,
    12.6, 21.7, 21.9, 2.0, 8.4, 9.3
];
```

* __What is the _mean_ of the original distribution?__ <br>
    The sample mean is 9.76667 but how close is it from the theoretical mean?
    Computing the 95% confidence interval on the mean for this data yields \[6.18467, 13.34866\], which means that the theoretical mean can be any number in this interval (with 95% confidence).
    The resulting interval is wide and hence the estimation is not very precise.
    This is good evidence that the experimental error is quite large and drawing conclusions and extrapolations based on an exact value of 9.76667 for the mean is very dangerous.
    Keeping the same confidence, the only way to reduce this interval is by increasing the sample size, i.e., running additional experiments.

* __What is the _median_ of the original distribution?__ <br>
    The 95% confidence interval on the median yields \[4.3, 10.6\].
    In this case, both bounds of the interval are observed values.

* __Theoretical distribution__ <br>
    In this example, the data was actually taken from an _exponential_ distribution with parameter λ = 0.1 (mean = 1/λ = 10 and median = ln(2)/λ = 6.93147…). In this special case, we can verify that both theoretical mean and median are indeed contained in their respective confidence interval.

# Examples

This crate makes it easy to compute confidence intervals based on sample data for various situations, including mean, quantiles, proportions, and comparison.

This crate is build around a type [`Confidence`] to express a confidence level and
a type [`Interval`] to represent a confidence interval.
Intervals are generic and can be instantiated for various types, beyond the usual
float or integer types.


## Mean and median
Given the example discussed above, the intervals can be computed on the _mean_ or on _quantiles_ (e.g., median) as described below:
```rust
// 1. import the crate
use stats_ci::*;
// 2. collect the data
let data = [
    10.6, 6.6, 26.7, 0.4, 5.7, 0.3, 1.1, 5.0, 8.4, 1.4, 15.1, 0.3,
    20.4, 1.2, 28.4, 10.7, 0.4, 10.1, 4.5, 7.1, 4.3, 37.4, 0.9,
    10.1, 12.6, 21.7, 21.9, 2.0, 8.4, 9.3
];
// 3. define the confidence level (for 95% confidence)
let confidence = Confidence::new(0.95);

// 4a. compute the interval for the arithmetic mean
if let Ok(ci) = mean::Arithmetic::ci(confidence, data) {
    // display the interval
    println!("{}% c.i. for the mean = {}", confidence.percent(), ci);
    if ! ci.contains(&10.) {
        println!("Does NOT contains the theoretical mean!");
    }
}
// 4b. compute the interval for the median (i.e., 0.5-quantile)
if let Ok(ci) = quantile::ci(confidence, data, 0.5) {
    // display the interval
    println!("{}% c.i. for the median = {}", confidence.percent(), ci);
    if ! ci.contains(&6.93147) {
        println!("Does NOT contains the theoretical median!");
    }
}
```

Similarly, the confidence interval on the _geometric mean_ and the _harmonic mean_ can be computed as follows.
```rust
# use stats_ci::*;
let data = [ 10.6, 6.6, /* ... */ ];
let confidence = Confidence::new(0.95);
let ci = mean::Geometric::ci(confidence, data);
let ci = mean::Harmonic::ci(confidence, data);
```

## Proportions
Confidence intervals can also be computed on proportions. This happens for instance when trying to estimate the loss rate of a given communication channel (i.e., the proportion of message lost against the number of message sent). This can be computed in a straightforward way if one knows the number of losses (qualified as "`success`" in the crate) and the total number of messages (qualified as "`population`"):
```rust
use stats_ci::*;
let confidence = Confidence::new(0.95);
let population = 10_000; // total number of sent messages
let successes = 89;      // number of lost messages
let ci = proportion::ci(confidence, population, successes).unwrap();
println!("Loss rate: {}", ci);
// > Loss rate: [0.007238518896069928, 0.010938644303608623]
//
// which means that the loss rate is estimated (95% confidence) to
// be between 0.7238% and 1.0939%.

// One-sided confidence
let confidence = Confidence::new_lower(0.95);
let ci = proportion::ci(confidence, population, successes).unwrap();
println!("Loss rate less than: {}", ci);
// > Loss rate less than: [0, 0.010583156571857643]
//
// which means that the loss rate is likely (95% confidence) to be
// less than 1.05832%.
```

Some convenience functions make it easier to deal with an iterator or array
of data and a "success" condition:
```rust
use stats_ci::*;
let data = [
    8, 11, 4, 18, 17, 9, 20, 3, 10, 14, 12, 7, 13, 16, 1, 6, 5, 2,
    15, 19
];
let confidence = Confidence::new(0.95);
let ci = proportion::ci_if(confidence, data, |x| x <= 10).unwrap();
println!("ci: {}", ci);
// > ci: [0.2992980081982124, 0.7007019918017876]
//
// yields the estimated proportion of numbers that are less or
// equal to 10, based on data obtained from random sampling.
```

## Comparison
A frequent use of confidence intervals is to compare groups of data. This happens
for instance when comparing two systems, say system A and system B, such as to
determine if the new system B introduces an improvement over the state-of-the-art
system A. In this case, there are two situations:

### Paired observations
Paired observations occur only when each measurement on system B can be paired
with a corresponding measurement on system A. This happens when experiments are
carefully designed such that both systems are measured under the exact same
conditions and for the exact same input.
```rust
use stats_ci::*;
// Zinc concentration in water samples from a river
// (from <https://online.stat.psu.edu/stat500/lesson/7/7.3/7.3.2>)
let data_bottom_water = [
    0.430, 0.266, 0.567, 0.531, 0.707, 0.716, 0.651, 0.589, 0.469,
    0.723,
];
let data_surface_water = [
    0.415, 0.238, 0.390, 0.410, 0.605, 0.609, 0.632, 0.523, 0.411,
    0.612,
];
let confidence = Confidence::new(0.95);
let ci = comparison::Paired::ci(
    confidence,
    &data_bottom_water,
    &data_surface_water
).unwrap();
```
The confidence interval is on the difference of the two means and can be interpreted as follows:

* if __both bounds__ are strictly positive, then system A has a higher mean than system B. Also, the increase in the mean is at least the lower bound and at most the upper bound of the interval.
* if __both bounds__ are strictly negative, then system A has a lower mean than system B.
* otherwise, if zero is included in the interval, then the means of system A and system B are not significantly different. Regardless of the values, claiming an improvement (or degradation) in that case is a bogus claim.


### Unpaired observations
Unpaired observations occur in all other cases where observations cannot be paired one-by-one. For instance, in the example above, two independent groups of users are made to use either system A or
system B and the collected data is used for comparison. In this case, the measurements cannot be
matched one-to-one and the number of measurements could even be different for one system versus the other.
Computing the interval on the difference of the means is more involved in this situation and also requires more measurements to achieve equivalent precision.

With this crate, unpaired observation is very similar to paired observations: 
```rust
use stats_ci::*;
// Gain in weight of 19 female rats between 28 and 84 days after birth.
// 12 were fed on a high protein diet and 7 on a low protein diet.
// (from <https://www.statsdirect.co.uk/help/parametric_methods/utt.htm>)
let data_high_protein = [
    134., 146., 104., 119., 124., 161., 107., 83., 113., 129., 97., 123.,
];
let data_low_protein = [70., 118., 101., 85., 107., 132., 94.];
let confidence = Confidence::new(0.95);
let ci = comparison::Unpaired::ci(
    confidence,
    &data_high_protein,
    &data_low_protein
).unwrap();
```
The interpretation of the confidence interval is the same as with paired observations above:

* if __both bounds__ are strictly positive, then system A has a __higher__ mean than system B.
* if __both bounds__ are strictly negative, then system A has a __lower__ mean than system B.
* if zero is included in the interval, then the means of system A and system B are not significantly different.

### Advanced: Incremental statistics
While the easier interface described until now is sufficient for most use cases, the crate offers a slightly more flexible interface making it possible to extend the number of samples after obtaining intervals.

The example below shows how to do this when computing the arithmetic mean, but that approach is also available for the other types of intervals; check the [API documentation](https://docs.rs/stats-ci) for each module for further information and additional examples. Points 1. to 3. are identical to the original example:
```rust
// 1. import the crate
use stats_ci::*;
// 2. collect the data
let data = [
    10.6, 6.6, 26.7, 0.4, 5.7, 0.3, 1.1, 5.0, 8.4, 1.4, 15.1, 0.3,
    20.4, 1.2, 28.4, 10.7, 0.4, 10.1, 4.5, 7.1, 4.3, 37.4, 0.9,
    10.1, 12.6, 21.7, 21.9, 2.0, 8.4, 9.3
];
// 3. define the confidence level (for 95% confidence)
let confidence = Confidence::new(0.95);

// 4. create a statistics object
let mut stats = mean::Arithmetic::new();
// 5. add data
stats.extend(data).unwrap();
// shortcut: combines 4. and 5.
let mut stats = mean::Arithmetic::from_iter(data).unwrap();

// 6. compute the confidence interval over the mean for some
//    confidence level
let ci = stats.ci_mean(confidence).unwrap();

// 7. add more data
stats.extend([ 10.7, 9.8, /* … */ ]).unwrap();
// 8. compute the new confidence interval
let ci = stats.ci_mean(confidence).unwrap();
// 9. or maybe with a different confidence level
let ci = stats.ci_mean(Confidence::new(0.99)).unwrap();
//10. and get other statistics
let mean = stats.sample_mean();
let variance = stats.sample_variance();
let std_dev = stats.sample_std_dev();
let std_err = stats.sample_sem();
```
Note that only the points 5. and 7. are potentially costly operations when the data is very large.

This interface is useful, for instance, in the following situations:

* when you have a stream of data and don't want to keep all values.
* when you want to continue collecting data until you have sufficient statistical significance (e.g., interval shorter than some width relative to the mean).
* when you want to continue collecting data and improve accuracy until some deadline.
* when you want to compute the confidence intervals of several confidence levels or of different types in a single pass through the data.
* when you need to improve performance through concurrent execution (see next section).

### Advanced: Parallel execution
If performance is an issue, the crate allows for easy parallel execution with the crate [`rayon`](https://crates.io/crates/rayon). 

```rust
// 1. import the crates
use stats_ci::*;
use rayon::prelude::*;
// 2. (as before:) collect the data
let data = [
    10.6, 6.6, 26.7, 0.4, 5.7, 0.3, 1.1, 5.0, 8.4, 1.4, 15.1, 0.3,
    20.4, 1.2, 28.4, 10.7, 0.4, 10.1, 4.5, 7.1, 4.3, 37.4, 0.9,
    10.1, 12.6, 21.7, 21.9, 2.0, 8.4, 9.3, /* … many more … */
];
// 3. (as before:) define the confidence level
let confidence = Confidence::new(0.95);
// 4. create and compute the statistics object
let stats = data
    .clone()
    .par_iter()
    .map(|&x| mean::Arithmetic::from_iter([x]).unwrap())
    .reduce(|| mean::Arithmetic::new(), |s1, s2| s1 + s2);
// 5. (as before:) compute the confidence interval
let ci = stats.ci_mean(confidence).unwrap();
```
Note that it makes little sense to parallelize when dealing with only a few thousand samples.

### Additional examples
You can find further information and additional examples from this crate's [API documentation](https://docs.rs/stats-ci).


# Statistics / computations

* Intervals on the mean use the Student t-distribution up to about 100_000 values, above which the computation switches to the normal distribution.
* Intervals on proportions and quantiles rely on the Wilson score method which is known to be more statistically stable than the Wald score method that is typically presented in textbooks.
* The crate uses compensated summation (Kahan summation) to avoid accumulating roundup errors during summation of very large data.


# Crate features

The crate has two features:

* `approx` _(default)_ enables approximate comparison between intervals. Adds the dependency to the crate [`approx`](https://crates.io/crates/approx).
* `serde` feature adds the crate [`serde`](https://crates.io/crates/serde) as a dependency and provides serialization and deserialization for both [`Confidence`](https://docs.rs/stats-ci/latest/stats_ci/enum.Confidence.html) and [`Interval`](https://docs.rs/stats-ci/latest/stats_ci/enum.Interval.html), as well as the incremental states for intervals on the mean.
```toml
stats-ci = { version = "{ latest version }", features = ["serde"] }
```

# References

* Raj Jain. [The Art of Computer Systems Performance Analysis: Techniques for Experimental Design, Measurement, Simulation, and Modeling,](https://www.cse.wustl.edu/~jain/books/perfbook.htm) John Wiley & Sons, 1991.
* [Wikipedia - Confidence interval](https://en.wikipedia.org/wiki/Confidence_interval)
* [Wikipedia - Binomial proportion confidence interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval)
* [Wikipedia article on normal approximation interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Normal_approximation_interval)
* Dransfield R.D., Brightwell R. (2012) Avoiding and Detecting Statistical Malpractice (or "How to Get On Top of Statistics): Design & Analysis for Biologists, with R. InfluentialPoints, UK [online](https://influentialpoints.com/hyperbook.htm)
* _idem_. Chapter [Confidence intervals of proportions and rates](https://influentialpoints.com/Training/confidence_intervals_of_proportions-principles-properties-assumptions.htm)
* Francis J. DiTraglia. [Blog post: The Wilson Confidence Interval for a Proportion](https://www.econometrics.blog/post/the-wilson-confidence-interval-for-a-proportion/). Feb 2022.
* Nilan Noris. "The standard errors of the geometric and harmonic means and their application to index numbers." Ann. Math. Statist. 11(4): 445-448 (December, 1940). DOI: [10.1214/aoms/1177731830](https://doi.org/10.1214/aoms/1177731830) [JSTOR](https://www.jstor.org/stable/2235727)
* PennState. Stat 500. [Online](https://online.stat.psu.edu/stat500/)


# License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

# Contributing

I will gladly and carefully consider any constructive comments that you have to offer.
In particular, I will be considering constructive feedback both on the interface and the calculations
with the following priorities correctness, code readability, genericity, efficiency.

Currently, the following are on my TODO list:

* \[feature\] confidence intervals for regression parameters.
* \[stats\] review/fix statistical tests
* \[API\] reduce panicking code
* \[Refactoring\] restructure error results

