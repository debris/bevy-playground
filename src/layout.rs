use bevy::{prelude::*, sprite::Anchor};
use crate::card;
use crate::card::actions::ExecuteActions;
use crate::grid::{Grid, GridMovesLabel, GridRefreshRequest, GridResetMovesRequest};
use crate::score::ScoreLabel;
use crate::simple_button::{button_system, SimpleButton};
use crate::tooltip_on_touch::TooltipView;

#[derive(Component)]
struct RefreshButton;

#[derive(Component)]
pub struct RedrawButton;

pub struct LayoutPlugin;

impl Plugin for LayoutPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<DisplayMainMenu>()
            .add_message::<DisplayGameView>()
            .add_systems(Startup, setup_root_view)
            .add_systems(Update, button_system::<PlayButton, DisplayGameView>)
            .add_systems(Update, button_system::<BackButton, DisplayMainMenu>)
            .add_systems(Update, button_system::<RefreshButton, GridRefreshRequest>)
            .add_systems(Update, button_system::<RedrawButton, card::CardRedrawRequest>)

            // both send the message
            // messages should be received in the next update
            // redraw only queue card redraw so the order should not matter
            // TODO: check for potential race condition
            .add_systems(Update, button_system::<CastButton, ExecuteActions>)
            .add_systems(Update, button_system::<CastButton, card::CardRedrawRequest>)
            .add_systems(Update, button_system::<CastButton, GridResetMovesRequest>)


            .add_systems(Update, display_main_menu.run_if(on_message::<DisplayMainMenu>))
            .add_systems(Update, display_game_view.run_if(on_message::<DisplayGameView>));
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
pub struct CastButton;

#[derive(Message, Default)]
pub struct DisplayMainMenu;

#[derive(Message, Default)]
pub struct DisplayGameView;

fn setup_root_view(
    mut commands: Commands, 
    mut writer: MessageWriter<DisplayMainMenu>
) {
    commands.spawn((
        RootView,
        Name::new("Root View"),
        Sprite::from_color(Color::linear_rgb(0.05, 0.05, 0.05), Vec2::new(800., 600.)),
        Transform::from_xyz(0., 0., 0.),
    ));

    writer.write(DisplayMainMenu);
}

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct BackButton;


fn display_main_menu(
    mut commands: Commands, 
    root: Single<(Entity, Option<&Children>), With<RootView>>,
) {
    let (root, children) = *root;
    if let Some(children) = children {
        for child in children.iter() {
            commands.entity(child).despawn();
        }
    }

    commands
        .entity(root)
        .with_children(|root| {
            root.spawn(
                SimpleButton::create(PlayButton, "play", Vec2::ZERO),
            );
            //root.spawn((
                //AnimatedSprite {
                    //filename: "green_expect.png".into(),
                    //tilesize: UVec2::splat(32),
                    //frames: 4,
                //},
                //Transform::from_xyz(0., 100., 0.),
            //));
        });
}

fn display_game_view(
    mut commands: Commands, 
    root: Single<(Entity, Option<&Children>), With<RootView>>,
) {
    let (root, children) = *root;
    if let Some(children) = children {
        for child in children.iter() {
            commands.entity(child).despawn();
        }
    }

    commands
        .entity(root)
        .with_children(|root| {

            // top one can be removed?
            root.spawn((
                Sprite::from_color(Color::WHITE, Vec2::new(800., 600.)),
                Transform::from_xyz(0., 0., 0.),
                children![(
                    TopBarView,
                    Sprite::from_color(Color::linear_rgb(0.1, 0.1, 0.1), Vec2::new(800., 96.)),
                    Transform::from_xyz(0., 300., 2.),
                    Anchor::TOP_CENTER,
                    children![
                        SimpleButton::create(BackButton, "back", (-400. + 48. + 8., -24. - 8.).into()),
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
                        ScoreLabel,
                        Text2d::new(""),
                        Transform::from_xyz(0., 192., 0.),
                    ), (
                        GridMovesLabel,
                        Text2d::new(""),
                        Transform::from_xyz(0., -120., 0.),
                    ), (
                        TooltipView,
                        Transform::from_xyz(300., 64., 0.)
                    )]
                ),(
                    BottomBarView,
                    Sprite::from_color(Color::linear_rgb(0.1, 0.1, 0.1), Vec2::new(800., 128.)),
                    Transform::from_xyz(0., -300., 2.),
                    Anchor::BOTTOM_CENTER,
                    children![(
                        card::CardsView,
                        Transform::from_xyz(0., 48. + 16., 3.),
                        Anchor::CENTER,
                        Visibility::Inherited,
                    ), 
                        SimpleButton::create(RedrawButton, "redraw", (-400. + 48. + 8., 32.).into()),
                        SimpleButton::create(CastButton, "cast", (400. - 48. - 8., 32.).into()),
                    ]
                )
                ]
            ));
        });
}

