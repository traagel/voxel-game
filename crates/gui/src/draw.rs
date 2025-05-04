use crate::{GuiContext, Widget};
use crate::widgets::{Button, Label, Slider, Checkbox};
use macroquad::prelude::*;

pub fn draw_gui(ctx: &GuiContext) {
    for window in ctx.windows.values().filter(|w| w.visible) {
        // Draw window background
        draw_ninepatch(&window.rect, &ctx.theme.window_skin, 16.0, &ctx.theme.tilemap);

        // Draw title bar text (centered horizontally)
        let title_font_size = 20;
        let title_dimensions = measure_text(&window.title, ctx.theme.font_default.as_ref(), title_font_size, 1.0);
        draw_text_ex(&window.title,
            window.rect.x + (window.rect.w - title_dimensions.width) / 2.0, // Center horizontally
            window.rect.y + 20.0, // Keep the same vertical position
            TextParams {
                font: ctx.theme.font_default.as_ref(),
                font_size: title_font_size,
                color: ctx.theme.color_text,
                ..Default::default()
            });

        // Draw widgets
        for widget in &window.widgets {
            match widget {
                Widget::Label(l) => draw_label(l, ctx),
                Widget::Button(b) => draw_button(b, ctx),
                Widget::Image(i) => draw_texture(&i.tex, i.rect.x, i.rect.y, WHITE),
                Widget::Checkbox(c) => draw_checkbox(c, ctx),
                Widget::Slider(s) => draw_slider(s, ctx),
            }
        }
    }
}

// Helper function to calculate text position based on alignment
fn calculate_text_position(
    text: &str,
    x: f32,
    y: f32,
    font: Option<&Font>,
    font_size: u16,
    alignment: crate::widgets::TextAlign
) -> (f32, f32) {
    let text_dimensions = measure_text(text, font, font_size, 1.0);
    
    let x_pos = match alignment {
        crate::widgets::TextAlign::Left => x,
        crate::widgets::TextAlign::Center => x - text_dimensions.width / 2.0,
        crate::widgets::TextAlign::Right => x - text_dimensions.width,
    };
    
    (x_pos, y + text_dimensions.offset_y)
}

pub fn draw_label(label: &crate::Label, ctx: &GuiContext) {
    // Calculate text position based on alignment
    let (x, y) = calculate_text_position(
        &label.text,
        label.rect.x,
        label.rect.y,
        ctx.theme.font_default.as_ref(),
        label.font_size as u16,
        label.alignment
    );
    
    // Draw label text with proper alignment
    draw_text_ex(
        &label.text,
        x,
        y,
        TextParams {
            font: ctx.theme.font_default.as_ref(),
            font_size: label.font_size as u16,
            color: ctx.theme.color_text,
            ..Default::default()
        }
    );
}

pub fn draw_button(button: &Button, ctx: &GuiContext) {
    // Choose correct button state texture
    let mut color = WHITE;
    if button.clicked {
        color = GRAY;
    } else if button.hovered {
        color = Color::new(1.0, 1.0, 1.0, 0.8);
    }

    // If we have a tilemap, use it for buttons
    if let Some(tilemap) = &ctx.theme.tilemap {
        // Get button tile position based on state
        let button_tile = if button.clicked {
            tilemap.tiles.get("button_pressed").unwrap_or(&crate::theme::TilePosition { x: 3, y: 1 })
        } else if button.hovered {
            tilemap.tiles.get("button_hover").unwrap_or(&crate::theme::TilePosition { x: 3, y: 0 })
        } else {
            tilemap.tiles.get("button").unwrap_or(&crate::theme::TilePosition { x: 3, y: 0 })
        };
        
        let tile_width = tilemap.tile_size.width as f32;
        let tile_height = tilemap.tile_size.height as f32;
        
        // Draw the button as a stretched tile
        draw_texture_ex(
            &ctx.theme.window_skin,
            button.rect.x,
            button.rect.y,
            color,
            DrawTextureParams {
                dest_size: Some(Vec2::new(button.rect.w, button.rect.h)),
                source: Some(Rect::new(
                    button_tile.x as f32 * tile_width,
                    button_tile.y as f32 * tile_height,
                    tile_width,
                    tile_height
                )),
                ..Default::default()
            }
        );
    } else {
        // Fallback to simple button rendering
        draw_rectangle(button.rect.x, button.rect.y, button.rect.w, button.rect.h, 
            Color::new(0.5, 0.5, 0.5, 0.9));
        draw_rectangle_lines(button.rect.x, button.rect.y, button.rect.w, button.rect.h, 1.0, color);
    }

    // Draw button text if any
    if !button.text.is_empty() {
        // Calculate center position for text
        let center_x = button.rect.x + button.rect.w / 2.0;
        let center_y = button.rect.y + button.rect.h / 2.0;
        
        // Get text position with center alignment
        let (x, y) = calculate_text_position(
            &button.text,
            center_x,
            center_y,
            ctx.theme.font_default.as_ref(),
            20,
            crate::widgets::TextAlign::Center
        );

        draw_text_ex(&button.text, x, y, TextParams {
            font: ctx.theme.font_default.as_ref(),
            font_size: 20,
            color: ctx.theme.color_text,
            ..Default::default()
        });
    }
}

pub fn draw_checkbox(checkbox: &Checkbox, ctx: &GuiContext) {
    let checkbox_size = Vec2::new(20.0, 20.0);
    
    // If we have a tilemap, use it for checkboxes
    if let Some(tilemap) = &ctx.theme.tilemap {
        // Get checkbox tile position based on state
        let checkbox_tile = if checkbox.checked {
            tilemap.tiles.get("checkbox_checked").unwrap_or(&crate::theme::TilePosition { x: 4, y: 1 })
        } else {
            tilemap.tiles.get("checkbox").unwrap_or(&crate::theme::TilePosition { x: 4, y: 0 })
        };
        
        let tile_width = tilemap.tile_size.width as f32;
        let tile_height = tilemap.tile_size.height as f32;
        
        // Draw the checkbox
        draw_texture_ex(
            &ctx.theme.window_skin,
            checkbox.rect.x,
            checkbox.rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(checkbox_size),
                source: Some(Rect::new(
                    checkbox_tile.x as f32 * tile_width,
                    checkbox_tile.y as f32 * tile_height,
                    tile_width,
                    tile_height
                )),
                ..Default::default()
            }
        );
    } else {
        // Fallback to simple checkbox rendering
        draw_rectangle(
            checkbox.rect.x, 
            checkbox.rect.y, 
            checkbox_size.x, 
            checkbox_size.y, 
            Color::new(0.2, 0.2, 0.2, 1.0)
        );
        
        draw_rectangle_lines(
            checkbox.rect.x, 
            checkbox.rect.y, 
            checkbox_size.x, 
            checkbox_size.y, 
            1.0, 
            WHITE
        );
        
        // Draw check mark if checked
        if checkbox.checked {
            draw_line(
                checkbox.rect.x + 2.0, 
                checkbox.rect.y + 10.0, 
                checkbox.rect.x + 8.0, 
                checkbox.rect.y + 16.0, 
                2.0, 
                GREEN
            );
            
            draw_line(
                checkbox.rect.x + 8.0, 
                checkbox.rect.y + 16.0, 
                checkbox.rect.x + 18.0, 
                checkbox.rect.y + 4.0, 
                2.0, 
                GREEN
            );
        }
    }
    
    // Draw checkbox label with proper vertical alignment
    if !checkbox.label.is_empty() {
        let text_dimensions = measure_text(&checkbox.label, ctx.theme.font_default.as_ref(), 20, 1.0);
        
        draw_text_ex(
            &checkbox.label,
            checkbox.rect.x + checkbox_size.x + 10.0,
            checkbox.rect.y + (checkbox_size.y / 2.0) + (text_dimensions.height / 4.0), // Vertically center with checkbox
            TextParams {
                font: ctx.theme.font_default.as_ref(),
                font_size: 20,
                color: ctx.theme.color_text,
                ..Default::default()
            }
        );
    }
}

pub fn draw_slider(slider: &Slider, ctx: &GuiContext) {
    let track_height = 8.0;
    let handle_size = Vec2::new(16.0, 20.0);
    
    // Calculate handle position based on value
    let normalized_value = (slider.value - slider.min) / (slider.max - slider.min);
    let handle_x = slider.rect.x + (slider.rect.w - handle_size.x) * normalized_value;
    let track_y = slider.rect.y + (slider.rect.h - track_height) / 2.0;
    
    // If we have a tilemap, use it for sliders
    if let Some(tilemap) = &ctx.theme.tilemap {
        // Get tile positions
        let slider_track = tilemap.tiles.get("slider_track").unwrap_or(&crate::theme::TilePosition { x: 5, y: 0 });
        let slider_handle = tilemap.tiles.get("slider_handle").unwrap_or(&crate::theme::TilePosition { x: 5, y: 1 });
        
        let tile_width = tilemap.tile_size.width as f32;
        let tile_height = tilemap.tile_size.height as f32;
        
        // Draw the slider track
        draw_texture_ex(
            &ctx.theme.window_skin,
            slider.rect.x,
            track_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(slider.rect.w, track_height)),
                source: Some(Rect::new(
                    slider_track.x as f32 * tile_width,
                    slider_track.y as f32 * tile_height,
                    tile_width,
                    tile_height
                )),
                ..Default::default()
            }
        );
        
        // Draw the slider handle
        draw_texture_ex(
            &ctx.theme.window_skin,
            handle_x,
            slider.rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(handle_size),
                source: Some(Rect::new(
                    slider_handle.x as f32 * tile_width,
                    slider_handle.y as f32 * tile_height,
                    tile_width,
                    tile_height
                )),
                ..Default::default()
            }
        );
    } else {
        // Fallback to simple slider rendering
        // Draw track (background)
        draw_rectangle(
            slider.rect.x, 
            track_y, 
            slider.rect.w, 
            track_height, 
            Color::new(0.2, 0.2, 0.2, 1.0)
        );
        
        // Draw filled portion
        draw_rectangle(
            slider.rect.x, 
            track_y, 
            handle_x - slider.rect.x + handle_size.x/2.0, 
            track_height, 
            Color::new(0.3, 0.5, 0.7, 1.0)
        );
        
        // Draw handle
        draw_rectangle(
            handle_x, 
            slider.rect.y, 
            handle_size.x, 
            handle_size.y, 
            Color::new(0.7, 0.7, 0.7, 1.0)
        );
        
        draw_rectangle_lines(
            handle_x, 
            slider.rect.y, 
            handle_size.x, 
            handle_size.y, 
            1.0, 
            WHITE
        );
    }
    
    // Draw value text with proper alignment
    let value_text = format!("{:.1}", slider.value);
    let text_dimensions = measure_text(&value_text, ctx.theme.font_default.as_ref(), 18, 1.0);
    
    draw_text_ex(
        &value_text,
        slider.rect.x + slider.rect.w + 10.0,
        slider.rect.y + (slider.rect.h / 2.0) + (text_dimensions.height / 4.0), // Vertically center
        TextParams {
            font: ctx.theme.font_default.as_ref(),
            font_size: 18,
            color: ctx.theme.color_text,
            ..Default::default()
        }
    );
}

// Enhanced nine-patch helper - renders a texture as a stretchable panel
pub fn draw_ninepatch(dst: &Rect, tex: &Texture2D, patch: f32, tilemap: &Option<crate::theme::TileMap>) {
    // Use tilemap if available, otherwise use default algorithm
    if let Some(map) = tilemap {
        // Get the tile size from the tilemap
        let tile_width = map.tile_size.width as f32;
        let tile_height = map.tile_size.height as f32;
        
        // Get positions for the 9 tiles
        let top_left = map.tiles.get("top_left").unwrap_or(&crate::theme::TilePosition { x: 0, y: 0 });
        let top = map.tiles.get("top").unwrap_or(&crate::theme::TilePosition { x: 1, y: 0 });
        let top_right = map.tiles.get("top_right").unwrap_or(&crate::theme::TilePosition { x: 2, y: 0 });
        let left = map.tiles.get("left").unwrap_or(&crate::theme::TilePosition { x: 0, y: 1 });
        let center = map.tiles.get("center").unwrap_or(&crate::theme::TilePosition { x: 1, y: 1 });
        let right = map.tiles.get("right").unwrap_or(&crate::theme::TilePosition { x: 2, y: 1 });
        let bottom_left = map.tiles.get("bottom_left").unwrap_or(&crate::theme::TilePosition { x: 0, y: 2 });
        let bottom = map.tiles.get("bottom").unwrap_or(&crate::theme::TilePosition { x: 1, y: 2 });
        let bottom_right = map.tiles.get("bottom_right").unwrap_or(&crate::theme::TilePosition { x: 2, y: 2 });
        
        // Calculate sizes
        let border_size = Vec2::new(tile_width, tile_height);
        let center_size = Vec2::new(dst.w - (tile_width * 2.0), dst.h - (tile_height * 2.0));
        
        // Draw the 9 tiles
        // Top-left corner
        draw_texture_ex(
            tex,
            dst.x,
            dst.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(border_size),
                source: Some(Rect::new(
                    top_left.x as f32 * tile_width,
                    top_left.y as f32 * tile_height,
                    tile_width,
                    tile_height
                )),
                ..Default::default()
            }
        );
        
        // Top edge
        draw_texture_ex(
            tex,
            dst.x + tile_width,
            dst.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(center_size.x, border_size.y)),
                source: Some(Rect::new(
                    top.x as f32 * tile_width,
                    top.y as f32 * tile_height,
                    tile_width,
                    tile_height
                )),
                ..Default::default()
            }
        );
        
        // Top-right corner
        draw_texture_ex(
            tex,
            dst.x + dst.w - tile_width,
            dst.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(border_size),
                source: Some(Rect::new(
                    top_right.x as f32 * tile_width,
                    top_right.y as f32 * tile_height,
                    tile_width,
                    tile_height
                )),
                ..Default::default()
            }
        );
        
        // Left edge
        draw_texture_ex(
            tex,
            dst.x,
            dst.y + tile_height,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(border_size.x, center_size.y)),
                source: Some(Rect::new(
                    left.x as f32 * tile_width,
                    left.y as f32 * tile_height,
                    tile_width,
                    tile_height
                )),
                ..Default::default()
            }
        );
        
        // Center
        draw_texture_ex(
            tex,
            dst.x + tile_width,
            dst.y + tile_height,
            WHITE,
            DrawTextureParams {
                dest_size: Some(center_size),
                source: Some(Rect::new(
                    center.x as f32 * tile_width,
                    center.y as f32 * tile_height,
                    tile_width,
                    tile_height
                )),
                ..Default::default()
            }
        );
        
        // Right edge
        draw_texture_ex(
            tex,
            dst.x + dst.w - tile_width,
            dst.y + tile_height,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(border_size.x, center_size.y)),
                source: Some(Rect::new(
                    right.x as f32 * tile_width,
                    right.y as f32 * tile_height,
                    tile_width,
                    tile_height
                )),
                ..Default::default()
            }
        );
        
        // Bottom-left corner
        draw_texture_ex(
            tex,
            dst.x,
            dst.y + dst.h - tile_height,
            WHITE,
            DrawTextureParams {
                dest_size: Some(border_size),
                source: Some(Rect::new(
                    bottom_left.x as f32 * tile_width,
                    bottom_left.y as f32 * tile_height,
                    tile_width,
                    tile_height
                )),
                ..Default::default()
            }
        );
        
        // Bottom edge
        draw_texture_ex(
            tex,
            dst.x + tile_width,
            dst.y + dst.h - tile_height,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(center_size.x, border_size.y)),
                source: Some(Rect::new(
                    bottom.x as f32 * tile_width,
                    bottom.y as f32 * tile_height,
                    tile_width,
                    tile_height
                )),
                ..Default::default()
            }
        );
        
        // Bottom-right corner
        draw_texture_ex(
            tex,
            dst.x + dst.w - tile_width,
            dst.y + dst.h - tile_height,
            WHITE,
            DrawTextureParams {
                dest_size: Some(border_size),
                source: Some(Rect::new(
                    bottom_right.x as f32 * tile_width,
                    bottom_right.y as f32 * tile_height,
                    tile_width,
                    tile_height
                )),
                ..Default::default()
            }
        );
    } else {
        // If no tilemap, use the original naive stretch approach
        let src_width = tex.width();
        let src_height = tex.height();
        
        // Calculate dimensions of patches
        let patch_size = vec2(patch, patch);
        let center_size = vec2(dst.w - patch * 2.0, dst.h - patch * 2.0);
        
        // Top-left corner
        draw_texture_ex(
            tex,
            dst.x,
            dst.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(patch_size),
                source: Some(Rect::new(0.0, 0.0, patch, patch)),
                ..Default::default()
            }
        );
        
        // Top edge
        draw_texture_ex(
            tex,
            dst.x + patch,
            dst.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(center_size.x, patch_size.y)),
                source: Some(Rect::new(patch, 0.0, src_width - patch * 2.0, patch)),
                ..Default::default()
            }
        );
        
        // Top-right corner
        draw_texture_ex(
            tex,
            dst.x + dst.w - patch,
            dst.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(patch_size),
                source: Some(Rect::new(src_width - patch, 0.0, patch, patch)),
                ..Default::default()
            }
        );
        
        // Left edge
        draw_texture_ex(
            tex,
            dst.x,
            dst.y + patch,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(patch_size.x, center_size.y)),
                source: Some(Rect::new(0.0, patch, patch, src_height - patch * 2.0)),
                ..Default::default()
            }
        );
        
        // Center
        draw_texture_ex(
            tex,
            dst.x + patch,
            dst.y + patch,
            WHITE,
            DrawTextureParams {
                dest_size: Some(center_size),
                source: Some(Rect::new(patch, patch, src_width - patch * 2.0, src_height - patch * 2.0)),
                ..Default::default()
            }
        );
        
        // Right edge
        draw_texture_ex(
            tex,
            dst.x + dst.w - patch,
            dst.y + patch,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(patch_size.x, center_size.y)),
                source: Some(Rect::new(src_width - patch, patch, patch, src_height - patch * 2.0)),
                ..Default::default()
            }
        );
        
        // Bottom-left corner
        draw_texture_ex(
            tex,
            dst.x,
            dst.y + dst.h - patch,
            WHITE,
            DrawTextureParams {
                dest_size: Some(patch_size),
                source: Some(Rect::new(0.0, src_height - patch, patch, patch)),
                ..Default::default()
            }
        );
        
        // Bottom edge
        draw_texture_ex(
            tex,
            dst.x + patch,
            dst.y + dst.h - patch,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(center_size.x, patch_size.y)),
                source: Some(Rect::new(patch, src_height - patch, src_width - patch * 2.0, patch)),
                ..Default::default()
            }
        );
        
        // Bottom-right corner
        draw_texture_ex(
            tex,
            dst.x + dst.w - patch,
            dst.y + dst.h - patch,
            WHITE,
            DrawTextureParams {
                dest_size: Some(patch_size),
                source: Some(Rect::new(src_width - patch, src_height - patch, patch, patch)),
                ..Default::default()
            }
        );
    }
} 