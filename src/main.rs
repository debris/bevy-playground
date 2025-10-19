mod grid;
mod tooltip;
mod touch;
mod styles;
mod scale_on_touch;
mod mouse;
mod press;
mod simple_button;
mod card;
mod grid_highlight;
mod layout;
mod score;

use bevy::{prelude::*, sprite::Anchor};
use bevy_rand::{self, plugin::EntropyPlugin, prelude::WyRand};
use bevy::input::common_conditions::input_toggle_active;
use bevy_egui::{EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use card::{actions::ActionPlugin, cards, Card, CardPlugin};
use grid::{Grid, GridConfig, GridPlugin, GridRefreshRequest};
use grid_highlight::GridHighlightPlugin;
use layout::LayoutPlugin;
use mouse::MousePlugin;
use press::{PressPlugin, PressState};
use scale_on_touch::ScaleOnTouchPlugin;
use score::ScorePlugin;
use simple_button::SimpleButton;
use styles::StylePlugin;
use touch::{TouchPlugin};
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
        .add_plugins(GridHighlightPlugin)
        .add_plugins(CardPlugin)
        .add_plugins(ActionPlugin)
        .add_plugins(LayoutPlugin)
        .add_plugins(ScorePlugin)
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

    //commands.spawn(Grid::create(Vec2::ZERO));

    //// top bar
    //commands.spawn((
        //Sprite::from_color(Color::BLACK, Vec2::new(800., 64.)),
        //Transform::from_xyz(0., 300., 0.),
        //Anchor::TOP_CENTER,
        //children![
            //SimpleButton::create(RefreshButton, "refresh", (400. - 48. - 8., -24. - 8.).into())
        //]
    //));

    //// bottom bar
    //commands.spawn((
        //Sprite::from_color(Color::BLACK, Vec2::new(800., 128.)),
        //Transform::from_xyz(0., -300., 0.),
        //Anchor::BOTTOM_CENTER,
    //));
    
    //commands.spawn(Card::create(
            //cards::CardRandom,
            //Vec2::new(0., -96. * 2.),
    //));
    //commands.spawn(Card::create(
            //cards::CardRandom,
            //Vec2::new(96., -96. * 2.),
    //));
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

