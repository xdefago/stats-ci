//!
//! Interval over a partially ordered type (NB: floating point numbers are only partially ordered because of `NaN`).
//!

use num_traits::float::FloatCore;
use std::ops::RangeBounds;
use std::ops::Sub;
use std::ops::{RangeFrom, RangeInclusive, RangeToInclusive};

/// Interval over a partially ordered type (NB: floating point numbers are partially ordered because of `NaN`).
/// The interval is defined by its lower and upper bounds.
/// One-sided intervals (with a single concrete bound) are also supported.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), stats_ci::error::IntervalError> {
/// use stats_ci::*;
///
/// let interval = Interval::new(0., 10.)?;
/// assert_eq!(interval.low(), Some(0.));
/// assert_eq!(interval.high(), Some(10.));
/// assert!(interval.contains(&5.));
/// assert!(!interval.contains(&20.));
///
/// let interval = Interval::try_from(0..=10)?;
/// assert_eq!(interval.low(), Some(0));
/// assert_eq!(interval.high(), Some(10));
/// # Ok(())
/// # }
/// ```
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Interval<T>
where
    T: PartialOrd,
{
    TwoSided(T, T),   // [T, T]
    UpperOneSided(T), // [T, +inf)
    LowerOneSided(T), // (-inf, T]
}

impl<T: PartialOrd> Interval<T> {
    ///
    /// Create a new interval from its left and right bounds for ordered types with equality.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> stats_ci::CIResult<()> {
    /// # use stats_ci::Interval;
    /// let interval = Interval::new(0., 1.)?;
    /// assert_eq!(interval.low(), Some(0.));
    /// assert_eq!(interval.high(), Some(1.));
    /// let interval2 = Interval::new("A", "Z")?;
    /// assert_eq!(interval2.low(), Some("A"));
    /// assert_eq!(interval2.high(), Some("Z"));
    /// let interval3 = Interval::new(0, 0_usize)?;
    /// assert_eq!(interval3.low(), Some(0));
    /// assert_eq!(interval3.high(), Some(0));
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(low: T, high: T) -> Result<Self, IntervalError> {
        if low > high {
            Err(IntervalError::InvalidBounds)
        } else {
            Ok(Interval::TwoSided(low, high))
        }
    }

    ///
    /// Create a new upper one-sided interval from its left bound.
    /// The interval is defined as [low, +∞).
    /// The bound is included in the interval.
    ///
    /// # Examples
    ///
    /// The interval below represents [0., +∞).
    /// ```
    /// # fn main() -> stats_ci::CIResult<()> {
    /// # use stats_ci::Interval;
    /// let interval = Interval::new_upper(0.);
    /// assert_eq!(interval.low(), Some(0.));
    /// assert_eq!(interval.high(), None);
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub fn new_upper(low: T) -> Self {
        Interval::UpperOneSided(low)
    }

    ///
    /// Create a new lower one-sided interval from its right bound.
    /// The interval is defined as (-∞, high].
    /// The bound is included in the interval.
    ///
    /// # Examples
    ///
    /// The interval below represents (-∞, 1.]
    /// ```
    /// # fn main() -> stats_ci::CIResult<()> {
    /// # use stats_ci::Interval;
    /// let interval = Interval::new_lower(1.);
    /// assert_eq!(interval.low(), None);
    /// assert_eq!(interval.high(), Some(1.));
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub fn new_lower(high: T) -> Self {
        Interval::LowerOneSided(high)
    }

    ///
    /// Test whether the interval is two-sided.
    ///
    pub fn is_two_sided(&self) -> bool {
        matches!(self, Interval::TwoSided(_, _))
    }

    ///
    /// Test whether the interval is one-sided.
    ///
    pub fn is_one_sided(&self) -> bool {
        !self.is_two_sided()
    }

    ///
    /// Test whether the interval is an upper one-sided interval.
    ///
    pub fn is_upper(&self) -> bool {
        matches!(self, Interval::UpperOneSided(_))
    }

    ///
    /// Test whether the interval is a lower one-sided interval.
    ///
    pub fn is_lower(&self) -> bool {
        matches!(self, Interval::LowerOneSided(_))
    }

    ///
    /// Test whether the interval contains a value.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> stats_ci::CIResult<()> {
    /// # use stats_ci::Interval;
    /// let interval = Interval::new(0., 1.)?;
    /// assert!(interval.contains(&0.5));
    /// assert!(!interval.contains(&2.));
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub fn contains(&self, x: &T) -> bool {
        match self {
            Interval::TwoSided(low, high) => low <= x && x <= high,
            Interval::UpperOneSided(low) => low <= x,
            Interval::LowerOneSided(high) => x <= high,
        }
    }

    ///
    /// Test whether the interval intersects another interval.
    /// Two intervals are considered to intersect even if they only have a single point in common (e.g., one of their bounds).
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> stats_ci::CIResult<()> {
    /// # use stats_ci::Interval;
    /// let interval = Interval::new(0., 1.)?;
    /// let interval2 = Interval::new(0.5, 1.5)?;
    /// assert!(interval.intersects(&interval2));
    /// let interval3 = Interval::new(2., 3.)?;
    /// assert!(!interval.intersects(&interval3));
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub fn intersects(&self, other: &Self) -> bool {
        match (self, other) {
            (Interval::UpperOneSided(_), Interval::UpperOneSided(_)) => true,
            (Interval::LowerOneSided(_), Interval::LowerOneSided(_)) => true,
            (Interval::UpperOneSided(x), Interval::LowerOneSided(y) | Interval::TwoSided(_, y)) => {
                x <= y
            }
            (Interval::LowerOneSided(x), Interval::UpperOneSided(y) | Interval::TwoSided(_, y)) => {
                x <= y
            }
            (Interval::TwoSided(x, y), Interval::UpperOneSided(z) | Interval::LowerOneSided(z)) => {
                x <= z && z <= y
            }
            (Interval::TwoSided(x, y), Interval::TwoSided(a, b)) => x <= b && a <= y,
        }
    }
    ///
    /// Test whether the interval is included in another interval.
    ///
    /// The inclusion is not strict, i.e. an interval is included in itself.
    ///
    pub fn is_included_in(&self, other: &Self) -> bool {
        other.includes(self)
    }
    ///
    /// Test whether the interval includes another interval.
    ///
    /// The inclusion is not strict, i.e. an interval includes itself.
    ///
    pub fn includes(&self, other: &Self) -> bool {
        match (self, other) {
            (Interval::UpperOneSided(x), Interval::UpperOneSided(y)) => x <= y,
            (Interval::LowerOneSided(x), Interval::LowerOneSided(y)) => x >= y,
            (Interval::UpperOneSided(x), Interval::TwoSided(y, _)) => x <= y,
            (Interval::LowerOneSided(x), Interval::TwoSided(_, y)) => x >= y,
            (Interval::TwoSided(x, y), Interval::TwoSided(a, b)) => x <= a && b <= y,
            (Interval::UpperOneSided(_), Interval::LowerOneSided(_))
            | (Interval::LowerOneSided(_), Interval::UpperOneSided(_))
            | (Interval::TwoSided(_, _), Interval::UpperOneSided(_))
            | (Interval::TwoSided(_, _), Interval::LowerOneSided(_)) => false,
        }
    }

    ///
    /// Get the left bound of the interval (if any).
    ///
    pub fn left(&self) -> Option<&T> {
        match self {
            Interval::UpperOneSided(x) | Interval::TwoSided(x, _) => Some(x),
            Interval::LowerOneSided(_) => None,
        }
    }
    ///
    /// Get the right bound of the interval (if any).
    ///
    pub fn right(&self) -> Option<&T> {
        match self {
            Interval::LowerOneSided(x) | Interval::TwoSided(_, x) => Some(x),
            Interval::UpperOneSided(_) => None,
        }
    }
}

impl<T: PartialOrd + PartialEq> Interval<T> {
    ///
    /// Test whether the interval is degenerate.
    /// A degenerate interval is an interval with a single point.
    /// For example, the interval [0, 0] is degenerate.
    ///
    pub fn is_degenerate(&self) -> bool {
        match self {
            Interval::TwoSided(x, y) => x == y,
            _ => false,
        }
    }
}

impl<T: PartialOrd + Clone> Interval<T> {
    ///
    /// Get the lower bound of the interval (if any) for partially ordered types.
    ///
    /// This function clones the bound. If cloning is an issue, use [`Self::low_as_ref()`] instead.
    ///
    pub fn low(&self) -> Option<T> {
        self.left().cloned()
    }
    ///
    /// Get the upper bound of the interval (if any) for partially ordered types.
    ///
    /// This function clones the bound. If cloning is an issue, use [`Self::high_as_ref()`] instead.
    ///
    pub fn high(&self) -> Option<T> {
        self.right().cloned()
    }
}

impl<T: num_traits::Float> Interval<T> {
    ///
    /// Get the lower bound of the interval (if any) for floating point types.
    /// This function returns [`T::NEG_INFINITY`] for lower one-sided intervals.
    ///
    pub fn low_f(&self) -> T {
        match self {
            Interval::TwoSided(low, _) => *low,
            Interval::UpperOneSided(low) => *low,
            Interval::LowerOneSided(_) => T::neg_infinity(),
        }
    }
    ///
    /// Get the upper bound of the interval (if any) for floating point types.
    /// This function returns [`T::INFINITY`] for upper one-sided intervals.
    ///
    pub fn high_f(&self) -> T {
        match self {
            Interval::TwoSided(_, high) => *high,
            Interval::UpperOneSided(_) => T::infinity(),
            Interval::LowerOneSided(high) => *high,
        }
    }
}

impl<T: num_traits::PrimInt + num_traits::Signed> Interval<T> {
    ///
    /// Get the lower bound of the interval (if any) for signed integer types.
    /// This function returns [`T::MIN`] for lower one-sided intervals.
    ///
    pub fn low_i(&self) -> T {
        match self {
            Interval::TwoSided(low, _) => *low,
            Interval::UpperOneSided(low) => *low,
            Interval::LowerOneSided(_) => <T>::min_value(),
        }
    }
    ///
    /// Get the upper bound of the interval (if any) for signed integer types.
    /// This function returns [`T::MAX`] for upper one-sided intervals.
    ///
    pub fn high_i(&self) -> T {
        match self {
            Interval::TwoSided(_, high) => *high,
            Interval::UpperOneSided(_) => <T>::max_value(),
            Interval::LowerOneSided(high) => *high,
        }
    }
}
impl<T: num_traits::PrimInt + num_traits::Unsigned> Interval<T> {
    ///
    /// Get the lower bound of the interval (if any) for unsigned integer types.
    /// This function returns `0` for lower one-sided intervals.
    ///
    pub fn low_u(&self) -> T {
        match self {
            Interval::TwoSided(low, _) => *low,
            Interval::UpperOneSided(low) => *low,
            Interval::LowerOneSided(_) => <T>::min_value(),
        }
    }
    ///
    /// Get the upper bound of the interval (if any) for unsigned integer types.
    /// This function returns [`T::MAX`] for upper one-sided intervals.
    ///
    pub fn high_u(&self) -> T {
        match self {
            Interval::TwoSided(_, high) => *high,
            Interval::UpperOneSided(_) => <T>::max_value(),
            Interval::LowerOneSided(high) => *high,
        }
    }
}

impl<T: PartialOrd> Interval<T> {
    ///
    /// Get a reference to the lower bound of the interval (if any).
    ///
    /// See also [`Self::low()`] if cloning is not an issue.
    ///
    pub fn low_as_ref(&self) -> Option<&T> {
        self.left()
    }
    ///
    /// Get a reference to the upper bound of the interval (if any).
    ///
    /// See also [`Self::high()`] if cloning is not an issue.
    ///
    pub fn high_as_ref(&self) -> Option<&T> {
        self.right()
    }
}

impl<T: PartialOrd> TryFrom<(T, T)> for Interval<T> {
    type Error = IntervalError;

    ///
    /// Create a new interval from a tuple of bounds.
    /// The first element of the tuple is the lower bound, the second element is the upper bound.
    /// If the lower bound is greater than the upper bound, an error is returned.
    ///
    fn try_from(value: (T, T)) -> Result<Self, Self::Error> {
        if value.0 <= value.1 {
            Interval::new(value.0, value.1)
        } else {
            Err(IntervalError::InvalidBounds)
        }
    }
}

impl<T: PartialOrd> TryFrom<(Option<T>, Option<T>)> for Interval<T> {
    type Error = IntervalError;

    ///
    /// Create a new interval from a tuple of optional bounds.
    /// The first element of the tuple is the lower bound, the second element is the upper bound.
    /// If one of the bounds is `None`, the interval is one-sided.
    /// If both bounds are `None`, an error is returned.
    ///
    fn try_from(value: (Option<T>, Option<T>)) -> Result<Self, Self::Error> {
        match value {
            (Some(low), Some(high)) => Interval::new(low, high),
            (Some(low), None) => Ok(Interval::new_upper(low)),
            (None, Some(high)) => Ok(Interval::new_lower(high)),
            (None, None) => Err(IntervalError::EmptyInterval),
        }
    }
}

impl<T: PartialOrd + Clone> From<Interval<T>> for (Option<T>, Option<T>) {
    ///
    /// Convert an interval to a tuple of optional bounds.
    /// The first element of the tuple is the lower bound, the second element is the upper bound.
    /// If the interval is one-sided, one of the bounds is `None`,
    ///
    fn from(interval: Interval<T>) -> Self {
        match interval {
            Interval::TwoSided(low, high) => (Some(low), Some(high)),
            Interval::UpperOneSided(low) => (Some(low), None),
            Interval::LowerOneSided(high) => (None, Some(high)),
        }
    }
}

macro_rules! impl_for_ints {
    ( $( $x:ty ),+ ) => {
        $(
            impl From<Interval<$x>> for ($x, $x) {
                fn from(value: Interval<$x>) -> Self {
                    match value {
                        Interval::TwoSided(low, high) => (low, high),
                        Interval::UpperOneSided(low) => (low, <$x>::max_value()),
                        Interval::LowerOneSided(high) => (<$x>::min_value(), high),
                    }
                }
            }
        )*
    };
}
impl_for_ints!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize);

macro_rules! impl_for_floats {
    ( $( $x:ty ),+ ) => {
        $(
            impl From<Interval<$x>> for ($x, $x) {
                fn from(value: Interval<$x>) -> Self {
                    match value {
                        Interval::TwoSided(low, high) => (low, high),
                        Interval::UpperOneSided(low) => (low, <$x>::infinity()),
                        Interval::LowerOneSided(high) => (<$x>::neg_infinity(), high),
                    }
                }
            }
        )*
    };
}
impl_for_floats!(f32, f64);

impl<T: Ord> TryFrom<RangeInclusive<T>> for Interval<T> {
    type Error = IntervalError;

    /// Create an interval from an inclusive range.
    /// The range must be non-empty or the function will return an error.
    /// ```
    /// use stats_ci::Interval;
    /// use std::ops::RangeInclusive;
    /// let interval = Interval::try_from(1..=2);
    /// assert!(interval.is_ok());
    /// assert_eq!(interval.unwrap(), Interval::new(1, 2).unwrap());
    /// ```
    ///
    fn try_from(range: RangeInclusive<T>) -> Result<Self, Self::Error> {
        let (start, end) = range.into_inner();
        Interval::new(start, end)
    }
}

impl<T: PartialOrd> From<RangeFrom<T>> for Interval<T> {
    ///
    /// Create an upper one-sided interval from a range starting from a given value.
    ///
    /// ```
    /// use stats_ci::Interval;
    /// use std::ops::RangeFrom;
    /// let interval = Interval::from(1..);
    /// assert_eq!(interval, Interval::new_upper(1));
    /// ```
    ///
    fn from(range: RangeFrom<T>) -> Self {
        Interval::new_upper(range.start)
    }
}

impl<T: PartialOrd> From<RangeToInclusive<T>> for Interval<T> {
    ///
    /// Create a lower one-sided interval from a range ending at a given value.
    ///
    /// ```
    /// use stats_ci::Interval;
    /// use std::ops::RangeToInclusive;
    /// let interval = Interval::from(..=1);
    /// assert_eq!(interval, Interval::new_lower(1));
    /// ```
    ///
    fn from(range: RangeToInclusive<T>) -> Self {
        Interval::new_lower(range.end)
    }
}

impl<T: PartialOrd> RangeBounds<T> for Interval<T> {
    fn start_bound(&self) -> std::ops::Bound<&T> {
        match self.left() {
            Some(low) => std::ops::Bound::Included(low),
            None => std::ops::Bound::Unbounded,
        }
    }
    fn end_bound(&self) -> std::ops::Bound<&T> {
        match self.right() {
            Some(high) => std::ops::Bound::Excluded(high),
            None => std::ops::Bound::Unbounded,
        }
    }
}

impl<'a, T: PartialOrd + Sub<Output = T> + num_traits::Zero + Clone> Interval<T> {
    ///
    /// Compute the width of the interval.
    /// If the interval is one-sided, the function returns `None`.
    ///
    pub fn width(&self) -> Option<T> {
        match self {
            Interval::LowerOneSided(_) | Interval::UpperOneSided(_) => None,
            Interval::TwoSided(low, high) => Some(high.clone() - low.clone()),
        }
    }
}

impl<T: PartialOrd + Clone> Clone for Interval<T> {
    fn clone(&self) -> Self {
        match self {
            Interval::TwoSided(low, high) => Interval::TwoSided(low.clone(), high.clone()),
            Interval::UpperOneSided(low) => Interval::UpperOneSided(low.clone()),
            Interval::LowerOneSided(high) => Interval::LowerOneSided(high.clone()),
        }
    }
}

impl<T: PartialOrd + Copy> Copy for Interval<T> {}

use std::fmt::Display;
impl<T: PartialOrd + Display> Display for Interval<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::TwoSided(low, high) => write!(f, "[{}, {}]", low, high),
            Interval::UpperOneSided(low) => write!(f, "[{},->)", low),
            Interval::LowerOneSided(high) => write!(f, "(<-,{}]", high),
        }
    }
}

use std::hash::Hash;
impl<T: PartialOrd + Hash> Hash for Interval<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Interval::TwoSided(low, high) => {
                0.hash(state);
                low.hash(state);
                high.hash(state);
            }
            Interval::UpperOneSided(low) => {
                1.hash(state);
                low.hash(state);
            }
            Interval::LowerOneSided(high) => {
                2.hash(state);
                high.hash(state);
            }
        }
    }
}

impl<T: PartialOrd> AsRef<Self> for Interval<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T: PartialOrd> PartialOrd for Interval<T> {
    ///
    /// Compare two intervals.
    /// Given two intervals `a` and `b`, `a < b` if and only if the upper bound of `a` is less than the lower bound of `b`.
    /// Although interval bounds are inclusive, two intervals that overlap only at a single bound are considered ordered.
    /// E.g., intervals [x,y] is considered less than [a,b] if y==a and x<b.
    ///
    /// # Examples
    /// ```
    /// # fn main() -> stats_ci::CIResult<()> {
    /// # use std::cmp::Ordering;
    /// # use stats_ci::Interval;
    /// let a = Interval::new(0, 10)?;
    /// let b = Interval::new(10, 20)?;
    /// let c = Interval::new(11, 20)?;
    /// let d = Interval::new(0, 10)?;
    /// let e = Interval::new_upper(10);
    /// assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
    /// assert_eq!(a.partial_cmp(&c), Some(Ordering::Less));
    /// assert_eq!(a.partial_cmp(&d), Some(Ordering::Equal));
    /// assert_eq!(a.partial_cmp(&e), Some(Ordering::Less));
    /// assert_eq!(c.partial_cmp(&a), Some(Ordering::Greater));
    /// assert_eq!(b.partial_cmp(&c), None);
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;
        match (self, other) {
            (xy, ab) if xy == ab => Some(Equal),
            (
                Interval::UpperOneSided(low) | Interval::TwoSided(low, _),
                Interval::LowerOneSided(high) | Interval::TwoSided(_, high),
            ) if low >= high => Some(Greater),
            (
                Interval::LowerOneSided(high) | Interval::TwoSided(_, high),
                Interval::UpperOneSided(low) | Interval::TwoSided(low, _),
            ) if low >= high => Some(Less),
            _ => None,
        }
    }
}

///
/// An error type for interval creation.
///
#[derive(thiserror::Error, Debug)]
pub enum IntervalError {
    #[error("Invalid bounds: the left bound is greater than the right bound")]
    InvalidBounds,

    #[error("Empty interval")]
    EmptyInterval,
}

/*
 *      #   # #   # ### #####   ##### #####  #### #####  ####
 *      #   # ##  #  #    #       #   #     #       #   #
 *      #   # # # #  #    #       #   ###    ###    #    ###
 *      #   # #  ##  #    #       #   #         #   #       #
 *       ###  #   # ###   #       #   ##### ####    #   ####
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_new() -> Result<(), IntervalError> {
        let interval = Interval::new(0., 1.)?;
        assert_eq!(interval.low(), Some(0.));
        assert_eq!(interval.high(), Some(1.));
        assert_eq!(interval.low_f(), 0.);
        assert_eq!(interval.high_f(), 1.);
        assert!(interval.contains(&0.5));
        assert!(!interval.contains(&2.));
        assert!(!interval.is_degenerate());
        assert!(interval.is_two_sided());
        assert!(!interval.is_one_sided());
        assert!(!interval.is_lower());
        assert!(!interval.is_upper());

        let interval = Interval::new_lower(0.);
        assert_eq!(interval.low(), None);
        assert_eq!(interval.high(), Some(0.));
        assert_eq!(interval.low_f(), f64::NEG_INFINITY);
        assert_ne!(interval.low_f(), f64::MIN);
        assert_eq!(interval.high_f(), 0.);
        assert!(!interval.is_degenerate());
        assert!(!interval.is_two_sided());
        assert!(interval.is_one_sided());
        assert!(interval.is_lower());
        assert!(!interval.is_upper());

        let interval = Interval::new_upper(0.);
        assert_eq!(interval.low(), Some(0.));
        assert_eq!(interval.high(), None);
        assert_eq!(interval.low_f(), 0.);
        assert_eq!(interval.high_f(), f64::INFINITY);
        assert_ne!(interval.high_f(), f64::MAX);
        assert!(!interval.is_degenerate());
        assert!(!interval.is_two_sided());
        assert!(interval.is_one_sided());
        assert!(!interval.is_lower());
        assert!(interval.is_upper());

        let interval = Interval::new(10, 20)?;
        assert_eq!(interval.low(), Some(10));
        assert_eq!(interval.high(), Some(20));
        assert_eq!(interval.low_i(), 10);
        assert_eq!(interval.high_i(), 20);
        assert!(interval.contains(&15));
        assert!(!interval.contains(&30));
        assert!(!interval.is_degenerate());
        assert!(interval.is_two_sided());
        assert!(!interval.is_one_sided());
        assert!(!interval.is_lower());
        assert!(!interval.is_upper());

        let interval = Interval::new_lower(10_i64);
        assert_eq!(interval.low(), None);
        assert_eq!(interval.high(), Some(10));
        assert_eq!(interval.low_i(), std::i64::MIN);
        assert_eq!(interval.high_i(), 10);
        assert!(!interval.is_degenerate());
        assert!(!interval.is_two_sided());
        assert!(interval.is_one_sided());
        assert!(interval.is_lower());
        assert!(!interval.is_upper());

        let interval = Interval::new_lower(10_usize);
        assert_eq!(interval.low(), None);
        assert_eq!(interval.high(), Some(10));
        assert_eq!(interval.low_u(), 0);
        assert_eq!(interval.high_u(), 10);
        assert!(!interval.is_degenerate());
        assert!(!interval.is_two_sided());
        assert!(interval.is_one_sided());
        assert!(interval.is_lower());
        assert!(!interval.is_upper());

        let interval = Interval::new(10, 10)?;
        assert_eq!(interval.low(), Some(10));
        assert_eq!(interval.high(), Some(10));
        assert!(interval.is_degenerate());
        assert!(interval.is_two_sided());
        assert!(!interval.is_one_sided());
        assert!(!interval.is_lower());
        assert!(!interval.is_upper());
        Ok(())
    }

    #[test]
    fn test_interval_contains() -> Result<(), IntervalError> {
        let interval = Interval::new(0., 1.)?;
        assert!(interval.contains(&0.5));
        assert!(!interval.contains(&2.));
        Ok(())
    }

    #[test]
    fn test_interval_includes() -> Result<(), IntervalError> {
        let interval1 = Interval::new(0., 10.)?;
        let interval2 = Interval::new(0., 1.)?;
        let interval3 = Interval::new(0., 10.)?;
        let interval4 = Interval::new(1., 11.)?;
        let interval5 = Interval::new(10., 20.)?;
        let interval6 = Interval::new_upper(0.);
        let interval7 = Interval::new_upper(11.);
        let interval8 = Interval::new_upper(20.);
        let interval9 = Interval::new_lower(10.);
        let interval10 = Interval::new_lower(1.);
        let interval11 = Interval::new_lower(-1.);

        assert!(interval1.includes(&interval2));
        assert!(interval1.includes(&interval3));
        assert!(!interval1.includes(&interval4));
        assert!(!interval1.includes(&interval5));
        assert!(!interval1.includes(&interval6));
        assert!(!interval1.includes(&interval7));
        assert!(!interval1.includes(&interval8));
        assert!(!interval1.includes(&interval9));
        assert!(!interval1.includes(&interval10));

        assert!(!interval2.includes(&interval1));
        assert!(interval3.includes(&interval1));
        assert!(!interval4.includes(&interval1));
        assert!(!interval5.includes(&interval1));
        assert!(interval6.includes(&interval1));
        assert!(!interval7.includes(&interval1));
        assert!(!interval8.includes(&interval1));
        assert!(interval9.includes(&interval1));
        assert!(!interval10.includes(&interval1));
        assert!(!interval11.includes(&interval1));

        Ok(())
    }

    #[test]
    fn test_interval_compare() -> Result<(), IntervalError> {
        use std::cmp::Ordering::*;

        let interval1 = Interval::new(0., 10.)?;
        let interval2 = Interval::new(0., 1.)?;
        let interval3 = Interval::new(0., 10.)?;
        let interval4 = Interval::new(1., 11.)?;
        let interval5 = Interval::new(10., 20.)?;
        let interval6 = Interval::new_upper(0.);
        let interval7 = Interval::new_upper(11.);
        let interval8 = Interval::new_upper(20.);
        let interval9 = Interval::new_lower(10.);
        let interval10 = Interval::new_lower(1.);
        let interval11 = Interval::new_lower(-1.);

        assert_eq!(interval1.partial_cmp(&interval2), None);
        assert_eq!(interval1.partial_cmp(&interval3), Some(Equal));
        assert_eq!(interval1.partial_cmp(&interval4), None);
        assert_eq!(interval1.partial_cmp(&interval5), Some(Less));
        assert_eq!(interval1.partial_cmp(&interval6), None);
        assert_eq!(interval1.partial_cmp(&interval7), Some(Less));
        assert_eq!(interval1.partial_cmp(&interval8), Some(Less));
        assert_eq!(interval1.partial_cmp(&interval9), None);
        assert_eq!(interval1.partial_cmp(&interval10), None);
        assert_eq!(interval1.partial_cmp(&interval11), Some(Greater));

        Ok(())
    }

    #[test]
    fn test_interval_from_range() -> Result<(), IntervalError> {
        let interval = Interval::try_from(0..=3)?;
        assert_eq!(interval, Interval::new(0, 3)?);
        assert_eq!(interval.low(), Some(0));
        assert_eq!(interval.high(), Some(3));
        assert!(interval.contains(&1));
        assert!(!interval.contains(&10));

        let interval = Interval::from(10..);
        assert_eq!(interval, Interval::new_upper(10));
        assert_eq!(interval.low(), Some(10));
        assert_eq!(interval.high(), None);
        assert!(interval.contains(&10));
        assert!(interval.contains(&100));
        assert!(!interval.contains(&0));

        let interval = Interval::from(..=10);
        assert_eq!(interval, Interval::new_lower(10));
        assert_eq!(interval.low(), None);
        assert_eq!(interval.high(), Some(10));
        assert!(interval.contains(&10));
        assert!(!interval.contains(&100));
        assert!(interval.contains(&0));

        Ok(())
    }

    #[test]
    fn test_special_case() {
        assert!(Interval::new(10, 10).is_ok());
        assert!(Interval::new(10, 8).is_err());
    }

    #[test]
    fn test_interval_intersection() -> Result<(), IntervalError> {
        let interval1 = Interval::new(0, 10)?;
        let interval2 = Interval::new(5, 15)?;
        let interval3 = Interval::new(10, 20)?;
        let interval4 = Interval::new(15, 25)?;

        assert!(interval1.intersects(&interval2));
        assert!(interval2.intersects(&interval1));
        assert!(interval2.intersects(&interval3));
        assert!(interval3.intersects(&interval2));
        assert!(interval3.intersects(&interval4));
        assert!(interval4.intersects(&interval3));

        // intervals are assumed to be inclusive
        assert!(interval1.intersects(&interval3));
        assert!(interval3.intersects(&interval1));

        assert!(!interval1.intersects(&interval4));
        assert!(!interval4.intersects(&interval1));

        Ok(())
    }

    #[test]
    fn test_interval_equality() -> Result<(), IntervalError> {
        let interval1 = Interval::new(0, 10)?;
        let interval2 = Interval::new(0, 10)?;
        let interval3 = Interval::new(0, 11)?;
        let interval4 = Interval::new(1, 10)?;
        let interval5 = Interval::new(1, 11)?;

        assert_eq!(interval1, interval2);
        assert_ne!(interval1, interval3);
        assert_ne!(interval1, interval4);
        assert_ne!(interval1, interval5);

        Ok(())
    }

    #[test]
    fn test_width() -> Result<(), IntervalError> {
        assert_eq!(Interval::new(0, 10)?.width(), Some(10));
        assert_eq!(Interval::new(0, 0)?.width(), Some(0));
        assert_eq!(Interval::new(-10, 0)?.width(), Some(10));
        assert_eq!(Interval::new(-10, -10)?.width(), Some(0));

        Ok(())
    }

    #[test]
    fn test_from() -> Result<(), IntervalError> {
        let interval = Interval::try_from(0..=10)?;
        assert_eq!(interval.low(), Some(0));
        assert_eq!(interval.high(), Some(10));

        let tuple: (i32, i32) = interval.into();
        assert_eq!(tuple, (0, 10));

        let tuple: (f64, f64) = Interval::new_lower(0.).into();
        let (lo, hi) = tuple;
        assert!(lo.is_infinite());
        assert!(lo.is_sign_negative());
        assert_eq!(hi, 0.);

        let tuple: (f64, f64) = Interval::new_upper(0.).into();
        assert_eq!(tuple, (0., f64::INFINITY));
        assert_eq!(tuple, (0., f64::infinity()));

        let tuple: (f64, f64) = Interval::new_lower(0.).into();
        assert_eq!(tuple, (f64::NEG_INFINITY, 0.));
        assert_eq!(tuple, (f64::neg_infinity(), 0.));

        let tuple: (usize, usize) = Interval::new_lower(10).into();
        assert_eq!(tuple, (0, 10));

        let tuple: (usize, usize) = Interval::new_upper(10).into();
        assert_eq!(tuple, (10, usize::MAX));
        Ok(())
    }

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Interval<f64>>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Interval<f64>>();
    }
}
