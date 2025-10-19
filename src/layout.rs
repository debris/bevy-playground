use bevy::{prelude::*, sprite::Anchor};
use crate::card::cards;
use crate::grid::Grid;
use crate::{SimpleButton, RefreshButton};
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
pub struct PlaceholderGrid;

#[derive(Component)]
pub struct PlaceholderScoreLabel;

#[derive(Component)]
pub struct PlaceholderCardsView;

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
                Transform::from_xyz(0., 0., 2.),
                Visibility::Inherited,
            ), (
                PlaceholderScoreLabel,
            )]
        ),(
            BottomBarView,
            Sprite::from_color(Color::linear_rgb(0.1, 0.1, 0.1), Vec2::new(800., 128.)),
            Transform::from_xyz(0., -300., 2.),
            Anchor::BOTTOM_CENTER,
            children![(
                Card,
                cards::CardRandom,
                Transform::from_xyz(0., 48. + 16., 3.),
                Anchor::CENTER,
            ), (
                PlaceholderCardsView,
            )]
        )
        ]
    ));
}

