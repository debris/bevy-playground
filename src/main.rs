mod grid;
mod tooltip;
mod touch;

use bevy::prelude::*;
use bevy_rand::{self, plugin::EntropyPlugin, prelude::WyRand};
use bevy::input::common_conditions::input_toggle_active;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use grid::{GridConfig, GridPlugin};
use touch::TouchPlugin;
use tooltip::TooltipPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)))
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(TouchPlugin)
        .add_plugins(TooltipPlugin)
        .add_plugins(GridPlugin::new(GridConfig {
            dimensions: (5, 3),
            tile_size: vec2(64., 64.),
            movement_speed: 64.,
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands, 
) {
    commands.spawn((
        Name::new("Camera2d"),
        Camera2d
    ));
}

