/// Interval over a partially ordered type (NB: floating point numbers are partially ordered because of `NaN`).
/// The interval is defined by its lower and upper bounds.
///
/// # Examples
///
/// ```
/// # use stats_ci::Interval;
/// let interval = Interval::new(0., 1.);
/// assert_eq!(interval.low().unwrap(), 0.);
/// assert_eq!(interval.high().unwrap(), 1.);
/// assert!(!interval.is_empty());
/// assert!(!interval.is_degenerate());
/// assert!(interval.is_concrete());
/// assert!(interval.contains(&0.5));
/// assert!(!interval.contains(&2.));
///
/// let interval2: Interval<_> = Interval::from(0..=10);
/// assert_eq!(interval2.low().unwrap(), 0);
/// assert_eq!(interval2.high().unwrap(), 10);
/// ```
#[derive(Debug, Default, PartialEq)]
pub enum Interval<T> {
    #[default]
    Empty,
    Degenerate(T),
    Concrete {
        left: T,
        right: T,
    },
}

impl<T: PartialOrd> Interval<T> {
    ///
    /// Create a new interval from its left and right bounds for ordered types with equality.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stats_ci::Interval;
    /// let interval = Interval::new(0., 1.);
    /// assert_eq!(interval.low().unwrap(), 0.);
    /// assert_eq!(interval.high().unwrap(), 1.);
    /// assert!(!interval.is_empty());
    /// let interval2 = Interval::new("A", "Z");
    /// assert_eq!(interval2.low().unwrap(), "A");
    /// assert_eq!(interval2.high().unwrap(), "Z");
    /// let interval3 = Interval::new(0, 0_usize);
    /// assert_eq!(interval3.low().unwrap(), 0);
    /// assert_eq!(interval3.high().unwrap(), 0);
    /// assert!(interval3.is_degenerate());
    /// let interval4 = Interval::new(1, 0);
    /// assert!(interval4.is_empty());
    /// ```
    pub fn new(low: T, high: T) -> Self {
        if low > high {
            Interval::Empty
        } else if low == high {
            Interval::Degenerate(low)
        } else {
            Interval::Concrete {
                left: low,
                right: high,
            }
        }
    }

    ///
    /// Test whether the interval contains a value.
    ///
    pub fn contains(&self, x: &T) -> bool {
        match self {
            Interval::Empty => false,
            Interval::Degenerate(y) => x == y,
            Interval::Concrete {
                left: low,
                right: high,
            } => low <= x && x <= high,
        }
    }
    ///
    /// Test whether the interval intersects another interval.
    ///
    pub fn intersects(&self, other: &Self) -> bool {
        match (self, other) {
            (Interval::Empty, _) | (_, Interval::Empty) => false,
            (Interval::Degenerate(x), _) => other.contains(x),
            (_, Interval::Degenerate(y)) => self.contains(y),
            (
                Interval::Concrete { left: x, right: y },
                Interval::Concrete { left: a, right: b },
            ) => x <= b && a <= y,
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
            (Interval::Empty, _) | (_, Interval::Empty) => false,
            (Interval::Degenerate(x), Interval::Degenerate(y)) => x == y,
            (Interval::Degenerate(_), _) => false,
            (_, Interval::Degenerate(y)) => self.contains(y),
            (
                Interval::Concrete { left: x, right: y },
                Interval::Concrete { left: a, right: b },
            ) => x <= a && b <= y,
        }
    }
}

impl<T> Interval<T> {
    ///
    /// Create a new interval from its left and right bounds for unordered types.
    /// The function is unchecked and always results in a concrete interval.
    /// NB: this function is not meant for ordered types; in particular for numerical types.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stats_ci::Interval;
    /// #[derive(Debug)]
    /// enum Directions { North, South, East, West};
    /// let interval = Interval::new_unordered_unchecked(Directions::North, Directions::West);
    /// assert!(matches!(interval.left().unwrap(), Directions::North));
    /// assert!(matches!(interval.right().unwrap(), Directions::West));
    /// assert!(!interval.is_empty());
    /// assert!(!interval.is_degenerate());
    /// assert!(interval.is_concrete());
    /// let interval = Interval::new_unordered_unchecked(Directions::North, Directions::North);
    /// // NB: the interval is not degenerate because the bounds equality is not checked
    /// assert!(!interval.is_degenerate());
    /// ```
    ///
    pub fn new_unordered_unchecked(left: T, right: T) -> Self {
        Interval::Concrete { left, right }
    }

    ///
    /// Test if the interval is empty.
    ///
    pub fn is_empty(&self) -> bool {
        matches!(self, Interval::Empty)
    }
    ///
    /// Test if the interval is degenerate, in the sense that it contains a single element.
    ///
    pub fn is_degenerate(&self) -> bool {
        matches!(self, Interval::Degenerate(_))
    }
    ///
    /// Test if the interval is concrete, in the sense that it contains at least two elements.
    ///
    pub fn is_concrete(&self) -> bool {
        matches!(self, Interval::Concrete { .. })
    }

    ///
    /// Get the left bound of the interval (if any).
    ///
    pub fn left(&self) -> Option<&T> {
        match self {
            Interval::Empty => None,
            Interval::Degenerate(x) => Some(x),
            Interval::Concrete { left: low, .. } => Some(low),
        }
    }
    ///
    /// Get the right bound of the interval (if any).
    ///
    pub fn right(&self) -> Option<&T> {
        match self {
            Interval::Empty => None,
            Interval::Degenerate(x) => Some(x),
            Interval::Concrete { right: high, .. } => Some(high),
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

impl<T: PartialOrd> Interval<T> {
    ///
    /// Get a reference to the lower bound of the interval (if any) for ordered types.
    ///
    /// See also [`Self::low()`] if cloning is not an issue.
    ///
    pub fn low_as_ref(&self) -> Option<&T> {
        self.left()
    }
    ///
    /// Get a reference to the upper bound of the interval (if any) for ordered types.
    ///
    /// See also [`Self::high()`] if cloning is not an issue.
    ///
    pub fn high_as_ref(&self) -> Option<&T> {
        self.right()
    }
}

impl<T: PartialEq> Interval<T> {
    ///
    /// Create a new interval from its left and right bounds for unordered types with equality.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stats_ci::Interval;
    /// #[derive(Debug, PartialEq)]
    /// enum Directions { North, South, East, West};
    /// let interval = Interval::new_unordered(Directions::North, Directions::West);
    /// assert_eq!(interval.left().unwrap(), &Directions::North);
    /// assert_eq!(interval.right().unwrap(), &Directions::West);
    /// assert!(!interval.is_empty());
    /// assert!(!interval.is_degenerate());
    /// assert!(interval.is_concrete());
    /// let interval = Interval::new_unordered(Directions::North, Directions::North);
    /// assert!(interval.is_degenerate());
    /// ```
    pub fn new_unordered(left: T, right: T) -> Self {
        if left == right {
            Interval::Degenerate(left)
        } else {
            Interval::Concrete { left, right }
        }
    }
}

impl<T: PartialOrd> From<(T, T)> for Interval<T> {
    fn from((low, high): (T, T)) -> Self {
        Interval::new(low, high)
    }
}

impl<T: PartialOrd> From<(Option<T>, Option<T>)> for Interval<T> {
    fn from((low, high): (Option<T>, Option<T>)) -> Self {
        match (low, high) {
            (Some(low), Some(high)) => Interval::new(low, high),
            (Some(low), None) => Interval::Degenerate(low),
            (None, Some(high)) => Interval::Degenerate(high),
            (None, None) => Interval::Empty,
        }
    }
}

impl<T: PartialOrd + Clone> From<Interval<T>> for (Option<T>, Option<T>) {
    fn from(interval: Interval<T>) -> Self {
        match interval {
            Interval::Empty => (None, None),
            Interval::Degenerate(x) => (Some(x.clone()), Some(x)),
            Interval::Concrete {
                left: low,
                right: high,
            } => (Some(low), Some(high)),
        }
    }
}

impl<T: PartialOrd + Clone> From<Interval<T>> for Option<(T, T)> {
    fn from(interval: Interval<T>) -> Self {
        match interval {
            Interval::Empty => None,
            Interval::Degenerate(x) => Some((x.clone(), x)),
            Interval::Concrete {
                left: low,
                right: high,
            } => Some((low, high)),
        }
    }
}

use std::ops::RangeBounds;
use std::ops::RangeInclusive;

impl<T: Ord> From<RangeInclusive<T>> for Interval<T> {
    fn from(range: RangeInclusive<T>) -> Self {
        let (start, end) = range.into_inner();
        Interval::new(start, end)
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

impl<T: std::ops::Sub<Output = T> + num_traits::Zero + Clone> Interval<T> {
    pub fn width(&self) -> Option<T> {
        match self {
            Interval::Empty => None,
            Interval::Degenerate(_) => Some(T::zero()),
            Interval::Concrete {
                left: low,
                right: high,
            } => Some(high.clone() - low.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval() {
        let interval = Interval::new(0., 1.);
        assert_eq!(interval.low().unwrap(), 0.);
        assert_eq!(interval.high().unwrap(), 1.);
        assert!(!interval.is_empty());
        assert!(!interval.is_degenerate());
        assert!(interval.is_concrete());
        assert!(interval.contains(&0.5));
        assert!(!interval.contains(&2.));
    }

    #[test]
    fn test_interval_from_range() {
        let interval = Interval::from(0..=3);
        assert_eq!(interval, Interval::new(0, 3));
        assert_eq!(interval.low().unwrap(), 0);
        assert_eq!(interval.high().unwrap(), 3);
        assert!(!interval.is_empty());
        assert!(!interval.is_degenerate());
        assert!(interval.is_concrete());
        assert!(interval.contains(&1));
        assert!(!interval.contains(&10));
    }

    #[test]
    fn test_special_case() {
        let interval = Interval::new(10, 10);
        assert!(interval.is_degenerate());
        assert_eq!(interval.low(), interval.high());
        assert!(interval.contains(&10));
        assert!(!interval.contains(&9));
        assert!(!interval.contains(&11));

        let interval = Interval::new(10, 8);
        assert!(interval.is_empty());
        assert_eq!(interval.low(), None);
        assert_eq!(interval.high(), None);
        assert!(!interval.contains(&8));
        assert!(!interval.contains(&9));
        assert!(!interval.contains(&10));
    }

    #[test]
    fn test_interval_intersection() {
        let interval1 = Interval::new(0, 10);
        let interval2 = Interval::new(5, 15);
        let interval3 = Interval::new(10, 20);
        let interval4 = Interval::new(15, 25);

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
    }

    #[test]
    fn test_interval_equality() {
        let interval1 = Interval::new(0, 10);
        let interval2 = Interval::new(0, 10);
        let interval3 = Interval::new(0, 11);
        let interval4 = Interval::new(1, 10);
        let interval5 = Interval::new(1, 11);

        assert_eq!(interval1, interval2);
        assert_ne!(interval1, interval3);
        assert_ne!(interval1, interval4);
        assert_ne!(interval1, interval5);
    }

    #[test]
    fn test_width() {
        let interval1 = Interval::new(0, 10);
        let interval2 = Interval::new(0, 0);
        let interval3 = Interval::new(0, -10);
        let interval4 = Interval::new(-10, 0);
        let interval5 = Interval::new(-10, -10);

        assert_eq!(interval1.width(), Some(10));
        assert_eq!(interval2.width(), Some(0));
        assert_eq!(interval3.width(), None);
        assert_eq!(interval4.width(), Some(10));
        assert_eq!(interval5.width(), Some(0));
    }
}
