mod touch;
mod grid;

use bevy::prelude::*;
use bevy_rand::{self, plugin::EntropyPlugin, prelude::WyRand};
use grid::{GridConfig, GridPlugin};
use touch::TouchPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(TouchPlugin)
        .add_plugins(GridPlugin::new(GridConfig {
            dimensions: (3, 3),
            tile_size: vec2(32., 32.),
            movement_speed: 64.,
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands, 
) {
    commands.spawn(Camera2d);
}

