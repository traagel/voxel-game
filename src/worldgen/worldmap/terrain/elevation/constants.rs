//! Normalised thresholds and weights used by the elevation pass.

pub const NUM_CRATERS: usize = 5;

pub const CONTINENT_SCALE_FACTOR:   f64 = 1.5;
pub const DETAIL_JAGGED_FREQ:       f64 = 2.0;
pub const DETAIL_FALLOFF_FREQ:      f64 = 3.0;
pub const RIDGE_FREQ:               f64 = 2.0;
pub const PLATEAU_FREQ:             f64 = 0.7;
pub const CRATER_DEPTH:             f64 = 0.25;

pub const LAKE_FREQ:                f64 = 0.8;
pub const LAKE_LOWERING:            f64 = 0.18;
pub const LAKE_FLATTEN_BLEND:       f64 = 0.25;

pub const BASELINE_SHIFT:           f64 = -0.15;

pub const WEIGHT_CONTINENTAL_MASK:  f64 = 0.5;
pub const WEIGHT_DETAIL:            f64 = 0.15;
pub const WEIGHT_RIDGE:             f64 = 1.0;  // scaled later
pub const WEIGHT_FALLOFF:           f64 = 0.4;
pub const WEIGHT_PLATEAU:           f64 = 1.0;  // already toned inside fn
