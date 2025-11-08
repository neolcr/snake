use crate::core::*;
use bevy::prelude::*;
use bevy::text::FontSmoothing;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_canvas);
    }
}

fn draw_canvas(grid: Res<GridSize>, mut commands: Commands) {
    // Background: convert UVec2 to Vec2 for Sprite custom_size
    let background_size = Vec2::new(BOARD_SIZE.x as f32, BOARD_SIZE.y as f32);
    commands.spawn((
        Sprite {
            color: BACKGROUND_COLOR,
            custom_size: Some(background_size),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, -2.0),
    ));

    // Canvas (inner playable area / grid)
    let board_size = Vec2::new(
        grid.size.x as f32 * grid.pixels as f32,
        grid.size.y as f32 * grid.pixels as f32,
    );

    commands.spawn((
        Sprite {
            color: CANVAS_COLOR,
            custom_size: Some(board_size),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));

    // Optional: outline the canvas with a thin border so it's visually distinct
    let border_thickness = 4.0;
    let half_w = board_size.x / 2.0;
    let half_h = board_size.y / 2.0;
    let border_color = Color::WHITE;

    // Top border
    commands.spawn((
        Sprite {
            color: border_color,
            custom_size: Some(Vec2::new(board_size.x, border_thickness)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, half_h - border_thickness / 2.0, -0.5),
    ));
    // Bottom border
    commands.spawn((
        Sprite {
            color: border_color,
            custom_size: Some(Vec2::new(board_size.x, border_thickness)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, -half_h + border_thickness / 2.0, -0.5),
    ));
    // Left border
    commands.spawn((
        Sprite {
            color: border_color,
            custom_size: Some(Vec2::new(border_thickness, board_size.y)),
            ..Default::default()
        },
        Transform::from_xyz(-half_w + border_thickness / 2.0, 0.0, -0.5),
    ));
    // Right border
    commands.spawn((
        Sprite {
            color: border_color,
            custom_size: Some(Vec2::new(border_thickness, board_size.y)),
            ..Default::default()
        },
        Transform::from_xyz(half_w - border_thickness / 2.0, 0.0, -0.5),
    ));
}
