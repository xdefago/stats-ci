use num_traits::Float;

///
/// compensated Kahan summation.
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
    use assert_approx_eq::assert_approx_eq;

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
        assert_approx_eq!(expected, kahan, 1e-10);
        assert!((expected - normal).abs() > 500_000.); // normal summation is not accurate for f32
    }
}
