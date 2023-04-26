// Tests to compare against test cases generated from numpy and stored in toml format
// The python generation is done by tests/cases/generate_cases.py
// The toml files are stored in tests/cases/
use assert_approx_eq::assert_approx_eq;
use serde::Deserialize;
use stats_ci::*;
use std::path::Path;
use toml;

mod common;

#[derive(Deserialize, Debug, Clone)]
struct TestCase {
    //    name: String,
    size: usize,
    level: f64,
    case: usize,
    ci_low: f64,
    ci_high: f64,
    data: Vec<f64>,
}

#[test]
fn test_toml() {
    for file in std::fs::read_dir("tests/cases").unwrap() {
        if let Ok(case_file) = file.map(|x| x.path()) {
            if lazy_regex::regex_is_match!(
                r"^case_.*\.toml$",
                case_file.file_name().unwrap().to_str().unwrap()
            ) {
                test_case(&case_file).unwrap();
            } else {
                println!();
            }
        }
    }
}

fn test_case(case_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let toml_string = std::fs::read_to_string(case_file)?;
    let case: TestCase = toml::from_str(&toml_string)?;
    let reference_ci = Interval::new(case.ci_low, case.ci_high)?;

    let confidence = Confidence::new_two_sided(case.level);
    let computed_ci = mean::Arithmetic::ci(confidence, case.data.into_iter())?;

    let diff_low = (computed_ci.low_f() - reference_ci.low_f()).abs();
    let diff_high = (computed_ci.high_f() - reference_ci.high_f()).abs();

    println!(
        ">> size {:>7} level {:>4.1}% no.{:2} {},  {}",
        case.size,
        case.level,
        case.case,
        common::color_closer_is_better(diff_low, 0., 1e-9)
            .paint(format!("Δlow: {:3.10}", diff_low)),
        common::color_closer_is_better(diff_high, 0., 1e-9)
            .paint(format!("Δhigh: {:3.10}", diff_high))
    );
    assert_approx_eq!(diff_low, 0., 1e-4);
    assert_approx_eq!(diff_high, 0., 1e-4);
    Ok(())
}
