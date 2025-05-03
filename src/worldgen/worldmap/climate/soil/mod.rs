pub fn make(
    elevation: &Vec<Vec<f64>>,
    precipitation: &Vec<Vec<f64>>,
    rivers: &Vec<Vec<bool>>,
) -> Vec<Vec<f64>> {
    let width = elevation.len();
    let height = if width > 0 { elevation[0].len() } else { 0 };
    let mut fertility = vec![vec![0.0; height]; width];
    for x in 0..width {
        for y in 0..height {
            let mut f = 0.5 * precipitation[x][y] + 0.3 * (1.0 - elevation[x][y]);
            if rivers[x][y] {
                f += 0.3;
            }
            fertility[x][y] = f.clamp(0.0, 1.0);
        }
    }
    fertility
}
