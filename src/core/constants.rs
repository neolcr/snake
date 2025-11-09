use bevy::prelude::*;

pub const BOARD_SIZE: UVec2 = UVec2::new(1920, 1080);

pub const GRID_SIZE: u32 = 30;
pub const GRID_PIXELS: u32 = 24;
pub const BACKGROUND_COLOR: Color = Color::srgb(13.0 / 255.0, 13.0 / 255.0, 24.0 / 255.0);
pub const CANVAS_COLOR: Color = Color::srgb(36.0 / 255.0, 36.0 / 255.0, 64.0 / 255.0);
pub const DEFAULT_TEXT_COLOR: Color = Color::srgb(220.0 / 255.0, 220.0 / 255.0, 230.0 / 255.0);
pub const FOOD_COLOR: Color = Color::srgb(92.0 / 255.0, 201.0 / 255.0, 113.0 / 255.0);
pub const SNAKE_COLOR: Color = Color::srgb(127.0 / 255.0, 127.0 / 255.0, 130.0 / 255.0);
pub const SNAKE_CELL_PADDING: f32 = 2.0;


