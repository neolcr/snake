use bevy::prelude::*;
use crate::core::*;

pub struct SharedPlugin;
impl Plugin for SharedPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_message::<GameStartEvent>()
		.insert_resource(GridSize::default());
	}
}
