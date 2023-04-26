use nu_ansi_term::Color;

#[allow(unused)]
pub fn highlight_color(observed: f64, expected: f64, tolerance: f64) -> Color {
    if observed < expected - tolerance {
        Color::LightRed
    } else if observed < expected {
        Color::LightYellow
    } else if observed <= expected + tolerance {
        Color::LightGreen
    } else {
        Color::Default
    }
}

#[allow(unused)]
pub fn color_larger_is_better(observed: f64, expected: f64, tolerance: f64) -> Color {
    if observed < expected - tolerance {
        Color::LightRed
    } else if observed < expected {
        Color::LightYellow
    } else if observed <= expected + tolerance {
        Color::LightGreen
    } else {
        Color::Default
    }
}

#[allow(unused)]
pub fn color_closer_is_better(observed: f64, expected: f64, tolerance: f64) -> Color {
    let abs_diff = (observed - expected).abs();
    if abs_diff < tolerance / 10. {
        Color::LightGreen
    } else if abs_diff < tolerance {
        Color::Default
    } else if abs_diff < tolerance * 10. {
        Color::LightYellow
    } else if abs_diff < tolerance * 100. {
        Color::Yellow
    } else {
        Color::LightRed
    }
}

pub fn color_smaller_is_better(observed: f64, expected: f64, tolerance: f64) -> Color {
    if observed < expected - tolerance {
        Color::Default
    } else if observed <= expected {
        Color::Green
    } else if observed <= expected + tolerance {
        Color::Yellow
    } else {
        Color::Red
    }
}
