pub fn find_heavy_bottle(weights: &[f64]) -> usize {
    let mut actual_sum = 0.0;
    for (i, &w) in weights.iter().enumerate() {
        actual_sum += w * ((i + 1) as f64);
    }

    let expected_sum: f64 = (1..=weights.len()).map(|i| i as f64 * 1.0).sum();

    let diff = actual_sum - expected_sum;
    let bottle_number = (diff / 0.1).round() as usize;

    bottle_number - 1
}
