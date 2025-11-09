use bevy::prelude::*;
use crate::core::*;

// Declare submodules so their code is compiled and usable.
pub mod board;
pub mod camera;
pub mod shared; // reserved for future shared plugin code

pub mod food;
pub mod snake;
pub mod game;

pub struct SharedPlugin;
impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App){
        app.insert_resource(GridSize::default());
    }
}