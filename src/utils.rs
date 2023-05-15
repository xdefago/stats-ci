use num_traits::Float;

///
/// Kahan compensated summation register
///
/// This is a register that can be used to sum a sequence of floating point numbers with a better precision than a naive summation.
///
/// See <https://en.wikipedia.org/wiki/Kahan_summation_algorithm>
///
/// # Examples
///
/// ```ignore
/// let repetitions = 10_000;
/// let mut naive = 0.0_f32;
/// let mut sum = KahanSum::new(0.0_f32);
/// (1..=repetitions).for_each(|_| {
///     sum += 0.1;
///     naive += 0.1;
/// });
/// assert_eq!(sum.sum(), repetitions as f32 * 0.1);
/// assert_ne!(naive, repetitions as f32 * 0.1);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct KahanSum<T: Float> {
    sum: T,
    compensation: T,
}

impl<T: Float> KahanSum<T> {
    ///
    /// Create a new KahanSum register with the given initial value
    /// 
    /// # Arguments
    /// 
    /// * `value` - the initial value
    /// 
    pub fn new(value: T) -> Self {
        Self {
            sum: value,
            compensation: T::zero(),
        }
    }

    ///
    /// Return the current value of the sum
    /// 
    pub fn value(&self) -> T {
        self.sum + self.compensation
    }
}

impl<T: Float> Default for KahanSum<T> {
    fn default() -> Self {
        Self::new(T::zero())
    }
}

impl<T: Float> PartialEq for KahanSum<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl<T: Float + std::fmt::Display> std::fmt::Display for KahanSum<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value().fmt(f)
    }
}

impl<T: Float> std::ops::AddAssign<Self> for KahanSum<T> {
    fn add_assign(&mut self, rhs: Self) {
        kahan_add(&mut self.sum, rhs.sum, &mut self.compensation);
        kahan_add(&mut self.sum, rhs.compensation, &mut self.compensation);
    }
}

impl<T: Float> std::ops::AddAssign<T> for KahanSum<T> {
    fn add_assign(&mut self, rhs: T) {
        kahan_add(&mut self.sum, rhs, &mut self.compensation);
    }
}

impl<T: Float, X> std::ops::Add<X> for KahanSum<T>
where
    Self: std::ops::AddAssign<X>,
{
    type Output = Self;

    fn add(self, rhs: X) -> Self::Output {
        let mut sum = self;
        sum += rhs;
        sum
    }
}

impl<T: Float> From<T> for KahanSum<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

///
/// Compensated Kahan summation.
/// See <https://en.wikipedia.org/wiki/Kahan_summation_algorithm>
///
/// The function is meant to be called at each iteration of the summation,
/// with relevant variables managed externally
///
/// # Arguments
///
/// * `current_sum` - the current sum
/// * `x` - the next value to add to the sum
/// * `compensation` - the compensation term
///
pub(crate) fn kahan_add<T: Float>(current_sum: &mut T, x: T, compensation: &mut T) {
    let sum = *current_sum;
    let c = *compensation;
    let y = x - c;
    let t = sum + y;
    *compensation = (t - sum) - y;
    *current_sum = t;
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn test_kahan_add() {
        type Float = f32;
        let iterations = 50_000_000_usize;
        let mut normal: Float = 0.;
        let mut kahan: Float = 0.;
        let mut kahan_c: Float = 0.;
        let x = 1.1;

        for _ in 0..iterations {
            normal += x;
            kahan_add(&mut kahan, x, &mut kahan_c);
        }
        let expected = iterations as Float * x;
        println!("should be: {}", expected);
        println!(
            "normal: {} (diff: {:.0}%)",
            normal,
            (normal - expected) / expected * 100.
        );
        println!(
            "kahan: {} (diff: {:.0}%)",
            kahan,
            (kahan - expected) / expected * 100.
        );
        assert_abs_diff_eq!(expected, kahan, epsilon = 1e-10);
        assert!((expected - normal).abs() > 500_000.); // normal summation is not accurate for f32
    }

    #[test]
    fn test_kahan_sum() {
        type Float = f32;

        let iterations = 50_000_000_usize;
        let mut normal: Float = 0.;
        let mut kahan = KahanSum::<Float>::default();
        let mut kahan2 = KahanSum::<Float>::default();

        let x = 1.1;

        for i in 0..iterations {
            normal += x;
            kahan += x;
            if i % 2 == 1 {
                let mut double = KahanSum::<Float>::default();
                double += x;
                double += x;
                kahan2 += double;
            }
        }
        let expected = iterations as Float * x;
        println!("should be: {}", expected);
        println!(
            "normal: {} (diff: {:.0}%)",
            normal,
            (normal - expected) / expected * 100.
        );
        println!(
            "kahan: {} (diff: {:.0}%)",
            kahan,
            (kahan.value() - expected) / expected * 100.
        );
        println!(
            "kahan2: {} (diff: {:.0}%)",
            kahan2,
            (kahan2.value() - expected) / expected * 100.
        );
        assert_abs_diff_eq!(expected, kahan.value(), epsilon = 1e-10);
        assert_abs_diff_eq!(expected, kahan2.value(), epsilon = 1e-10);
        assert!((expected - normal).abs() > 500_000.); // normal summation is not accurate for f32
    }

    #[test]
    fn test_doctest() {
        let repetitions = 10_000;
        let mut naive = 0.0_f32;
        let mut sum = KahanSum::new(0.0_f32);
        (1..=repetitions).for_each(|_| {
            sum += 0.1;
            naive += 0.1;
        });
        assert_eq!(sum.value(), repetitions as f32 * 0.1);
        assert_ne!(naive, repetitions as f32 * 0.1);
    }
}
