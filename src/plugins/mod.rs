use bevy::prelude::*;
use crate::core::*;

// Declare submodules so their code is compiled and usable.
pub mod board;
pub mod camera;
pub mod shared; // reserved for future shared plugin code

pub mod food;
pub mod snake;
pub mod game;

// Re-export the SharedPlugin from the `shared` module so `use plugins::SharedPlugin` picks the one
// that also registers messages (GameStartEvent) and inserts GridSize.
pub use shared::SharedPlugin;