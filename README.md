# voxel-game

A fantasy world colony simulator and procedural world map generator written in Rust using Macroquad.

**Version: 0.1 alpha – World Generator Demo**

_Currently expanding on the tileset. Features animated water for ocean, river, and sea biomes._

## Features
- Animated water for ocean, river, and sea biomes (water tileset demo)
- Expanding and improving biome tileset (work in progress)
- Fantasy world simulation: manage a colony in a procedurally generated world
- Procedural world generation with biomes, elevation, rivers, civilizations, and cities
- Biome sprite rendering (PNG assets, see `assets/biome_sprite_map.json`)
- Colony management: dig, build, and interact with the world
- Creatures, particles, and local map simulation
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
- Switch between world map and local map with <kbd>Tab</kbd>
- Zoom and pan with your mouse or keyboard
- Dig and build using mouse buttons in local map mode

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
