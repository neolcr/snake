use bevy::prelude::*;
use rand::Rng; // for gen_range
use crate::{core::*, plugins::snake};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            initialize_food,
            handle_food_collision,
        ));
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
        let x: u32 = rand.rng.random_range(0..grid.size.x);
        let y: u32 = rand.rng.random_range(0..grid.size.y);
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

fn handle_food_collision(
    grid: Res<GridSize>,
    mut rand: ResMut<RandomSource>,
    mut growth: ResMut<Growth>,
    // Mark disjointness: snake query excludes Food so it can't overlap q_food
    q_snake: Query<(&SnakeSegment, &GridPosition), Without<Food>>,
    mut q_food: Query<(&mut GridPosition, &mut Transform), With<Food>>,
) {
    // Require at least one food item and a snake
    let Ok((mut food_pos, mut food_tf)) = q_food.single_mut() else { return; };
    if q_snake.is_empty() { return; }

    // Find the head (smallest index, typically 0)
    let mut head_pos_opt: Option<GridPosition> = None;
    for (seg, pos) in q_snake.iter() {
        if head_pos_opt.map(|_| seg.index == 0).unwrap_or(true) {
            // Prefer index 0 if present; otherwise just take the first we see
            if seg.index == 0 {
                head_pos_opt = Some(*pos);
                break;
            } else if head_pos_opt.is_none() {
                head_pos_opt = Some(*pos);
            }
        }
    }
    let Some(head_pos) = head_pos_opt else { return; };

    // If head is on the food, move food to a random empty cell
    if head_pos.x == food_pos.x && head_pos.y == food_pos.y {
        // Collect occupied cells (snake body)
        let occupied: Vec<GridPosition> = q_snake.iter().map(|(_, p)| *p).collect();
        let mut new_pos = *food_pos;
        // Try until we find a free cell
        for _ in 0..(grid.size.x * grid.size.y) {
            let x: u32 = rand.rng.random_range(0..grid.size.x);
            let y: u32 = rand.rng.random_range(0..grid.size.y);
            if !occupied.iter().any(|p| p.x == x && p.y == y) {
                new_pos.x = x;
                new_pos.y = y;
                break;
            }
        }

        // Apply
        *food_pos = new_pos;
        food_tf.translation = grid.to_pixels(new_pos, 1.0);
        // Request growth by 1
        growth.pending = growth.pending.saturating_add(1);
    }
}