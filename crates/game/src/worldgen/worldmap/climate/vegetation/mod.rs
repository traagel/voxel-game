pub fn make(
    temperature: &Vec<Vec<f64>>,
    precipitation: &Vec<Vec<f64>>,
    soil_fertility: &Vec<Vec<f64>>,
) -> Vec<Vec<f64>> {
    let width = temperature.len();
    let height = if width > 0 { temperature[0].len() } else { 0 };
    let mut vegetation = vec![vec![0.0; height]; width];
    for x in 0..width {
        for y in 0..height {
            let v = 0.4 * temperature[x][y] + 0.4 * precipitation[x][y] + 0.2 * soil_fertility[x][y];
            vegetation[x][y] = v.clamp(0.0, 1.0);
        }
    }
    vegetation
}
