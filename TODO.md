# TODO List

This is an unordered list of items that I want to fix eventually.

## Breaking Changes

* Return `CIResult` more consistently. E.g., `quantile::ci` returns an `Option` whereas the other return a `CIResult`.
* Make `Interval` creation faillible and ensure that data is always consistent.
* Refactor `Interval` removing `Empty` and `Degenerate`, adding `LeftBounded` and `RightBounded` variants.
* Remove empty intervals and make bounds mandatory.
* Require `PartialOrder` type bound on `Interval` parametric type.

## Tests

* Move most test code outside of `src`.
* Add accuracy tests based on large samples.
* Improve test coverage, especially for edge cases.

## Functionality

* Confidence intervals for unpaired comparisons.
* Confidence intervals on regression parameters.
* Function to check for statistical significance.
* Function to estimate minimal sample size for target significance.
* Function to estimate best confidence level for a given data set and interval constraints.
* Export index calcutations for `quantile::ci`

## Issues

* one-sided intervals are not created properly.

## Pondering

* Integrate intervals with either crate [intervallum](https://crates.io/crates/intervallum) or crate [intervals-general](https://crates.io/crates/intervals-general)
* Use propotion intervals to compute quantile intervals
