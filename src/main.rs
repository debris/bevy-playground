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

use bevy::prelude::*;
use bevy_rand::{self, plugin::EntropyPlugin, prelude::WyRand};
use bevy::input::common_conditions::input_toggle_active;
use bevy_egui::{EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use card::{actions::ActionPlugin, Card, CardPlugin};
use grid::{GridConfig, GridPlugin, GridRefreshRequest};
use grid_highlight::GridHighlightPlugin;
use layout::LayoutPlugin;
use mouse::MousePlugin;
use press::{PressPlugin, PressState};
use scale_on_touch::ScaleOnTouchPlugin;
use score::ScorePlugin;
use simple_button::SimpleButton;
use styles::StylePlugin;
use touch::TouchPlugin;
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
            tile_size: vec2(64., 64.),
            movement_speed: 128.,
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, refresh_button_system)
        .add_systems(Update, redraw_button_system)
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

}

#[derive(Component)]
struct RefreshButton;

fn refresh_button_system(
    state: Single<&PressState, (Changed<PressState>, With<RefreshButton>)>,
    mut refresh: MessageWriter<GridRefreshRequest>,
) {
    if **state == PressState::JustReleased {
        refresh.write(GridRefreshRequest);
    }
}

#[derive(Component)]
pub struct RedrawButton;

fn redraw_button_system(
    state: Single<&PressState, (Changed<PressState>, With<RedrawButton>)>
) {
    if **state == PressState::JustReleased {
        // redraw cards
    }
}
