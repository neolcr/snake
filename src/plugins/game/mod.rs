use bevy::prelude::*;
use crate::core::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_game_message);
    }
}

fn initialize_game_message(mut writer: MessageWriter<GameStartEvent>) {
    // For Bevy messages, use write().push(...) to enqueue a message.
    writer.write(GameStartEvent);
}