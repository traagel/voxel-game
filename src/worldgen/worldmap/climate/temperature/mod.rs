pub fn make(elevation: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let width = elevation.len();
    let height = if width > 0 { elevation[0].len() } else { 0 };
    let mut temperature = vec![vec![0.0; height]; width];
    let equator = height as f64 / 2.0;
    for x in 0..width {
        for y in 0..height {
            let latitude = ((y as f64 - equator).abs()) / equator; // 0 at equator, 1 at poles
            let elev = elevation[x][y];
            // Base temp: hot at equator, cold at poles, colder at high elevation
            let base_temp = 1.0 - latitude - (elev * 0.5); // tweak as needed
            // Clamp to [0, 1] for normalized temperature
            temperature[x][y] = base_temp.clamp(0.0, 1.0);
        }
    }
    temperature
}
