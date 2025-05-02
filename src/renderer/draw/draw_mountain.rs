use macroquad::prelude::*;
use crate::{
    renderer::{camera::Camera, world_map_renderer::SpriteInfo},
    world::worldmap::{biome::BiomeId, world_map::WorldMap},
};
use std::collections::HashMap;

#[inline]
fn frame(sprite: &SpriteInfo, col: f32, row: f32) -> Rect {
    Rect::new(
        sprite.x as f32 + col * 16.0,
        sprite.y as f32 + row * 16.0,
        16.0,
        16.0,
    )
}

#[inline]
fn screen(x: isize, y: isize, cam: &Camera, draw: f32, off: f32) -> Vec2 {
    Vec2::new(
        (x as f32 - cam.x) * (draw + off) + off,
        (y as f32 - cam.y) * (draw + off) + off,
    )
}

/// Draw a mountain tile, returning `true` if something was rendered.
pub fn draw_mountain_tile(
    sprites: &HashMap<String, SpriteInfo>,
    texs:   &HashMap<String, Texture2D>,
    map:    &WorldMap,
    x: usize,
    y: usize,
    cam:  &Camera,
    draw: f32,
    off:  f32,
) -> bool {
    if map.biomes[x][y] != BiomeId::Mountain { return false; }

    let Some(sprite) = sprites.get("Mountain_A1") else { return false; };
    let Some(tex)    = texs.get(&sprite.filename)  else { return false; };

    let (w, h)  = (map.width as isize, map.height as isize);
    let (xi, yi) = (x as isize, y as isize);

    let mtn = |dx: isize, dy: isize| {
        let nx = xi + dx;
        let ny = yi + dy;
        nx >= 0 && ny >= 0 && nx < w && ny < h &&
        map.biomes[nx as usize][ny as usize] == BiomeId::Mountain
    };

    let col_parity = if xi % 2 == 0 { 2.0 } else { 3.0 };
    let row_parity = if yi % 2 == 0 { 2.0 } else { 3.0 };

    // ── WEST edge ──
    if !mtn(-1, 0) {
        draw_texture_ex(
            tex,
            screen(xi - 1, yi, cam, draw, off).x,
            screen(xi, yi, cam, draw, off).y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(draw, draw)),
                source:    Some(frame(sprite, 0.0, row_parity)),
                ..Default::default()
            },
        );
        draw_texture_ex(
            tex,
            screen(xi, yi, cam, draw, off).x,
            screen(xi, yi, cam, draw, off).y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(draw, draw)),
                source:    Some(frame(sprite, 1.0, row_parity)),
                ..Default::default()
            },
        );
    }

    // ── EAST edge ──
    if !mtn(1, 0) {
        draw_texture_ex(
            tex,
            screen(xi, yi, cam, draw, off).x,
            screen(xi, yi, cam, draw, off).y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(draw, draw)),
                source:    Some(frame(sprite, 4.0, row_parity)),
                ..Default::default()
            },
        );
        draw_texture_ex(
            tex,
            screen(xi + 1, yi, cam, draw, off).x,
            screen(xi,     yi, cam, draw, off).y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(draw, draw)),
                source:    Some(frame(sprite, 5.0, row_parity)),
                ..Default::default()
            },
        );
    }

    // ── SOUTH edge ──
    if !mtn(0, 1) {
        // Only draw skirt (row 3) at the current tile for debugging
        draw_texture_ex(
            tex,
            screen(xi, yi, cam, draw, off).x,
            screen(xi, yi, cam, draw, off).y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(draw, draw)),
                source:    Some(frame(sprite, col_parity, 4.0)),
                ..Default::default()
            },
        );
    }

    // ── NORTH edge (row 1) ──
    if !mtn(0, -1) {
        draw_texture_ex(
            tex,
            screen(xi, yi, cam, draw, off).x,
            screen(xi, yi, cam, draw, off).y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(draw, draw)),
                source:    Some(frame(sprite, col_parity, 1.0)),
                ..Default::default()
            },
        );
    }
    let no_edges = mtn(-1,0) && mtn(1,0) && mtn(0,-1) && mtn(0,1);
    if no_edges {
        draw_texture_ex(
            tex,
            screen(xi, yi, cam, draw, off).x,
            screen(xi, yi, cam, draw, off).y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(draw, draw)),
                source:    Some(frame(sprite, col_parity, row_parity)), // body sprite
                ..Default::default()
            },
        );
    }


    // ── OVERLAY CAP (row 0) – always last, shifted up one tile ──
    let cap_pos = screen(xi, yi - 1, cam, draw, off);
    draw_texture_ex(
        tex,
        cap_pos.x,
        cap_pos.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(draw, draw)),
            source:    Some(frame(sprite, col_parity, 0.0)),
            ..Default::default()
        },
    );

    // Note: The skirt (bottom edge) is drawn at the tile below the current one (yi + 1),
    // but only if that tile is not a mountain. This is correct for per-tile rendering.

    true
}

/// Draw a snow mountain tile, returning `true` if something was rendered.
pub fn draw_snow_mountain_tile(
    sprites: &HashMap<String, SpriteInfo>,
    texs:   &HashMap<String, Texture2D>,
    map:    &WorldMap,
    x: usize,
    y: usize,
    cam:  &Camera,
    draw: f32,
    off:  f32,
) -> bool {
    if map.biomes[x][y] != BiomeId::Snow {
        return false;
    }

    let Some(sprite) = sprites.get("Mountain_A2") else {
        return false;
    };
    let Some(tex)    = texs.get(&sprite.filename)  else {
        return false;
    };

    let (w, h)  = (map.width as isize, map.height as isize);
    let (xi, yi) = (x as isize, y as isize);

    let mtn = |dx: isize, dy: isize| {
        let nx = xi + dx;
        let ny = yi + dy;
        nx >= 0 && ny >= 0 && nx < w && ny < h &&
        map.biomes[nx as usize][ny as usize] == BiomeId::Snow
    };

    let col_parity = if xi % 2 == 0 { 2.0 } else { 3.0 };
    let row_parity = if yi % 2 == 0 { 2.0 } else { 3.0 };

    // ── WEST edge ──
    if !mtn(-1, 0) {
        draw_texture_ex(
            tex,
            screen(xi - 1, yi, cam, draw, off).x,
            screen(xi, yi, cam, draw, off).y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(draw, draw)),
                source:    Some(frame(sprite, 0.0, row_parity)),
                ..Default::default()
            },
        );
        draw_texture_ex(
            tex,
            screen(xi, yi, cam, draw, off).x,
            screen(xi, yi, cam, draw, off).y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(draw, draw)),
                source:    Some(frame(sprite, 1.0, row_parity)),
                ..Default::default()
            },
        );
    }

    // ── EAST edge ──
    if !mtn(1, 0) {
        draw_texture_ex(
            tex,
            screen(xi, yi, cam, draw, off).x,
            screen(xi, yi, cam, draw, off).y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(draw, draw)),
                source:    Some(frame(sprite, 4.0, row_parity)),
                ..Default::default()
            },
        );
        draw_texture_ex(
            tex,
            screen(xi + 1, yi, cam, draw, off).x,
            screen(xi,     yi, cam, draw, off).y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(draw, draw)),
                source:    Some(frame(sprite, 5.0, row_parity)),
                ..Default::default()
            },
        );
    }

    // ── SOUTH edge ──
    if !mtn(0, 1) {
        // Only draw skirt (row 3) at the current tile for debugging
        draw_texture_ex(
            tex,
            screen(xi, yi, cam, draw, off).x,
            screen(xi, yi, cam, draw, off).y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(draw, draw)),
                source:    Some(frame(sprite, col_parity, 4.0)),
                ..Default::default()
            },
        );
    }

    // ── NORTH edge (row 1) ──
    if !mtn(0, -1) {
        draw_texture_ex(
            tex,
            screen(xi, yi, cam, draw, off).x,
            screen(xi, yi, cam, draw, off).y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(draw, draw)),
                source:    Some(frame(sprite, col_parity, 1.0)),
                ..Default::default()
            },
        );
    }
    let no_edges = mtn(-1,0) && mtn(1,0) && mtn(0,-1) && mtn(0,1);
    if no_edges {
        draw_texture_ex(
            tex,
            screen(xi, yi, cam, draw, off).x,
            screen(xi, yi, cam, draw, off).y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(draw, draw)),
                source:    Some(frame(sprite, col_parity, row_parity)), // body sprite
                ..Default::default()
            },
        );
    }

    // ── OVERLAY CAP (row 0) – always last, shifted up one tile ──
    let cap_pos = screen(xi, yi - 1, cam, draw, off);
    draw_texture_ex(
        tex,
        cap_pos.x,
        cap_pos.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(draw, draw)),
            source:    Some(frame(sprite, col_parity, 0.0)),
            ..Default::default()
        },
    );

    true
}
