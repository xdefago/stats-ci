//!
//! Implements the [`Interval`] enum, which represents a confidence interval
//! over a partially ordered type
//!
//! Note that floating point numbers are only partially ordered because of `NaN` values.
//!

use core::ops::{Add, Div, Mul, Neg, Sub};
use core::ops::{Bound, RangeBounds};
use core::ops::{RangeFrom, RangeInclusive, RangeToInclusive};
use num_traits::float::FloatCore;
use num_traits::Num;

/// Interval over a partially ordered type (NB: floating point numbers are only partially ordered because of `NaN` values).
/// The interval is defined by its lower and upper bounds. One-sided intervals (with a single concrete bound) are also supported.
/// In this crate, intervals are considered inclusive of their (finite) bounds.
///
/// ## Type parameters
///
/// * `T`: The type over which the interval is defined. It must be partially ordered.
///
/// ## Variants
///
/// * `TwoSided(T, T)`: Two-sided interval with lower and upper bounds. The interval is defined as [low, high]. The bounds are included in the interval.
/// * `UpperOneSided(T)`: Upper one-sided interval with a lower bound. The interval is defined as [low, +∞). The lower bound is included in the interval.
/// * `LowerOneSided(T)`: Lower one-sided interval with an upper bound. The interval is defined as (-∞, high]. The upper bound is included in the interval.
///
/// Intervals support various operations that depend on the type `T` over which they are defined.
///
/// ## Operations
///
/// ### Creation
///
/// * [`Self::new(low, high)`](#method.new): Create a new interval from its left and right bounds for ordered types with equality.
/// * [`Self::new_upper(low)`](#method.new_upper): Create a new upper one-sided interval from its left bound.
/// * [`Self::new_lower(high)`](#method.new_lower): Create a new lower one-sided interval from its right bound.
///
/// ### Accessors
///
/// * [`Self::low()`](#method.low): Get the lower bound of the interval (if any) for partially ordered types.
/// * [`Self::high()`](#method.high): Get the upper bound of the interval (if any) for partially ordered types.
/// * [`Self::low_f()`](#method.low_f): Get the lower bound of the interval (if any) for floating point types.
/// * [`Self::high_f()`](#method.high_f): Get the upper bound of the interval (if any) for floating point types.
/// * [`Self::low_i()`](#method.low_i): Get the lower bound of the interval (if any) for signed integer types.
/// * [`Self::high_i()`](#method.high_i): Get the upper bound of the interval (if any) for signed integer types.
/// * [`Self::low_u()`](#method.low_u): Get the lower bound of the interval (if any) for unsigned integer types.
/// * [`Self::high_u()`](#method.high_u): Get the upper bound of the interval (if any) for unsigned integer types.
/// * [`Self::low_as_ref()`](#method.low_as_ref): Get a reference to the lower bound of the interval (if any).
/// * [`Self::high_as_ref()`](#method.high_as_ref): Get a reference to the upper bound of the interval (if any).
/// * [`Self::left()`](#method.left): Get the left bound of the interval (if any).
/// * [`Self::right()`](#method.right): Get the right bound of the interval (if any).
///
/// ### Characteristics
///
/// * [`Self::is_two_sided()`](#method.is_two_sided): Test whether the interval is two-sided.
/// * [`Self::is_one_sided()`](#method.is_one_sided): Test whether the interval is one-sided.
/// * [`Self::is_upper()`](#method.is_upper): Test whether the interval is an upper one-sided interval.
/// * [`Self::is_lower()`](#method.is_lower): Test whether the interval is a lower one-sided interval.
/// * [`Self::is_degenerate()`](#method.is_degenerate): Test whether the interval is degenerate.
///
/// ### Comparison
///
/// * [`Self::intersects(other)`](#method.intersects): Test whether the interval intersects another interval.
/// * [`Self::is_included_in(other)`](#method.is_included_in): Test whether the interval is included in another interval.
/// * [`Self::includes(other)`](#method.includes): Test whether the interval includes another interval.
/// * [`Self::contains(x)`](#method.contains): Test whether the interval contains a value.
/// * approximate equality with [`approx`](https://docs.rs/approx/0.3.3/approx/) if the `approx` feature is enabled.
///
/// ### Operators with a scalar value
///
/// * [`Self::mul(rhs)`](#method.mul): Multiply the interval by a value.
/// * [`Self::div(rhs)`](#method.div): Divide the interval by a value.
/// * [`Self::add(rhs)`](#method.add): Add a value to the interval.
/// * [`Self::sub(rhs)`](#method.sub): Subtract a value from the interval.
///
/// ### Operators with another interval
///
/// * [`Self::relative_to(reference)`](#method.relative_to): Given two intervals, compute the relative interval compared to the reference (argument). The relative interval is defined as the interval of the ratios of the two intervals.
///
/// ### Conversions
///
/// * [`Self::try_from(value)`](#method.try_from): Create a new interval from a tuple of bounds. The first element of the tuple is the lower bound, the second element is the upper bound. If the lower bound is greater than the upper bound, an error is returned.
/// * [`Self::from(range)`](#method.from): Create a new interval from a range. The range must be bounded. If the lower bound is greater than the upper bound, an error is returned.
///
/// ### Display
///
/// * [`Self::fmt()`](#method.fmt): Format the interval as a string.
///  
/// # Examples
///
/// ## Creation
/// ```
/// use stats_ci::*;
/// let interval = Interval::new(0., 10.)?;
/// let interval = Interval::new_upper(0.);
/// let interval = Interval::new_lower(10.);
/// # Ok::<(),stats_ci::error::IntervalError>(())
/// ```
///
/// ## Accessors
/// ```
/// # use stats_ci::*;
/// let interval = Interval::new(0., 10.)?;
/// assert_eq!(interval.low(), Some(0.));
/// assert_eq!(interval.high(), Some(10.));
/// assert_eq!(interval.low_f(), 0.);
/// assert_eq!(interval.high_f(), 10.);
/// assert_eq!(interval.width(), Some(10.));
/// assert_eq!(interval.is_one_sided(), false);
/// assert_eq!(interval.is_two_sided(), true);
/// assert_eq!(interval.is_upper(), false);
/// assert_eq!(interval.is_lower(), false);
/// assert_eq!(interval.is_degenerate(), false);
/// # Ok::<(),stats_ci::error::IntervalError>(())
/// ```
///
/// ## Comparison
/// ```
/// # use stats_ci::*;
/// let interval = Interval::new(0., 10.)?;
/// let interval2 = Interval::new(8., 15.)?;
/// let interval3 = Interval::new(2., 5.)?;
/// assert!(interval.intersects(&interval2));
/// assert!(interval3.is_included_in(&interval));
/// assert!(interval.includes(&interval3));
/// assert!(interval3 < interval2);
/// assert!(interval == Interval::new(0., 10.)?);
/// assert!(interval.contains(&5.));
/// assert!(!interval.contains(&20.));
/// # Ok::<(),stats_ci::error::IntervalError>(())
/// ```
///
/// ## Operations
/// ```
/// # use stats_ci::*;
/// let interval = Interval::new(2., 4.)?;
/// assert_eq!(interval * 2., Interval::new(4., 8.)?);
/// assert_eq!(interval + 2., Interval::new(4., 6.)?);
/// assert_eq!(interval - 2., Interval::new(0., 2.)?);
/// assert_eq!(interval / 2., Interval::new(1., 2.)?);
/// # Ok::<(),stats_ci::error::IntervalError>(())
/// ```
///
/// ## Conversions
/// ```
/// # use stats_ci::*;
/// let interval = Interval::new(2., 4.)?;
/// assert_eq!(interval, Interval::try_from(2. ..= 4.)?);
/// assert_eq!(interval, Interval::try_from((2., 4.))?);
/// assert_eq!(interval, Interval::try_from((Some(2.), Some(4.)))?);
/// assert_eq!(Interval::from(..= 10.), Interval::new_lower(10.));
/// assert_eq!(Interval::from(2. ..), Interval::new_upper(2.));
/// assert_eq!(format!("{}", interval), String::from("[2, 4]"));
/// assert_eq!(format!("{}", Interval::new_lower(3.)), String::from("(<-,3]"));
/// assert_eq!(format!("{}", Interval::new_upper(2.)), String::from("[2,->)"));
/// # Ok::<(),stats_ci::error::IntervalError>(())
/// ```
///
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Interval<T>
where
    T: PartialOrd,
{
    ///
    /// Two-sided interval with lower and upper bounds.
    /// The interval is defined as [low, high].
    /// The bounds are included in the interval.
    ///
    TwoSided(T, T), // [T, T]

    ///
    /// Upper one-sided interval with a lower bound.
    /// The interval is defined as [low, +∞).
    /// The lower bound is included in the interval.
    ///
    UpperOneSided(T), // [T, +inf)

    ///
    /// Lower one-sided interval with an upper bound.
    /// The interval is defined as (-∞, high].
    /// The upper bound is included in the interval.
    ///
    LowerOneSided(T), // (-inf, T]
}

impl<T: PartialOrd> Interval<T> {
    ///
    /// Create a new interval from its left and right bounds for ordered types with equality.
    ///
    /// # Examples
    ///
    /// ```
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
    /// # Ok::<(),stats_ci::error::IntervalError>(())
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
    /// # use stats_ci::Interval;
    /// let interval = Interval::new_upper(0.);
    /// assert_eq!(interval.low(), Some(0.));
    /// assert_eq!(interval.high(), None);
    /// # Ok::<(),stats_ci::error::IntervalError>(())
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
    /// # use stats_ci::Interval;
    /// let interval = Interval::new_lower(1.);
    /// assert_eq!(interval.low(), None);
    /// assert_eq!(interval.high(), Some(1.));
    /// # Ok::<(),stats_ci::error::IntervalError>(())
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
    /// # use stats_ci::Interval;
    /// let interval = Interval::new(0., 1.)?;
    /// assert!(interval.contains(&0.5));
    /// assert!(!interval.contains(&2.));
    /// # Ok::<(),stats_ci::error::IntervalError>(())
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
    /// # use stats_ci::Interval;
    /// let interval = Interval::new(0., 1.)?;
    /// let interval2 = Interval::new(0.5, 1.5)?;
    /// assert!(interval.intersects(&interval2));
    /// let interval3 = Interval::new(2., 3.)?;
    /// assert!(!interval.intersects(&interval3));
    /// # Ok::<(),stats_ci::error::IntervalError>(())
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
    /// This function returns the negative infinite value for `T` for lower one-sided intervals.
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
    /// This function returns the infinite value for `T` for upper one-sided intervals.
    ///
    pub fn high_f(&self) -> T {
        match self {
            Interval::TwoSided(_, high) => *high,
            Interval::UpperOneSided(_) => T::infinity(),
            Interval::LowerOneSided(high) => *high,
        }
    }

    ///
    /// Given two intervals, compute the relative interval compared to the reference (argument).
    /// The relative interval is defined as the interval of the ratios of the two intervals.
    ///
    /// E.g., for two two-sided intervals \\( [x, y] \\) and reference \\( [a, b] \\), the relative interval is \\( [(x-b)/b, (y-a)/a] \\).
    ///
    pub fn relative_to(&self, reference: &Interval<T>) -> Interval<T> {
        match (reference, self) {
            (Interval::TwoSided(a, b), _) if a.is_zero() || b.is_zero() => {
                panic!("Cannot compute relative interval to a zero interval");
            }
            (Interval::LowerOneSided(a) | Interval::UpperOneSided(a), _) if a.is_zero() => {
                panic!("Cannot compute relative interval to a zero interval");
            }
            (&Interval::TwoSided(a, b), &Interval::TwoSided(x, y)) => {
                Interval::TwoSided((x - b) / b, (y - a) / a)
            }
            (
                &Interval::UpperOneSided(a) | &Interval::TwoSided(a, _),
                &Interval::LowerOneSided(y) | &Interval::TwoSided(_, y),
            ) => Interval::LowerOneSided((y - a) / a),
            (
                &Interval::LowerOneSided(b) | &Interval::TwoSided(_, b),
                &Interval::UpperOneSided(x) | &Interval::TwoSided(x, _),
            ) => Interval::UpperOneSided((x - b) / b),
            (&Interval::UpperOneSided(_), &Interval::UpperOneSided(_))
            | (&Interval::LowerOneSided(_), &Interval::LowerOneSided(_)) => {
                panic!(
                    "Cannot compute relative interval to one-sided interval with same direction"
                );
            }
        }
    }
}

impl<T: num_traits::PrimInt + num_traits::Signed> Interval<T> {
    ///
    /// Get the lower bound of the interval (if any) for signed integer types.
    /// This function returns the minimal value for `T` for lower one-sided intervals.
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
    /// This function returns the maximal value for `T` for upper one-sided intervals.
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
    /// This function returns the maximum value for `T` for upper one-sided intervals.
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
impl<T: PartialOrd + Copy> Interval<T> {
    fn applied<F>(&self, f_low: F, f_high: F) -> Self
    where
        F: FnOnce(T) -> T,
    {
        match self {
            Interval::TwoSided(low, high) => Interval::TwoSided(f_low(*low), f_high(*high)),
            Interval::LowerOneSided(low) => Interval::UpperOneSided(f_low(*low)),
            Interval::UpperOneSided(high) => Interval::LowerOneSided(f_high(*high)),
        }
    }

    fn applied_both<F>(&self, f: F) -> Self
    where
        F: Fn(T) -> T,
    {
        self.applied(&f, &f)
    }
}

#[cfg(feature = "approx")]
impl<T: approx::AbsDiffEq + PartialOrd> approx::AbsDiffEq for Interval<T>
where
    T::Epsilon: Copy,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        match (self, other) {
            (Interval::TwoSided(a, b), Interval::TwoSided(x, y)) => {
                T::abs_diff_eq(a, x, epsilon) && T::abs_diff_eq(b, y, epsilon)
            }
            (Interval::UpperOneSided(a), Interval::UpperOneSided(x)) => {
                T::abs_diff_eq(a, x, epsilon)
            }
            (Interval::LowerOneSided(b), Interval::LowerOneSided(y)) => {
                T::abs_diff_eq(b, y, epsilon)
            }
            _ => false,
        }
    }
}

#[cfg(feature = "approx")]
impl<T: approx::RelativeEq + PartialOrd> approx::RelativeEq for Interval<T>
where
    T::Epsilon: Copy,
{
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
        match (self, other) {
            (Interval::TwoSided(a, b), Interval::TwoSided(x, y)) => {
                T::relative_eq(a, x, epsilon, max_relative)
                    && T::relative_eq(b, y, epsilon, max_relative)
            }
            (Interval::UpperOneSided(a), Interval::UpperOneSided(x)) => {
                T::relative_eq(a, x, epsilon, max_relative)
            }
            (Interval::LowerOneSided(b), Interval::LowerOneSided(y)) => {
                T::relative_eq(b, y, epsilon, max_relative)
            }
            _ => false,
        }
    }
}

#[cfg(feature = "approx")]
impl<T: approx::UlpsEq + PartialOrd> approx::UlpsEq for Interval<T>
where
    T::Epsilon: Copy,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        match (self, other) {
            (Interval::TwoSided(a, b), Interval::TwoSided(x, y)) => {
                T::ulps_eq(a, x, epsilon, max_ulps) && T::ulps_eq(b, y, epsilon, max_ulps)
            }
            (Interval::UpperOneSided(a), Interval::UpperOneSided(x)) => {
                T::ulps_eq(a, x, epsilon, max_ulps)
            }
            (Interval::LowerOneSided(b), Interval::LowerOneSided(y)) => {
                T::ulps_eq(b, y, epsilon, max_ulps)
            }
            _ => false,
        }
    }
}

impl<F: Mul<F, Output = F> + PartialOrd + Copy> Mul<F> for Interval<F> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        self.applied_both(|x| x * rhs)
    }
}

impl<F: Div<F, Output = F> + PartialOrd + Copy> Div<F> for Interval<F> {
    type Output = Self;

    fn div(self, rhs: F) -> Self::Output {
        self.applied_both(|x| x / rhs)
    }
}

impl<F: Add<F, Output = F> + PartialOrd + Copy> Add<F> for Interval<F> {
    type Output = Self;

    fn add(self, rhs: F) -> Self::Output {
        self.applied_both(|x| x + rhs)
    }
}

impl<F: Sub<F, Output = F> + PartialOrd + Copy> Sub<F> for Interval<F> {
    type Output = Self;

    fn sub(self, rhs: F) -> Self::Output {
        self.applied_both(|x| x - rhs)
    }
}

impl<F: Neg<Output = F> + PartialOrd + Copy> Neg for Interval<F> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.applied_both(|x| -x)
    }
}

impl<F: Num + PartialOrd + Copy> Add for Interval<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Interval::TwoSided(a, b), Interval::TwoSided(x, y)) => {
                Interval::TwoSided(a + x, b + y)
            }
            (Interval::TwoSided(a, _) | Interval::UpperOneSided(a), Interval::UpperOneSided(x)) => {
                Interval::UpperOneSided(a + x)
            }
            (Interval::TwoSided(_, b) | Interval::LowerOneSided(b), Interval::LowerOneSided(y)) => {
                Interval::LowerOneSided(b + y)
            }
            (Interval::UpperOneSided(a), Interval::TwoSided(x, _)) => {
                Interval::UpperOneSided(a + x)
            }
            (Interval::LowerOneSided(b), Interval::TwoSided(_, y)) => {
                Interval::LowerOneSided(b + y)
            }
            (Interval::UpperOneSided(_), Interval::LowerOneSided(_))
            | (Interval::LowerOneSided(_), Interval::UpperOneSided(_)) => {
                panic!("Cannot add one-sided intervals with different directions (all values interval)")
            }
        }
    }
}

impl<F: Num + PartialOrd + Copy> Sub for Interval<F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Interval::TwoSided(a, b), Interval::TwoSided(x, y)) => {
                Interval::TwoSided(a - y, b - x)
            }
            (Interval::TwoSided(_, b) | Interval::LowerOneSided(b), Interval::UpperOneSided(x)) => {
                Interval::LowerOneSided(b - x)
            }
            (Interval::TwoSided(a, _) | Interval::UpperOneSided(a), Interval::LowerOneSided(y)) => {
                Interval::UpperOneSided(a - y)
            }
            (Interval::UpperOneSided(a), Interval::TwoSided(_, y)) => {
                Interval::UpperOneSided(a - y)
            }
            (Interval::LowerOneSided(b), Interval::TwoSided(x, _)) => {
                Interval::LowerOneSided(b - x)
            }
            (Interval::UpperOneSided(_), Interval::UpperOneSided(_))
            | (Interval::LowerOneSided(_), Interval::LowerOneSided(_)) => {
                panic!(
                    "Cannot subtract one-sided intervals of the same directions (empty interval)"
                )
            }
        }
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

impl<T: PartialOrd> TryFrom<RangeInclusive<T>> for Interval<T> {
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
    fn start_bound(&self) -> Bound<&T> {
        match self.left() {
            Some(low) => Bound::Included(low),
            None => Bound::Unbounded,
        }
    }

    fn end_bound(&self) -> Bound<&T> {
        match self.right() {
            Some(high) => Bound::Excluded(high),
            None => Bound::Unbounded,
        }
    }
}

impl<T: PartialOrd + Sub<Output = T> + num_traits::Zero + Clone> Interval<T> {
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

use core::fmt::Display;
impl<T: PartialOrd + Display> Display for Interval<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Interval::TwoSided(low, high) => write!(f, "[{}, {}]", low, high),
            Interval::UpperOneSided(low) => write!(f, "[{},->)", low),
            Interval::LowerOneSided(high) => write!(f, "(<-,{}]", high),
        }
    }
}

use core::hash::Hash;
impl<T: PartialOrd + Hash> Hash for Interval<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
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
    /// E.g., intervals `[x,y]` is considered less than `[a,b]` if `y==a` and `x<b`.
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
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        use core::cmp::Ordering::*;
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

pub use crate::error::IntervalError;

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

    #[test]
    fn test_approx() {
        use approx::*;

        let interval1 = Interval::new(0., 10.).unwrap();
        let interval2 = Interval::new(1e-7, 10.000000001).unwrap();
        assert!(interval1.abs_diff_eq(&interval2, 1e-6));
        assert_abs_diff_eq!(interval1, interval2, epsilon = 1e-6);
    }
}
