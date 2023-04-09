//! compute confidence intervals for quantiles
//! 
//! # Examples
//! 
//! ```
//! use stats_ci::quantile;
//! 
//! let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
//! let confidence = 0.95;
//! let quantile = 0.5; // median
//! let interval = quantile::ci(confidence, &data, quantile).unwrap();
//! assert_eq!(interval, (4, 12));
//! 
//! let confidence = 0.8;
//! let interval2 = quantile::ci(confidence, &data, quantile).unwrap();
//! assert_eq!(interval2, (6, 10));
//! 
//! let confidence = 0.5;
//! let quantile = 0.2; // 20th percentile
//! let interval3 = quantile::ci(confidence, &data, quantile).unwrap();
//! assert_eq!(interval3, (2, 5));
//! ```
//! 
use super::*;

/// compute the confidence interval for a given quantile, assuming that the data as already.
/// 
/// 
/// # Arguments
/// 
/// * `confidence` - the confidence level (must be in (0, 1))
/// * `sorted` - the sorted sample
/// * `quantile` - the quantile to compute the confidence interval for (must be in (0, 1))
/// 
/// # Errors
/// 
/// * `TooFewSamples` - if the number of samples is too small to compute a confidence interval
/// * `InvalidConfidenceLevel` - if the confidence level is not in (0, 1)
/// * `InvalidQuantile` - if the quantile is not in (0, 1)
/// 
/// # Examples
/// 
/// ```
/// # use stats_ci::quantile;
/// use assert_approx_eq::assert_approx_eq;
/// 
/// let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
/// let confidence = 0.95;
/// let quantile = 0.5; // median
/// let interval = quantile::ci_sorted_unchecked(confidence, &data, quantile).unwrap();
/// assert_eq!(interval, (4, 12));
/// 
/// let confidence = 0.8;
/// let interval2 = quantile::ci_sorted_unchecked(confidence, &data, quantile).unwrap();
/// assert_eq!(interval2, (6, 10));
/// 
/// let confidence = 0.5;
/// let quantile = 0.2; // 20th percentile
/// let interval3 = quantile::ci_sorted_unchecked(confidence, &data, quantile).unwrap();
/// assert_eq!(interval3, (2, 5)); 
/// ```
/// 
pub fn ci_sorted_unchecked<T: Clone>(confidence: f64, sorted: &[T], quantile: f64) -> Option<(T, T)> {
    assert!(quantile > 0. && quantile < 1.);
    assert!(confidence > 0. && confidence < 1.);

    let len = sorted.len();
    if len < 3 {
        // too few smaples to compute
        return None;
    }

    let z = z_value(confidence);
    let q = quantile; /* 0.5 for median */
    let n = len as f64;
    let mid_span = z * f64::sqrt(n * q * (1. - q));
    let lo = 1.max(f64::ceil(n * q - mid_span) as usize) - 1;
    let hi = (len - 1).min(f64::ceil(n * q + mid_span) as usize - 1);
    Some((sorted[lo].clone(), sorted[hi].clone()))
}

/// compute the confidence interval for a given quantile
/// 
/// # Arguments
/// 
/// * `confidence` - the confidence level (must be in (0, 1))
/// * `data` - the sample
/// * `quantile` - the quantile to compute the confidence interval for (must be in (0, 1))
/// 
/// # Errors
/// 
/// * `TooFewSamples` - if the number of samples is too small to compute a confidence interval
/// * `InvalidConfidenceLevel` - if the confidence level is not in (0, 1)
/// * `InvalidQuantile` - if the quantile is not in (0, 1)
/// 
/// # Examples
/// 
/// ```
/// # use stats_ci::quantile;
/// use assert_approx_eq::assert_approx_eq;
/// 
/// let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
/// let confidence = 0.95;
/// let quantile = 0.5; // median
/// let interval = quantile::ci(confidence, &data, quantile).unwrap();
/// assert_eq!(interval, (4, 12));
/// 
/// let data2 = [2, 14, 13, 6, 8, 4, 15, 9, 3, 11, 10, 7, 1, 12, 5];
/// let interval2 = quantile::ci(confidence, &data, quantile).unwrap();
/// assert_eq!(interval, interval2);
/// 
/// let confidence = 0.8;
/// let interval3 = quantile::ci(confidence, &data, quantile).unwrap();
/// assert_eq!(interval3, (6, 10));
/// 
/// let confidence = 0.5;
/// let quantile = 0.2; // 20th percentile
/// let interval4 = quantile::ci(confidence, &data, quantile).unwrap();
/// assert_eq!(interval4, (2, 5));
/// ```
pub fn ci<T: PartialOrd + Clone>(confidence: f64, data: &[T], quantile: f64) -> Option<Interval<T>> {
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    ci_sorted_unchecked(confidence, &sorted, quantile)
        .map(|(lo, hi)| Interval::new(lo, hi))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_median_ci() {
        let data = [
            8., 11., 12., 13., 15., 17., 19., 20., 21., 21., 22., 23., 25., 26., 28.,
        ];
        let median_ci = ci_sorted_unchecked(0.95, &data, 0.5);
        assert_eq!(median_ci, Some((13., 23.)));
    }

    #[test]
    fn test_quantile_ci() {
        let data = [
            8., 11., 12., 13., 15., 17., 19., 20., 21., 21., 22., 23., 25., 26., 28.,
        ];
        let quantile_ci = ci_sorted_unchecked(0.95, &data, 0.25);
        assert_eq!(quantile_ci, Some((8., 20.)));
    }

    #[test]
    fn test_median_ci_unsorted() {
        use rand::seq::SliceRandom;
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let confidence = 0.95;
        let quantile = 0.5; // median
        for _i in 0..100 {
            let mut shuffled = data.to_vec();
            shuffled.shuffle(&mut thread_rng());
            if _i == 0 {
                println!("shuffled: {:?}", shuffled);
            }
            let interval = ci(confidence, &shuffled, quantile).unwrap();
            assert_eq!(interval, Interval::new(4, 12));
        }
    }
}
