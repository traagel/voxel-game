use macroquad::prelude::*;
use crate::world::worldmap::world_map::WorldMap;
use crate::renderer::camera::Camera;
use crate::renderer::world_map_renderer::SpriteInfo;
use crate::world::worldmap::biome::BiomeId;
use std::collections::HashMap;
use crate::renderer::draw::draw_mountain_tile;
use crate::renderer::draw::draw_snow_mountain_tile;
use crate::renderer::draw::draw_sprite_tile;

pub fn draw_biome_tile(
    biome_sprite_map: &HashMap<String, SpriteInfo>,
    biome_textures: &HashMap<String, Texture2D>,
    world_map: &WorldMap,
    x: usize,
    y: usize,
    sx: f32,
    sy: f32,
    draw_size: f32,
    camera: &Camera,
    offset: f32,
) {
    let biome = world_map.biomes[x][y];
    if biome == BiomeId::Ocean || biome == BiomeId::River || biome == BiomeId::Sea {
        // Animated water logic with different speeds
        let frame_count = 30; // 480 / 16
        let frame_w = 16.0;
        let frame_h = 16.0;
        let anim_speed = match biome {
            BiomeId::Ocean => 3.0,   // slowest
            BiomeId::Sea   => 5.0,   // medium
            BiomeId::River => 7.0,   // fastest
            _ => 4.0,
        };
        let t = get_time() as f32;
        // Optionally offset by tile position for wavy effect
        let frame_idx = ((t * anim_speed + (x as f32 * 0.5) + (y as f32 * 0.3)) as usize) % frame_count;
        if let Some(tex) = biome_textures.get("water.png") {
            let src = Rect::new(frame_idx as f32 * frame_w, 0.0, frame_w, frame_h);
            draw_texture_ex(
                tex,
                sx,
                sy,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(draw_size, draw_size)),
                    source: Some(src),
                    ..Default::default()
                },
            );
            return;
        }
        // fallback to static sprite if water.png not loaded
        let key = match biome {
            BiomeId::Ocean => "Ocean",
            BiomeId::River => "River",
            BiomeId::Sea => "Sea",
            _ => unreachable!(),
        };
        if let Some(sprite) = biome_sprite_map.get(key) {
            if let Some(tex) = biome_textures.get(&sprite.filename) {
                draw_sprite_tile(sprite, tex, sx, sy, draw_size);
                return;
            }
        }
    } else if biome == BiomeId::Mountain {
        if draw_mountain_tile(biome_sprite_map, biome_textures, world_map, x, y, camera, draw_size, offset) {
            return;
        }
        if y + 1 < world_map.height && world_map.biomes[x][y + 1] == BiomeId::Mountain {
            if let Some(sprite) = biome_sprite_map.get("Mountain_A1") {
                if let Some(tex) = biome_textures.get(&sprite.filename) {
                    let tile_px = 16.0;
                    let col = if (x as isize) % 2 == 0 { 2.0 } else { 3.0 };
                    let src = Rect::new(col * tile_px, 0.0, tile_px, tile_px);
                    draw_texture_ex(
                        tex, sx, sy, WHITE,
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(draw_size, draw_size)),
                            source: Some(src), ..Default::default()
                        },
                    );
                    return;
                }
            }
        }
    } else if biome == BiomeId::Snow {
        if draw_snow_mountain_tile(biome_sprite_map, biome_textures, world_map, x, y, camera, draw_size, offset) {
            return;
        }
        if y + 1 < world_map.height && world_map.biomes[x][y + 1] == BiomeId::Snow {
            if let Some(sprite) = biome_sprite_map.get("Mountain_A2") {
                if let Some(tex) = biome_textures.get(&sprite.filename) {
                    let tile_px = 16.0;
                    let col = if (x as isize) % 2 == 0 { 2.0 } else { 3.0 };
                    let src = Rect::new(col * tile_px, 0.0, tile_px, tile_px);
                    draw_texture_ex(
                        tex, sx, sy, WHITE,
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(draw_size, draw_size)),
                            source: Some(src), ..Default::default()
                        },
                    );
                    return;
                }
            }
        }
    } else if let Some(key) = crate::renderer::world_map_renderer::biome_id_to_sprite_key(biome) {
        if let Some(sprite) = biome_sprite_map.get(key) {
            if let Some(tex) = biome_textures.get(&sprite.filename) {
                draw_sprite_tile(sprite, tex, sx, sy, draw_size);
                return;
            }
        }
    }
    let color = biome.color();
    draw_rectangle(sx, sy, draw_size, draw_size, color);
} 