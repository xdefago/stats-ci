///
/// Defines an interval over a partially ordered type (NB: floating point numbers are partially ordered because of `NaN`).
/// The interval is defined by its lower and upper bounds.
///
/// # Examples
///
/// ```
/// # use stats_ci::Interval;
/// let interval = Interval::new(0., 1.);
/// assert_eq!(*interval.low().unwrap(), 0.);
/// assert_eq!(*interval.high().unwrap(), 1.);
/// assert!(!interval.is_empty());
/// assert!(!interval.is_degenerate());
/// assert!(interval.is_concrete());
/// assert!(interval.contains(&0.5));
/// assert!(!interval.contains(&2.));
/// ```
#[derive(Debug, Default, PartialEq)]
pub enum Interval<T: PartialOrd> {
    #[default]
    Empty,
    Degenerate(T),
    Concrete {
        low: T,
        high: T,
    },
}

impl<T: PartialOrd> Interval<T> {
    pub fn new(low: T, high: T) -> Self {
        if low > high {
            Interval::Empty
        } else if low == high {
            Interval::Degenerate(low)
        } else {
            Interval::Concrete { low, high }
        }
    }
    pub fn low(&self) -> Option<&T> {
        match self {
            Interval::Empty => None,
            Interval::Degenerate(x) => Some(x),
            Interval::Concrete { low, .. } => Some(low),
        }
    }
    pub fn high(&self) -> Option<&T> {
        match self {
            Interval::Empty => None,
            Interval::Degenerate(x) => Some(x),
            Interval::Concrete { high, .. } => Some(high),
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Interval::Empty => true,
            _ => false,
        }
    }
    pub fn is_degenerate(&self) -> bool {
        match self {
            Interval::Degenerate(_) => true,
            _ => false,
        }
    }
    pub fn is_concrete(&self) -> bool {
        match self {
            Interval::Concrete { .. } => true,
            _ => false,
        }
    }
    pub fn contains(&self, x: &T) -> bool {
        match self {
            Interval::Empty => false,
            Interval::Degenerate(y) => x == y,
            Interval::Concrete { low, high } => low <= x && x <= high,
        }
    }
    pub fn intersects(&self, other: &Self) -> bool {
        match (self, other) {
            (Interval::Empty, _) | (_, Interval::Empty) => false,
            (Interval::Degenerate(x), _) => other.contains(x),
            (_, Interval::Degenerate(y)) => self.contains(y),
            (Interval::Concrete { low: x, high: y }, Interval::Concrete { low: a, high: b }) => {
                x <= b && a <= y
            }
        }
    }
}

impl<T: Ord> From<(T, T)> for Interval<T> {
    fn from((low, high): (T, T)) -> Self {
        Interval::new(low, high)
    }
}

impl<T: Ord> From<(Option<T>, Option<T>)> for Interval<T> {
    fn from((low, high): (Option<T>, Option<T>)) -> Self {
        match (low, high) {
            (Some(low), Some(high)) => Interval::new(low, high),
            (Some(low), None) => Interval::Degenerate(low),
            (None, Some(high)) => Interval::Degenerate(high),
            (None, None) => Interval::Empty,
        }
    }
}

impl<T: Ord + Clone> From<Interval<T>> for (Option<T>, Option<T>) {
    fn from(interval: Interval<T>) -> Self {
        match interval {
            Interval::Empty => (None, None),
            Interval::Degenerate(x) => (Some(x.clone()), Some(x.clone())),
            Interval::Concrete { low, high } => (Some(low.clone()), Some(high.clone())),
        }
    }
}

impl<T: Ord + Clone> From<Interval<T>> for Option<(T, T)> {
    fn from(interval: Interval<T>) -> Self {
        match interval {
            Interval::Empty => None,
            Interval::Degenerate(x) => Some((x.clone(), x.clone())),
            Interval::Concrete { low, high } => Some((low.clone(), high.clone())),
        }
    }
}

use std::ops::Range;
use std::ops::RangeBounds;

impl<T: Ord> From<Range<T>> for Interval<T> {
    fn from(range: Range<T>) -> Self {
        Interval::new(range.start, range.end)
    }
}

impl<T: Ord> RangeBounds<T> for Interval<T> {
    fn start_bound(&self) -> std::ops::Bound<&T> {
        match self.low() {
            Some(low) => std::ops::Bound::Included(low),
            None => std::ops::Bound::Unbounded,
        }
    }
    fn end_bound(&self) -> std::ops::Bound<&T> {
        match self.high() {
            Some(high) => std::ops::Bound::Excluded(high),
            None => std::ops::Bound::Unbounded,
        }
    }
}
