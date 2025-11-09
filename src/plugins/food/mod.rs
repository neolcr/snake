use bevy::prelude::*;
use crate::{core::*, plugins::snake};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, initialize_food);
    }
}

fn initialize_food(mut commands: Commands, grid: Res<GridSize>, snake_segments: Query<&GridPosition, With<SnakeSegment>>, mut rand: ResMut<RandomSource>, mut game_start: MessageReader<GameStartEvent>) {
    if snake_segments.is_empty() {
        return;
    }

    if game_start.is_empty() {
        return;
    }

    game_start.clear();

    let occupied_cells: Vec<GridPosition> = snake_segments.iter().copied().collect();
    let mut food_position = GridPosition{x: 0, y: 0};
    loop {
        let x: u32 = rand.rng.gen_range(0..grid.size.x);
        let y: u32 = rand.rng.gen_range(0..grid.size.y);
        if !occupied_cells.iter().any(|pos| pos.x == x && pos.y == y) {
            food_position.x = x;
            food_position.y = y;
            break;
        }
    }

    commands.spawn((
       food_position,
       Food,
       Sprite {
        color: FOOD_COLOR,
        custom_size: Some(Vec2::splat(grid.pixels as f32 - 6.0)),
        ..Default::default()
       } ,
       Transform::from_translation(grid.to_pixels(food_position, 1.0))
    ));
}