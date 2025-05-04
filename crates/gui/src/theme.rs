use macroquad::prelude::{Color, Texture2D, Font, load_ttf_font, load_texture, WHITE, Image};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileSize {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSize {
    pub columns: i32,
    pub rows: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileMap {
    pub tiles: std::collections::HashMap<String, TilePosition>,
    pub tile_size: TileSize,
    pub map_size: MapSize,
    pub image: String,
}

pub struct Theme {
    pub font_default: Option<Font>,  // Make Font optional so we can function without one
    pub color_text: Color,
    pub color_text_disabled: Color,
    pub color_window_background: Color,
    pub color_button_normal: Color,
    pub color_button_hovered: Color,
    pub color_button_active: Color,
    pub window_skin: Texture2D,   // 3×3 nine-patch
    pub button_skin: Texture2D,   // stacked sprite
    pub checkbox_checked: Texture2D,
    pub checkbox_unchecked: Texture2D,
    pub slider_bg: Texture2D,
    pub slider_handle: Texture2D,
    // Store the tilemap data if available
    pub tilemap: Option<TileMap>,
}

impl Theme {
    pub async fn load() -> Result<Self> {
        // Try to load the font - if it fails, we'll just use None
        let font = match load_ttf_font("assets/gui/fonts/Roboto-Regular.ttf").await {
            Ok(font) => Some(font),
            Err(err) => {
                println!("Warning: Failed to load GUI font: {}. Text rendering will be limited.", err);
                None
            }
        };
        
        // Load the tilemap definition
        let tilemap = match Self::load_tilemap_definition("assets/gui/dwarven_gui_tilemap.json") {
            Ok(map) => {
                println!("Successfully loaded tilemap definition");
                Some(map)
            },
            Err(err) => {
                println!("Warning: Failed to load tilemap definition: {}. Using default window skin.", err);
                None
            }
        };
        
        // Try to load the window skin using the tilemap
        let window_skin = if let Some(map) = &tilemap {
            match load_texture(&format!("assets/gui/{}", map.image)).await {
                Ok(tex) => {
                    println!("Successfully loaded window skin from tilemap image");
                    tex
                },
                Err(err) => {
                    println!("Warning: Failed to load tilemap image: {}. Using default window skin.", err);
                    Self::create_default_window_skin()
                }
            }
        } else {
            // Fallback to default skin
            Self::create_default_window_skin()
        };
        
        // Try to load button skin
        let button_skin = match load_texture("assets/gui/button.png").await {
            Ok(tex) => {
                println!("Successfully loaded button skin");
                tex
            },
            Err(err) => {
                println!("Warning: Failed to load button skin: {}. Using default button skin.", err);
                Self::create_default_button_skin()
            }
        };
        
        // Create default checkbox textures
        let checkbox_checked = Self::create_default_checkbox(true);
        let checkbox_unchecked = Self::create_default_checkbox(false);
        
        // Create default slider textures
        let slider_bg = Self::create_default_slider_bg();
        let slider_handle = Self::create_default_slider_handle();
        
        Ok(Self {
            font_default: font,
            color_text: WHITE,
            color_text_disabled: Color::new(0.5, 0.5, 0.5, 0.7),
            color_window_background: Color::new(0.2, 0.2, 0.2, 0.8),
            color_button_normal: Color::new(0.25, 0.25, 0.25, 1.0),
            color_button_hovered: Color::new(0.35, 0.35, 0.35, 1.0),
            color_button_active: Color::new(0.45, 0.45, 0.45, 1.0),
            window_skin,
            button_skin,
            checkbox_checked,
            checkbox_unchecked,
            slider_bg,
            slider_handle,
            tilemap,
        })
    }
    
    // Load the tilemap definition from a JSON file
    fn load_tilemap_definition(path: &str) -> Result<TileMap> {
        // Open and read the file
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        // Parse the JSON
        let tilemap: TileMap = serde_json::from_str(&contents)?;
        Ok(tilemap)
    }
    
    // Create a default window skin texture
    fn create_default_window_skin() -> Texture2D {
        let mut img = Image::gen_image_color(48, 48, Color::new(0.2, 0.2, 0.2, 0.8));
        
        // Draw a border
        for x in 0..48 {
            for y in 0..48 {
                if x < 1 || x >= 47 || y < 1 || y >= 47 {
                    img.set_pixel(x, y, Color::new(0.4, 0.4, 0.4, 1.0));
                }
            }
        }
        
        Texture2D::from_image(&img)
    }
    
    // Create a default button skin texture with three states
    fn create_default_button_skin() -> Texture2D {
        let mut img = Image::gen_image_color(100, 120, Color::new(0.0, 0.0, 0.0, 0.0));
        
        // Normal state (top third)
        for y in 0..40 {
            for x in 0..100 {
                img.set_pixel(x, y, Color::new(0.25, 0.25, 0.25, 1.0));
                
                // Add a border
                if x < 1 || x >= 99 || y < 1 || y >= 39 {
                    img.set_pixel(x, y, Color::new(0.4, 0.4, 0.4, 1.0));
                }
            }
        }
        
        // Hover state (middle third)
        for y in 40..80 {
            for x in 0..100 {
                img.set_pixel(x, y, Color::new(0.35, 0.35, 0.35, 1.0));
                
                // Add a border
                if x < 1 || x >= 99 || y < 41 || y >= 79 {
                    img.set_pixel(x, y, Color::new(0.5, 0.5, 0.5, 1.0));
                }
            }
        }
        
        // Pressed state (bottom third)
        for y in 80..120 {
            for x in 0..100 {
                img.set_pixel(x, y, Color::new(0.45, 0.45, 0.45, 1.0));
                
                // Add a border
                if x < 1 || x >= 99 || y < 81 || y >= 119 {
                    img.set_pixel(x, y, Color::new(0.6, 0.6, 0.6, 1.0));
                }
            }
        }
        
        Texture2D::from_image(&img)
    }
    
    // Create a default checkbox texture
    fn create_default_checkbox(checked: bool) -> Texture2D {
        let mut img = Image::gen_image_color(20, 20, Color::new(0.2, 0.2, 0.2, 1.0));
        
        // Draw a border
        for x in 0..20 {
            for y in 0..20 {
                if x < 1 || x >= 19 || y < 1 || y >= 19 {
                    img.set_pixel(x, y, Color::new(0.5, 0.5, 0.5, 1.0));
                }
            }
        }
        
        // If checked, draw an X
        if checked {
            for i in 4..16 {
                // Draw two diagonal lines
                img.set_pixel(i, i, Color::new(1.0, 1.0, 1.0, 1.0));
                img.set_pixel(i, 20 - i, Color::new(1.0, 1.0, 1.0, 1.0));
                
                // Make the X thicker
                if i > 4 && i < 15 {
                    img.set_pixel(i, i+1, Color::new(1.0, 1.0, 1.0, 1.0));
                    img.set_pixel(i, 20 - i - 1, Color::new(1.0, 1.0, 1.0, 1.0));
                }
            }
        }
        
        Texture2D::from_image(&img)
    }
    
    // Create a default slider background texture
    fn create_default_slider_bg() -> Texture2D {
        let mut img = Image::gen_image_color(100, 4, Color::new(0.15, 0.15, 0.15, 1.0));
        
        // Add a thin border
        for x in 0..100 {
            for y in 0..4 {
                if y == 0 || y == 3 {
                    img.set_pixel(x, y, Color::new(0.3, 0.3, 0.3, 1.0));
                }
            }
        }
        
        Texture2D::from_image(&img)
    }
    
    // Create a default slider handle texture
    fn create_default_slider_handle() -> Texture2D {
        let mut img = Image::gen_image_color(10, 20, Color::new(0.4, 0.4, 0.4, 1.0));
        
        // Add a border
        for x in 0..10 {
            for y in 0..20 {
                if x == 0 || x == 9 || y == 0 || y == 19 {
                    img.set_pixel(x, y, Color::new(0.6, 0.6, 0.6, 1.0));
                }
            }
        }
        
        Texture2D::from_image(&img)
    }
    
    /// Create a default minimal theme for initialization purposes
    pub fn default() -> Self {
        let default_image = Image::gen_image_color(16, 16, Color::new(0.2, 0.2, 0.2, 0.8));
        let default_texture = Texture2D::from_image(&default_image);
        
        Self {
            font_default: None,
            color_text: WHITE,
            color_text_disabled: Color::new(0.5, 0.5, 0.5, 0.7),
            color_window_background: Color::new(0.2, 0.2, 0.2, 0.8),
            color_button_normal: Color::new(0.25, 0.25, 0.25, 1.0),
            color_button_hovered: Color::new(0.35, 0.35, 0.35, 1.0),
            color_button_active: Color::new(0.45, 0.45, 0.45, 1.0),
            window_skin: default_texture.clone(),
            button_skin: default_texture.clone(),
            checkbox_checked: default_texture.clone(),
            checkbox_unchecked: default_texture.clone(),
            slider_bg: default_texture.clone(),
            slider_handle: default_texture.clone(),
            tilemap: None,
        }
    }
} 