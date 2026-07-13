pub fn average(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }

    values.iter().sum::<f64>() / values.len() as f64
}

pub fn minimum(values: &[f64]) -> f64 {
    values
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min)
}

pub fn maximum(values: &[f64]) -> f64 {
    values
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max)
}

pub fn std(values: &[f64]) -> f64 {
    if values.len() <= 1 {
        return 0.0;
    }

    let avg = average(values);

    let variance = values
        .iter()
        .map(|v| (v - avg).powi(2))
        .sum::<f64>()
        / values.len() as f64;

    variance.sqrt()
}