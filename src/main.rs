mod grid;
mod tooltip;
mod touch;
mod styles;
mod scale_on_touch;

use bevy::prelude::*;
use bevy_rand::{self, plugin::EntropyPlugin, prelude::WyRand};
use bevy::input::common_conditions::input_toggle_active;
use bevy_egui::{EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use grid::{Grid, GridConfig, GridPlugin, GridRefreshRequest};
use scale_on_touch::ScaleOnTouchPlugin;
use styles::StylePlugin;
use touch::TouchPlugin;
use tooltip::TooltipPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)))
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(StylePlugin)
        .add_plugins(TouchPlugin)
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
    commands.spawn(button(&asset_server));
}

#[derive(Component)]
struct RefreshButton;

fn button_system(
    mut query: Query<&Interaction, (Changed<Interaction>, With<RefreshButton>)>,
    mut refresh: MessageWriter<GridRefreshRequest>,
) {
    for interaction in &mut query {
        match *interaction {
            Interaction::Pressed => {
                refresh.write(GridRefreshRequest);
            },
            Interaction::Hovered => {
            },
            Interaction::None => {
            }
        }
    }
}

fn button(asset_server: &AssetServer) -> impl Bundle {
    (
        Button,
        RefreshButton,
        Node {
            width: px(64.),
            height: px(64.),
            border: UiRect::all(px(5)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor::all(Color::WHITE),
        BorderRadius::all(px(15.0)),
        BackgroundColor(Color::BLACK),
        children![(
            Text::new("R"),
            TextFont {
                font: asset_server.load("fonts/fragmentcore.otf"),
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )]
    )
}
