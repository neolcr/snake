use bevy::prelude::*;
use std::collections::HashMap;

use crate::core::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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
            timer: Timer::from_seconds(0.15, TimerMode::Repeating),
            force_step: false,
        }
    }
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SnakeState::default()).add_systems(
            Update,
            (
                initialize_snake,
                snake_input,
                snake_move,
                restart_on_click,
            ),
        );
    }
}

fn initialize_snake(
    mut commands: Commands,
    grid: Res<GridSize>,
    mut game_start: MessageReader<GameStartEvent>,
) {
    if game_start.is_empty() {
        return;
    }
    game_start.clear();

    let center = GridPosition {
        x: (grid.size.x / 2) as u32,
        y: (grid.size.y / 2) as u32,
    };

    for snake_index in 0..=2 {
        let position = GridPosition {
            x: (center.x - snake_index) as u32,
            y: center.y as u32,
        };
        commands.spawn((
            position,
            SnakeSegment { index: snake_index },
            Sprite {
                color: SNAKE_COLOR,
                custom_size: Some(Vec2::splat(grid.pixels as f32 - SNAKE_CELL_PADDING)),
                ..Default::default()
            },
            Transform::from_translation(grid.to_pixels(position, 1.0)),
        ));
    }
}

fn snake_input(
    status: Res<GameStatus>,
    mut state: ResMut<SnakeState>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if *status == GameStatus::GameOver {
        return;
    }

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
        let opposite = match state.direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
        if dir != opposite && state.direction != dir {
            state.direction = dir;
            state.force_step = true;
        }
    }
}

#[derive(Component)]
struct GameOverUi;

#[derive(Component)]
struct RestartButton;

fn snake_move(
    mut commands: Commands,
    time: Res<Time>,
    grid: Res<GridSize>,
    mut state: ResMut<SnakeState>,
    mut growth: ResMut<Growth>,
    mut status: ResMut<GameStatus>,
    mut q_segments: Query<(Entity, &SnakeSegment, &mut GridPosition, &mut Transform)>,
) {
    if *status == GameStatus::GameOver || q_segments.is_empty() {
        return;
    }

    state.timer.tick(time.delta());
    let forced = if state.force_step {
        state.force_step = false;
        true
    } else {
        false
    };
    if !state.timer.just_finished() && !forced {
        return;
    }
    if forced {
        state.timer.reset();
    }

    let mut parts: Vec<(Entity, u32, GridPosition)> = q_segments
        .iter_mut()
        .map(|(e, seg, pos, _)| (e, seg.index, *pos))
        .collect();
    parts.sort_by_key(|(_, idx, _)| *idx);

    let mut new_pos: HashMap<Entity, GridPosition> = HashMap::with_capacity(parts.len());

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

    for i in 1..parts.len() {
        let prev_old = parts[i - 1].2;
        let e = parts[i].0;
        new_pos.insert(e, prev_old);
    }

    for (e, _seg, mut pos, mut tf) in q_segments.iter_mut() {
        if let Some(p) = new_pos.get(&e) {
            *pos = *p;
            tf.translation = grid.to_pixels(*p, 1.0);
        }
    }

    if growth.pending > 0 {
        if let Some((_, last_idx, last_old_pos)) = parts.last().copied() {
            let new_index = last_idx + 1;
            let new_pos = last_old_pos;
            commands.spawn((
                new_pos,
                SnakeSegment { index: new_index },
                Sprite {
                    color: SNAKE_COLOR,
                    custom_size: Some(Vec2::splat(grid.pixels as f32 - SNAKE_CELL_PADDING)),
                    ..Default::default()
                },
                Transform::from_translation(grid.to_pixels(new_pos, 1.0)),
            ));
            growth.pending -= 1;
        }
    }

    let mut head: Option<GridPosition> = None;
    let mut body: Vec<GridPosition> = Vec::new();
    let mut entries: Vec<(u32, GridPosition)> = q_segments
        .iter_mut()
        .map(|(_, seg, pos, _)| (seg.index, *pos))
        .collect();
    entries.sort_by_key(|(i, _)| *i);
    for (i, p) in entries.into_iter() {
        if i == 0 {
            head = Some(p);
        } else {
            body.push(p);
        }
    }
    if let Some(h) = head {
        if body.iter().any(|&p| p.x == h.x && p.y == h.y) {
            *status = GameStatus::GameOver;
            spawn_game_over_ui(&mut commands);
        }
    }
}

fn spawn_game_over_ui(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.4)),
            GameOverUi,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(12.0),
                        column_gap: Val::Px(12.0),
                        padding: UiRect::all(Val::Px(16.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.25)),
                ))
                .with_children(|panel| {
                    panel.spawn(Text::new("GAME OVER"));
                    panel
                        .spawn((
                            Button,
                            Node {
                                padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                                ..Default::default()
                            },
                            BackgroundColor(Color::srgb(0.25, 0.25, 0.4)),
                            RestartButton,
                        ))
                        .with_children(|b| {
                            b.spawn(Text::new("Restart"));
                        });
                });
        });
}

fn restart_on_click(
    mut commands: Commands,
    mut status: ResMut<GameStatus>,
    mut growth: ResMut<Growth>,
    mut state: ResMut<SnakeState>,
    mut start_writer: MessageWriter<GameStartEvent>,
    q_ui: Query<Entity, With<GameOverUi>>,
    q_btn: Query<(&Interaction,), (Changed<Interaction>, With<RestartButton>)>,
    q_snake: Query<Entity, With<SnakeSegment>>,
    q_food: Query<Entity, With<Food>>,
    children_q: Query<&Children>,
) {
    for (interaction,) in q_btn.iter() {
        if *interaction == Interaction::Pressed {
            for e in q_ui.iter() { despawn_with_children(&mut commands, e, &children_q); }
            *status = GameStatus::Running;
            growth.pending = 0;
            *state = SnakeState::default();
            for e in q_snake.iter() {
                commands.entity(e).despawn();
            }
            for e in q_food.iter() {
                commands.entity(e).despawn();
            }
            start_writer.write(GameStartEvent);
        }
    }
}

fn despawn_with_children(commands: &mut Commands, root: Entity, children_q: &Query<&Children>) {
    if let Ok(children) = children_q.get(root) {
        let len = children.len();
        for i in 0..len {
            let child = children[i];
            despawn_with_children(commands, child, children_q);
        }
    }
    commands.entity(root).despawn();
}