use bevy::prelude::*;

#[derive(Message)]
pub struct GameStartEvent;

#[derive(Message)]
pub struct SnakeGrow(pub u32);