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

impl<T> Interval<T> {
    pub fn new_unordered_unchecked(left: T, right: T) -> Self {
        Interval::Concrete { left, right }
    }
    pub fn is_empty(&self) -> bool {
        matches!(self, Interval::Empty)
    }
    pub fn is_degenerate(&self) -> bool {
        matches!(self, Interval::Degenerate(_))
    }
    pub fn is_concrete(&self) -> bool {
        matches!(self, Interval::Concrete { .. })
    }
}

impl<T> Interval<T> {
    pub fn left(&self) -> Option<&T> {
        match self {
            Interval::Empty => None,
            Interval::Degenerate(x) => Some(x),
            Interval::Concrete { left: low, .. } => Some(low),
        }
    }
    pub fn right(&self) -> Option<&T> {
        match self {
            Interval::Empty => None,
            Interval::Degenerate(x) => Some(x),
            Interval::Concrete { right: high, .. } => Some(high),
        }
    }
}

impl<T: PartialOrd + Copy> Interval<T> {
    pub fn low(&self) -> Option<T> {
        self.left().copied()
    }

    pub fn high(&self) -> Option<T> {
        self.right().copied()
    }
}
impl<T: PartialOrd> Interval<T> {
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

use std::ops::Range;
use std::ops::RangeBounds;

impl<T: PartialOrd> From<Range<T>> for Interval<T> {
    fn from(range: Range<T>) -> Self {
        Interval::new(range.start, range.end)
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
        let interval = Interval::from(0..3);
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
