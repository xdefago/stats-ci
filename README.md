# stats-ci

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)

NB: As probably obvious from the `0.0.x` version number, this crate is currently in a __very unstable__ state and any commit can possibly introduce breaking changes. At this stage, I am making no efforts of backward compatibility. Therefore, please use at your own risks at least until a version `0.x`. 

## Description

Stats-ci provides some basic functions to compute confidence intervals of sample data.
This includes confidence intervals around the mean for numerical data, confidence intervals around a quantile (e.g., median) for arbitrary ordered data, and confidence intervals for proprotions.

## Motivation

I created this crate from the need in personal projects and also out of frustration from having to look up the formulas for computing various confidence intervals.
I am far from being a statistician and I will gladly welcome any advice or correct mistakes.

## Usage

Add the most recent release to your `Cargo.toml`

```Rust
[dependencies]
stats-ci = "0.0.1"
```

## Examples

### C.I. for the Mean

```rust
use stats_ci::*;
let data = [
    82., 94., 68., 6., 39., 80., 10., 97., 34., 66., 62., 7., 39., 68., 93., 64., 10., 74.,
    15., 34., 4., 48., 88., 94., 17., 99., 81., 37., 68., 66., 40., 23., 67., 72., 63.,
    71., 18., 51., 65., 87., 12., 44., 89., 67., 28., 86., 62., 22., 90., 18., 50., 25.,
    98., 24., 61., 62., 86., 100., 96., 27., 36., 82., 90., 55., 26., 38., 97., 73., 16.,
    49., 23., 26., 55., 26., 3., 23., 47., 27., 58., 27., 97., 32., 29., 56., 28., 23.,
    37., 72., 62., 77., 63., 100., 40., 84., 77., 39., 71., 61., 17., 77.,
];
let confidence = Confidence::new_two_sided(0.95);
let ci = mean::Arithmetic::ci(confidence, data)?;
// mean: 53.67
// stddev: 28.097613040716798
println!("ci: {}", ci); // ci: [48.09482399055084, 59.24517600944916]
println!("low: {:?}", ci.low()); // low: Some(48.09482399055084)
println!("high: {:?}", ci.high()); // high: Some(59.24517600944916)

let ci = Harmonic::ci(confidence, data)?;
// harmonic mean: 30.031313156339586
println!("ci: {}", ci); // ci: [23.614092539460778, 41.23786064976718]

let ci = Geometric::ci(confidence, data)?;
// geometric mean: 43.7268032829256
println!("ci: {}", ci); // ci: [37.731050052007845, 50.67532768656474]
```

## C.I. for Quantiles

```rust
use stats_ci::*;

let quantile = 0.5; // median

let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

let confidence = Confidence::new_two_sided(0.95);
let ci = quantile::ci(confidence, &data, quantile)?;
assert_eq!(ci, Interval::new(4, 12));

let confidence = Confidence::new_two_sided(0.8);
let ci = quantile::ci(confidence, &data, quantile)?;
assert_eq!(ci, Interval::new(6, 10));

let data = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O"];
let confidence = Confidence::new_two_sided(0.95);
let ci = quantile::ci(confidence, &data, quantile)?;
assert_eq!(interval3, Interval::new("D", "L"));
```

## C.I. for Proportions

```rust
use stats_ci::*;
let confidence = Confidence::new_two_sided(0.95);

let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
let ci = proportion::ci_if(confidence, &data, |&x| x <= 10)?;
println!("ci: {}", ci); // ci: [0.2992980081982124, 0.7007019918017876]
assert!(ci.contains(0.5));

let population = 500;
let successes = 421;
let ci = proportion::ci(confidence, population, successes)?;
println!("ci: {}", ci); // ci: [0.8074376489887337, 0.8713473021355645]
assert!(ci.contains(0.842));
```

## Contributing

I will gladly and carefully consider any constructive comments that you have to offer.
In particular, I will be considering constructive feedback both on the interface and the calculations
with the following priorities correctness, code readability, genericity, efficiency.


## References


