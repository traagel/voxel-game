use macroquad::prelude::*;
use crate::renderer::world_map_renderer::SpriteInfo;

pub fn draw_sprite_tile(sprite: &SpriteInfo, tex: &Texture2D, sx: f32, sy: f32, draw_size: f32) {
    let src = Rect::new(sprite.x as f32, sprite.y as f32, sprite.width as f32, sprite.height as f32);
    let sprite_w = sprite.width as f32;
    let sprite_h = sprite.height as f32;
    let scale = (draw_size / sprite_w).min(draw_size / sprite_h);
    let dest_w = sprite_w * scale;
    let dest_h = sprite_h * scale;
    let dest_x = sx + (draw_size - dest_w) / 2.0;
    let dest_y = sy + (draw_size - dest_h) / 2.0;
    draw_texture_ex(
        tex,
        dest_x,
        dest_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(dest_w, dest_h)),
            source: Some(src),
            ..Default::default()
        },
    );
} 