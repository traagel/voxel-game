use super::generator::WorldGenerator;
use super::pipeline::GenStage;
use super::stages::height::HeightStage;
use noise::OpenSimplex;

pub struct WorldGeneratorBuilder {
    seed: u32,
    scale: f64,
}

impl WorldGeneratorBuilder {
    pub fn new(seed: u32) -> Self {
        Self { seed, scale: 0.005 }
    }

    pub fn with_scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_height_noise(self, _noise: OpenSimplex) -> Self {
        // placeholder – different noise types can be stored later
        self
    }

    pub fn build(self) -> WorldGenerator {
        // assemble stage list
        let stages: Vec<Box<dyn GenStage>> = vec![
            Box::new(HeightStage::new(self.seed, self.scale)),
            // later: more stages appended here
        ];
        WorldGenerator::from_stages(stages)
    }
}
