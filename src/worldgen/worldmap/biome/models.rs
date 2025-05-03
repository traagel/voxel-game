#[derive(Copy, Clone)]
pub struct TileEnv {
    pub elev: f64,
    pub ridge: f64,
    pub moisture: f64,
    pub temp: f64,
    pub precip: f64,
    pub soil: f64,
    pub veg: f64,
    pub sea: f64,
    pub coast: f64,
    pub mountain: f64,
    pub river_here: bool,
    pub lake_here: bool,
}

