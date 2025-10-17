mod grid;
mod tooltip;
mod touch;
mod styles;
mod scale_on_touch;
mod mouse;
mod press;
mod simple_button;

use bevy::prelude::*;
use bevy_rand::{self, plugin::EntropyPlugin, prelude::WyRand};
use bevy::input::common_conditions::input_toggle_active;
use bevy_egui::{EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use grid::{Grid, GridConfig, GridPlugin, GridRefreshRequest};
use mouse::MousePlugin;
use press::{PressArea, PressPlugin, PressState};
use scale_on_touch::ScaleOnTouchPlugin;
use simple_button::SimpleButton;
use styles::StylePlugin;
use touch::{TouchArea, TouchPlugin};
use tooltip::TooltipPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)))
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(MousePlugin)
        .add_plugins(StylePlugin)
        .add_plugins(TouchPlugin)
        .add_plugins(PressPlugin)
        .add_plugins(ScaleOnTouchPlugin)
        .add_plugins(TooltipPlugin)
        .add_plugins(GridPlugin::new(GridConfig {
            dimensions: (5, 3),
            tile_size: vec2(96., 96.),
            movement_speed: 128.,
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    let projection = OrthographicProjection {
        scaling_mode: bevy::camera::ScalingMode::AutoMin { min_width: 800., min_height: 600. },
        ..OrthographicProjection::default_2d()
    };

    commands.spawn((
        Name::new("Camera2d"),
        Camera2d,
        Projection::Orthographic(projection)
    ));

    commands.spawn(Grid::create(Vec2::ZERO));
    //commands.spawn(button(&asset_server));
    commands.spawn(SimpleButton::create(RefreshButton, "refresh", (0., 96. * 2.).into()));
}

#[derive(Component)]
struct RefreshButton;

fn button_system(
    interactions: Query<&PressState, (Changed<PressState>, With<RefreshButton>)>,
    mut refresh: MessageWriter<GridRefreshRequest>,
) {
    interactions 
        .into_iter()
        .filter(|i| *i == &PressState::JustReleased)
        .for_each(|_| {
            refresh.write(GridRefreshRequest);
        });
}

fn button(asset_server: &AssetServer) -> impl Bundle {
    (
        //Button,
        PressArea,
        TouchArea {
            area: Vec2::splat(100.),
        },
        RefreshButton,
        Sprite::from_color(Color::linear_rgba(1., 0., 0., 0.2), Vec2 { x: 100., y: 100. }),
        Transform::from_xyz(0., 96. * 2., 0.),
        children![(
            Transform::from_xyz(0., 0., 0.),
            Text2d::new("R"),
            TextFont {
                font: asset_server.load("fonts/fragmentcore.otf"),
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        )]
    )
}

