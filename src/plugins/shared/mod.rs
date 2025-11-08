// Empty shared module; remove if unused. Placeholder for future common systems/resources.

use bevy::prelude::*;

pub struct SharedResourcesPlugin;
impl Plugin for SharedResourcesPlugin {
	fn build(&self, _app: &mut App) {}
}
