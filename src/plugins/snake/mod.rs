use bevy::prelude::*;
use std::collections::HashMap;
use crate::{core::*, plugins::game};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction { Up, Down, Left, Right }

#[derive(Resource)]
struct SnakeState {
    direction: Direction,
    timer: Timer,
    force_step: bool,
}

impl Default for SnakeState {
    fn default() -> Self {
        SnakeState {
            direction: Direction::Right,
            // Keep the original tick cadence; responsiveness comes from force_step
            timer: Timer::from_seconds(0.15, TimerMode::Repeating),
            force_step: false,
        }
    }
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SnakeState::default())
            .add_systems(Update, (
                initialize_snake,
                snake_input,
                snake_move,
            ));
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

fn snake_input(mut state: ResMut<SnakeState>, keys: Res<ButtonInput<KeyCode>>) {
    let new_dir = if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        Some(Direction::Up)
    } else if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        Some(Direction::Down)
    } else if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
        Some(Direction::Left)
    } else if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
        Some(Direction::Right)
    } else {
        None
    };

    if let Some(dir) = new_dir {
        // Prevent reversing directly into the body
        let opposite = match state.direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
        if dir != opposite {
            if state.direction != dir {
                // Apply direction immediately and force a step this frame
                state.direction = dir;
                state.force_step = true;
            }
        }
    }
}

fn snake_move(
    time: Res<Time>,
    grid: Res<GridSize>,
    mut state: ResMut<SnakeState>,
    mut q_segments: Query<(Entity, &SnakeSegment, &mut GridPosition, &mut Transform)>,
) {
    if q_segments.is_empty() { return; }

    state.timer.tick(time.delta());
    let forced = if state.force_step { state.force_step = false; true } else { false };
    if !state.timer.just_finished() && !forced { return; }
    // If we forced a step due to input, reset the timer so we don't double-step immediately
    if forced { state.timer.reset(); }

    // Snapshot current positions and sort by index
    let mut parts: Vec<(Entity, u32, GridPosition)> = q_segments
        .iter_mut()
        .map(|(e, seg, pos, _)| (e, seg.index, *pos))
        .collect();
    parts.sort_by_key(|(_, idx, _)| *idx);

    // Map of new positions by entity
    let mut new_pos: HashMap<Entity, GridPosition> = HashMap::with_capacity(parts.len());

    // Head movement
    if let Some((head_e, _head_idx, head_pos)) = parts.first().copied() {
        let mut next = head_pos;
        match state.direction {
            Direction::Up => {
                next.y = if next.y + 1 < grid.size.y { next.y + 1 } else { 0 };
            }
            Direction::Down => {
                next.y = if next.y > 0 { next.y - 1 } else { grid.size.y - 1 };
            }
            Direction::Left => {
                next.x = if next.x > 0 { next.x - 1 } else { grid.size.x - 1 };
            }
            Direction::Right => {
                next.x = if next.x + 1 < grid.size.x { next.x + 1 } else { 0 };
            }
        }
        new_pos.insert(head_e, next);
    }

    // Body follows previous part's old position
    for i in 1..parts.len() {
        let prev_old = parts[i - 1].2;
        let e = parts[i].0;
        new_pos.insert(e, prev_old);
    }

    // Apply updates and update transforms
    for (e, _seg, mut pos, mut tf) in q_segments.iter_mut() {
        if let Some(p) = new_pos.get(&e) {
            *pos = *p;
            tf.translation = grid.to_pixels(*p, 1.0);
        }
    }
}