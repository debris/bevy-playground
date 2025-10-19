use bevy::{prelude::*, sprite::Anchor};
use crate::card::cards;
use crate::grid::{Grid, GridMovesLabel};
use crate::score::ScoreLabel;
use crate::{RedrawButton, RefreshButton, SimpleButton};
use crate::Card;

pub struct LayoutPlugin;

impl Plugin for LayoutPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_game_view);
    }
}

#[derive(Component)]
pub struct RootView;

#[derive(Component)]
pub struct TopBarView;

#[derive(Component)]
pub struct ContentView;

#[derive(Component)]
pub struct BottomBarView;

#[derive(Component)]
pub struct PlaceholderRefreshButton;

#[derive(Component)]
pub struct PlaceholderCardsView;

#[derive(Component)]
pub struct CastButton;

fn setup_game_view(
    mut commands: Commands, 
) {
    commands.spawn((
        RootView,
        Sprite::from_color(Color::WHITE, Vec2::new(800., 600.)),
        Transform::from_xyz(0., 0., 0.),
        children![(
            TopBarView,
            Sprite::from_color(Color::linear_rgb(0.1, 0.1, 0.1), Vec2::new(800., 96.)),
            Transform::from_xyz(0., 300., 2.),
            Anchor::TOP_CENTER,
            children![
                PlaceholderRefreshButton,
                SimpleButton::create(RefreshButton, "refresh", (400. - 48. - 8., -24. - 8.).into())
            ]
        ),(
            ContentView,
            Sprite::from_color(Color::linear_rgb(0.05, 0.05, 0.05), Vec2::new(800., 600.)),
            Transform::from_xyz(0., 0., 1.),
            Anchor::CENTER,
            children![(
                Grid,
                Transform::from_xyz(0., 0., 0.),
                Visibility::Inherited,
            ), (
                ScoreLabel,
                Text2d::new(""),
                Transform::from_xyz(0., 192., 0.),
            ), (
                GridMovesLabel,
                Text2d::new(""),
                Transform::from_xyz(0., -120., 0.),
            )]
        ),(
            BottomBarView,
            Sprite::from_color(Color::linear_rgb(0.1, 0.1, 0.1), Vec2::new(800., 128.)),
            Transform::from_xyz(0., -300., 2.),
            Anchor::BOTTOM_CENTER,
            children![(
                Card,
                cards::CardRandom,
                Transform::from_xyz(-96., 48. + 16., 3.),
                Anchor::CENTER,
                Visibility::Inherited,
            ), (
                Card,
                cards::CardRandom,
                Transform::from_xyz(0., 48. + 16., 3.),
                Anchor::CENTER,
                Visibility::Inherited,
            ), (
                Card,
                cards::CardRandom,
                Transform::from_xyz(96., 48. + 16., 3.),
                Anchor::CENTER,
                Visibility::Inherited,
            ),
                SimpleButton::create(RedrawButton, "redraw", (-400. + 48. + 8., 32.).into()),
                SimpleButton::create(CastButton, "cast", (400. - 48. - 8., 32.).into()),
            ]
        )
        ]
    ));
}

