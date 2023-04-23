use nu_ansi_term::Color;

pub fn highlight_color(hit_rate: f64, target: f64, tolerance: f64) -> Color {
    if hit_rate < target - tolerance {
        Color::LightRed
    } else if hit_rate < target {
        Color::LightYellow
    } else if hit_rate <= target + tolerance {
        Color::LightGreen
    } else {
        Color::Default
    }
}
