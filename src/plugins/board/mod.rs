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

    //canvas
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

    for x in 0..=grid.size.x {
        for y in 0..=grid.size.y {
            let position = GridPosition { x: x, y: y };
            commands.spawn((
                Sprite {
                    color: Color::srgb(210.0 / 255.0, 210.0 / 255.0, 210.0 / 255.0),
                    custom_size: Some(Vec2::splat(grid.pixels as f32 - 2.0)),
                    ..Default::default()
                },
                Transform::from_translation(grid.to_pixels(position, 1.0))
            ));
        }
    }
}
