/// Normalised thresholds used throughout the classifier.
/// Keep gameplay tuning in one place.
// Temperature
pub const TEMP_FREEZING: f64 = 0.15;
pub const TEMP_COLD:     f64 = 0.30;
pub const TEMP_TEMPERATE:f64 = 0.60;
pub const TEMP_WARM:     f64 = 0.85;

// Vegetation
pub const VEG_NONE:      f64 = 0.10;
pub const VEG_SPARSE:    f64 = 0.25;
pub const VEG_GRASS:     f64 = 0.40;
pub const VEG_SHRUBS:    f64 = 0.60;
pub const VEG_FOREST:    f64 = 0.80;

// Precipitation
pub const PRECIP_ARID:   f64 = 0.15;
pub const PRECIP_SEMI:   f64 = 0.30;
pub const PRECIP_MOD:    f64 = 0.60;
pub const PRECIP_WET:    f64 = 0.85;

// Ridge & elevation offsets
pub const RIDGE_MOUNTAIN:f64 = 0.35;
pub const RIDGE_HILLS:   f64 = 0.20;
pub const HILL_OFFSET:   f64 = 0.02;
pub const PEAK_OFFSET:   f64 = 0.05; 