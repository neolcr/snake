use bevy::prelude::*;
use crate::{core::*, plugins::game};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, initialize_snake);
    }
}

fn initialize_snake(mut commands: Commands, grid: Res<GridSize>, mut game_start: MessageReader<GameStartEvent>){
    if game_start.is_empty() {
        return;
    }

    game_start.clear();

    let center = GridPosition {
        x: (grid.size.x  / 2) as u32,
        y: (grid.size.y  / 2) as u32,
    };

    for snake_index in 0..=2 {
        let position = GridPosition {
            x: (center.x - snake_index) as u32,
            y: center.y as u32,
        };

        commands.spawn((
           position,
           SnakeSegment {index: snake_index},
           Sprite {
                color: SNAKE_COLOR,
                custom_size: Some(Vec2::splat(grid.pixels as f32 - SNAKE_CELL_PADDING)),
                ..Default::default()
           },
           Transform::from_translation(grid.to_pixels(position, 1.0)) 
        ));
    }
}