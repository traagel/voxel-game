use macroquad::prelude::*;

const PLAINS_COLOR: Color = GREEN;
const MOUNTAIN_COLOR: Color = GRAY;
const OCEAN_COLOR: Color = BLUE;
const SEA_COLOR: Color = SKYBLUE;
const RIVER_COLOR: Color = DARKBLUE;
const BEACH_COLOR: Color = YELLOW;
const DESERT_COLOR: Color = ORANGE;
const FOREST_COLOR: Color = DARKGREEN;
const JUNGLE_COLOR: Color = LIME;
const TUNDRA_COLOR: Color = LIGHTGRAY;
const SWAMP_COLOR: Color = DARKPURPLE;
const LAKE_COLOR: Color = SKYBLUE;
const HILLS_COLOR: Color = BROWN;
const SNOW_COLOR: Color = WHITE;
const SAVANNA_COLOR: Color = GOLD;
const TAIGA_COLOR: Color = Color::new(0.5, 0.5, 0.0, 1.0);
const DEFAULT_COLOR: Color = WHITE;
const RAINFOREST_COLOR: Color = Color::new(0.0, 0.6, 0.0, 1.0);
const TEMPERATE_FOREST_COLOR: Color = Color::new(0.1, 0.7, 0.2, 1.0);
const BOREAL_FOREST_COLOR: Color = Color::new(0.2, 0.5, 0.3, 1.0);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TemperatureType { Freezing, Cold, Temperate, Warm, Hot }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VegetationType { None, Sparse, Grass, Shrubs, Forest, Jungle, Cacti }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrecipitationType { Arid, SemiArid, Moderate, Wet, Rainforest }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ElevationType { Ocean, Coast, Lowland, Hill, Mountain, Peak }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BiomeId {
    Ocean,
    Sea,
    Lake,
    River,
    Plains,
    Hills,
    Mountain,
    Snow,
    Forest,
    Jungle,
    Desert,
    Savanna,
    Tundra,
    Taiga,
    Swamp,
    Beach,
    TemperateForest,
    BorealForest,
    Rainforest,
}

impl TemperatureType {
    pub fn color(&self) -> Color {
        match self {
            TemperatureType::Freezing => BLUE,
            TemperatureType::Cold => SKYBLUE,
            TemperatureType::Temperate => GREEN,
            TemperatureType::Warm => ORANGE,
            TemperatureType::Hot => RED,
        }
    }
}

impl VegetationType {
    pub fn color(&self) -> Color {
        match self {
            VegetationType::None => LIGHTGRAY,
            VegetationType::Sparse => YELLOW,
            VegetationType::Grass => LIME,
            VegetationType::Shrubs => DARKGREEN,
            VegetationType::Forest => GREEN,
            VegetationType::Jungle => Color::new(0.0, 0.5, 0.0, 1.0),
            VegetationType::Cacti => Color::new(0.0, 0.7, 0.3, 1.0),
        }
    }
}

impl PrecipitationType {
    pub fn color(&self) -> Color {
        match self {
            PrecipitationType::Arid => ORANGE,
            PrecipitationType::SemiArid => GOLD,
            PrecipitationType::Moderate => GREEN,
            PrecipitationType::Wet => SKYBLUE,
            PrecipitationType::Rainforest => DARKGREEN,
        }
    }
}

impl ElevationType {
    pub fn color(&self) -> Color {
        match self {
            ElevationType::Ocean => BLUE,
            ElevationType::Coast => YELLOW,
            ElevationType::Lowland => LIME,
            ElevationType::Hill => BROWN,
            ElevationType::Mountain => GRAY,
            ElevationType::Peak => WHITE,
        }
    }
}

impl BiomeId {
    pub fn color(&self) -> Color {
        match self {
            BiomeId::Plains => PLAINS_COLOR,
            BiomeId::Mountain => MOUNTAIN_COLOR,
            BiomeId::Ocean => OCEAN_COLOR,
            BiomeId::Sea => SEA_COLOR,
            BiomeId::River => RIVER_COLOR,
            BiomeId::Beach => BEACH_COLOR,
            BiomeId::Desert => DESERT_COLOR,
            BiomeId::Forest => FOREST_COLOR,
            BiomeId::Rainforest => RAINFOREST_COLOR,
            BiomeId::TemperateForest => TEMPERATE_FOREST_COLOR,
            BiomeId::BorealForest => BOREAL_FOREST_COLOR,
            BiomeId::Tundra => TUNDRA_COLOR,
            BiomeId::Swamp => SWAMP_COLOR,
            BiomeId::Lake => LAKE_COLOR,
            BiomeId::Hills => HILLS_COLOR,
            BiomeId::Snow => SNOW_COLOR,
            BiomeId::Savanna => SAVANNA_COLOR,
            BiomeId::Taiga => TAIGA_COLOR,
            BiomeId::Jungle => JUNGLE_COLOR,
            _ => DEFAULT_COLOR,
        }
    }
}

// Category classification functions (stubs)
pub fn classify_temperature(temp: f64) -> TemperatureType {
    if temp < 0.15 {
        TemperatureType::Freezing
    } else if temp < 0.3 {
        TemperatureType::Cold
    } else if temp < 0.6 {
        TemperatureType::Temperate
    } else if temp < 0.85 {
        TemperatureType::Warm
    } else {
        TemperatureType::Hot
    }
}

pub fn classify_vegetation(veg: f64, temp: f64, precip: f64) -> VegetationType {
    // Placeholder logic, can be improved
    if veg < 0.1 {
        VegetationType::None
    } else if veg < 0.25 {
        if temp > 0.6 && precip < 0.3 {
            VegetationType::Cacti
        } else {
            VegetationType::Sparse
        }
    } else if veg < 0.4 {
        VegetationType::Grass
    } else if veg < 0.6 {
        VegetationType::Shrubs
    } else if veg < 0.8 {
        VegetationType::Forest
    } else {
        if temp > 0.7 && precip > 0.7 {
            VegetationType::Jungle
        } else {
            VegetationType::Forest
        }
    }
}

pub fn classify_precipitation(precip: f64) -> PrecipitationType {
    if precip < 0.15 {
        PrecipitationType::Arid
    } else if precip < 0.3 {
        PrecipitationType::SemiArid
    } else if precip < 0.6 {
        PrecipitationType::Moderate
    } else if precip < 0.85 {
        PrecipitationType::Wet
    } else {
        PrecipitationType::Rainforest
    }
}

pub fn classify_elevation(elev: f64, sea: f64, coast: f64, mountain: f64) -> ElevationType {
    if elev <= sea {
        ElevationType::Ocean
    } else if elev <= coast {
        ElevationType::Coast
    } else if elev < mountain {
        ElevationType::Lowland
    } else if elev < mountain + 0.03 {
        ElevationType::Hill
    } else if elev < mountain + 0.07 {
        ElevationType::Mountain
    } else {
        ElevationType::Peak
    }
} 