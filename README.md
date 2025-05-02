# voxel-game

A procedural voxel world map generator and renderer written in Rust using Macroquad.

## Features
- Procedural world generation with biomes, elevation, rivers, and civilizations
- Biome sprite rendering (PNG assets, see `assets/biome_sprite_map.json`)
- Civilization and city placement, trade routes
- Debug logging for world generation steps and timing
- Multiple map views: Biome, Temperature, Vegetation, Precipitation, Elevation, Civilization

## Getting Started

### Prerequisites
- Rust (https://rustup.rs/)

### Build and Run
```sh
cargo run --release
```

### Controls
- Zoom and pan with your mouse or keyboard (see in-game instructions if available)

## Assets
- Biome sprites are mapped in `assets/biome_sprite_map.json` and loaded from the `assets/` directory.

## Debugging
- World generation prints timing and statistics to the console for profiling.

## Dependencies
- [macroquad](https://github.com/not-fl3/macroquad) for rendering
- [serde](https://serde.rs/) for JSON parsing
- [noise](https://crates.io/crates/noise) for procedural generation

---
Feel free to contribute or open issues for suggestions and bug reports!
