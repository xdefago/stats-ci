//! Implements the [`Confidence`] enum, which represents a confidence interval.
//!

/// Confidence level of a confidence interval.
///
/// # Examples
///
/// To create a two-sided confidence interval with 95% confidence:
/// ```
/// # use stats_ci::Confidence;
/// #
/// let confidence = Confidence::new_two_sided(0.95);
/// ```
///
/// To create an upper one-sided confidence interval with 90% confidence:
/// ```
/// # use stats_ci::Confidence;
/// #
/// let confidence = Confidence::new_upper(0.9);
/// ```
///
/// To create a lower one-sided confidence interval with 99% confidence:
/// ```
/// # use stats_ci::Confidence;
/// #
/// let confidence = Confidence::new_lower(0.99);
/// ```
///
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Confidence {
    /// Confidence for a two-sided interval.
    TwoSided(f64),

    /// Confidence for an upper one-sided interval.
    UpperOneSided(f64),

    /// Confidence for a lower one-sided interval.
    LowerOneSided(f64),
}

impl Confidence {
    /// Create a new two-sided confidence interval with the given confidence level.
    ///
    /// # Arguments
    ///
    /// * `confidence` - the confidence level, e.g. 0.95 for 95% confidence
    ///
    /// # Panics
    ///
    /// * if `confidence` is not in the range (0, 1)
    ///
    pub fn new_two_sided(confidence: f64) -> Self {
        assert!(confidence > 0. && confidence < 1.);
        //     if confidence <= 0. || confidence >= 1. { return Err(CIError::InvalidConfidenceLevel(confidence)); }
        Confidence::TwoSided(confidence)
    }

    /// Create a new one-sided upper confidence interval with the given confidence level.
    ///
    /// # Arguments
    ///
    /// * `confidence` - the confidence level, e.g. 0.95 for 95% confidence
    ///
    /// # Panics
    ///
    /// * if `confidence` is not in the range (0, 1)
    ///
    pub fn new_upper(confidence: f64) -> Self {
        assert!(confidence > 0. && confidence < 1.);
        Confidence::UpperOneSided(confidence)
    }

    /// Create a new one-sided lower confidence interval with the given confidence level.
    ///
    /// # Arguments
    ///
    /// * `confidence` - the confidence level, e.g. 0.95 for 95% confidence
    ///
    /// # Panics
    ///
    /// * if `confidence` is not in the range (0, 1)
    ///
    pub fn new_lower(confidence: f64) -> Self {
        assert!(confidence > 0. && confidence < 1.);
        Confidence::LowerOneSided(confidence)
    }

    /// Return the confidence level of the interval as a number in the range (0, 1).
    ///
    pub fn level(&self) -> f64 {
        match self {
            Confidence::TwoSided(confidence)
            | Confidence::UpperOneSided(confidence)
            | Confidence::LowerOneSided(confidence) => *confidence,
        }
    }

    /// Return the confidence level of the interval as a percentage.
    ///
    pub fn percent(&self) -> f64 {
        self.level() * 100.
    }

    /// Return the kind of the confidence interval as a string (in English).
    ///
    pub fn kind(&self) -> &'static str {
        match self {
            Confidence::TwoSided(_) => "two-sided",
            Confidence::UpperOneSided(_) => "upper one-sided",
            Confidence::LowerOneSided(_) => "lower one-sided",
        }
    }

    /// Test if the confidence interval is two-sided.
    ///
    pub fn is_two_sided(&self) -> bool {
        matches!(self, Confidence::TwoSided(_))
    }

    /// Test if the confidence interval is one-sided.
    ///
    pub fn is_one_sided(&self) -> bool {
        !self.is_two_sided()
    }

    /// Test if the confidence interval is upper (one-sided).
    ///
    pub fn is_upper(&self) -> bool {
        matches!(self, Confidence::UpperOneSided(_))
    }

    /// Test if the confidence interval is lower (one-sided).
    ///
    pub fn is_lower(&self) -> bool {
        matches!(self, Confidence::LowerOneSided(_))
    }

    /// Return the confidence interval with the same confidence level but flipped.
    /// For a two-sided interval, this is the same interval.
    /// For a one-sided interval, this is the interval with the opposite direction.
    /// For example, a lower one-sided interval with confidence 0.95 flipped is an upper one-sided interval with confidence 0.95.
    ///
    pub fn flipped(&self) -> Self {
        match self {
            Confidence::TwoSided(_) => *self,
            Confidence::UpperOneSided(confidence) => Confidence::LowerOneSided(*confidence),
            Confidence::LowerOneSided(confidence) => Confidence::UpperOneSided(*confidence),
        }
    }

    /// Return the quantile of the confidence interval.
    ///
    /// For a two-sided interval, this is (1-\alpha/2) where \alpha is 1-confidence.
    /// For a one-sided interval, this is the confidence level.
    ///
    /// # Example
    ///
    /// `quantile()` returns 0.975 for two-sided 95% confidence.
    ///
    pub(crate) fn quantile(&self) -> f64 {
        match self {
            Confidence::TwoSided(confidence) => 1. - (1. - confidence) / 2.,
            Confidence::UpperOneSided(confidence) | Confidence::LowerOneSided(confidence) => {
                *confidence
            }
        }
    }
}

impl Default for Confidence {
    ///
    /// Create a new two-sided confidence interval with the default confidence level of 95%.
    ///
    fn default() -> Self {
        Confidence::new_two_sided(0.95)
    }
}

impl PartialOrd for Confidence {
    // NB: the partial ordering obtained from derivation rule is unsound, so we need to
    // implement it manually.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Confidence::TwoSided(x), Confidence::TwoSided(y))
            | (Confidence::UpperOneSided(x), Confidence::UpperOneSided(y))
            | (Confidence::LowerOneSided(x), Confidence::LowerOneSided(y)) => x.partial_cmp(y),
            _ => None,
        }
    }
}

use crate::error::CIError;
impl TryFrom<f64> for Confidence {
    type Error = CIError;

    fn try_from(confidence: f64) -> Result<Self, Self::Error> {
        if confidence > 0. && confidence < 1. {
            Ok(Confidence::new_two_sided(confidence))
        } else {
            Err(CIError::InvalidConfidenceLevel(confidence))
        }
    }
}

impl TryFrom<f32> for Confidence {
    type Error = CIError;

    fn try_from(confidence: f32) -> Result<Self, Self::Error> {
        Confidence::try_from(confidence as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        let two_sided = Confidence::new_two_sided(0.95);
        let upper = Confidence::new_upper(0.95);
        let lower = Confidence::new_lower(0.95);
        assert!(!(two_sided > upper));
        assert!(!(two_sided < upper));
        assert!(!(two_sided > lower));
        assert!(!(two_sided < lower));
        assert!(!(lower > upper));
        assert!(!(lower < upper));

        assert!(two_sided < Confidence::new_two_sided(0.99));
        assert!(two_sided > Confidence::new_two_sided(0.9));

        assert!(upper < Confidence::new_upper(0.99));
        assert!(upper > Confidence::new_upper(0.9));

        assert!(lower < Confidence::new_lower(0.99));
        assert!(lower > Confidence::new_lower(0.9));

        assert_eq!(two_sided, Confidence::new_two_sided(0.95));
        assert_eq!(upper, Confidence::new_upper(0.95));
        assert_eq!(lower, Confidence::new_lower(0.95));
    }

    #[test]
    fn test_quantile() {
        let two_sided = Confidence::new_two_sided(0.95);
        let upper = Confidence::new_upper(0.95);
        let lower = Confidence::new_lower(0.95);
        assert_eq!(two_sided.quantile(), 0.975);
        assert_eq!(upper.quantile(), 0.95);
        assert_eq!(lower.quantile(), 0.95);
    }

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Confidence>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Confidence>();
    }
}
