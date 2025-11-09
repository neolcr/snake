#![allow(unused)]
use bevy::prelude::*;
mod core;
use core::*;
use dotenvy::dotenv;
mod plugins;
use plugins::{SharedPlugin, board::BoardPlugin, camera::CameraPlugin};

use crate::plugins::{food::FoodPlugin, game::GamePlugin, snake::SnakePlugin};

fn main() {
    dotenv().ok();
    let game_title = std::env::var("GAME_TITLE").unwrap_or_else(|_| "My Snake Game".to_string());
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: game_title.into(),
                // BOARD_SIZE is UVec2; WindowResolution implements From<UVec2>
                resolution: BOARD_SIZE.into(),
                ..default()
            }),
            ..default()
        }))
        // Our custom plugins/resources
        .add_plugins((
            SharedPlugin, 
            BoardPlugin, 
            CameraPlugin,
            GamePlugin,
            SnakePlugin,
            FoodPlugin
        ))
        .run();

    error!("App is running");
    warn!("App is running");
    info!("App is running");
    debug!("App is running");
    trace!("App is running");

}
