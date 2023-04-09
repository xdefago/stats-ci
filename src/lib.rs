pub mod error;
mod interval;
pub mod mean;
pub mod proportion;
pub mod quantile;

pub use interval::Interval;

fn z_value(confidence: f64) -> f64 {
    assert!(confidence > 0. && confidence < 1.);
    use statrs::distribution::ContinuousCDF;
    use statrs::distribution::Normal;
    let alpha = 1. - confidence;
    let n = Normal::new(0., 1.).unwrap();
    n.inverse_cdf(1. - alpha / 2.)
}

fn t_value(confidence: f64, degrees_of_freedom: usize) -> f64 {
    assert!(confidence > 0. && confidence < 1.);
    use statrs::distribution::ContinuousCDF;
    use statrs::distribution::StudentsT;
    let alpha = 1. - confidence;
    let t = StudentsT::new(0., 1., degrees_of_freedom as f64).unwrap();
    t.inverse_cdf(1. - alpha / 2.)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_t_value() {
        for confidence in [0.5, 0.8, 0.9, 0.95, 0.99, 0.999] {
            let t_value = t_value(confidence, 1000);
            let z_value = z_value(confidence);
            assert_approx_eq!(t_value, z_value, 1e-2);
        }
    }
}
