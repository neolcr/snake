use bevy::prelude::*;

#[derive(Component, Copy, Clone, Debug)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
}