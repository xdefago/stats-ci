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
