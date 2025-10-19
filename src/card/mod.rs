//pub mod asset;
pub mod actions;
pub mod cards;

use bevy::{platform::collections::HashMap, prelude::*};

use crate::{grid::{GridTileColor, Index}, grid_highlight::GridHighlightRequest, press::PressArea, scale_on_touch::ScaleOnTouch, touch::{TouchArea, TouchState}};

pub struct CardPlugin;

#[derive(Component)]
pub struct Card;

#[derive(Component)]
pub struct CardRequirement {
    pub tiles: HashMap<Index, GridTileColor>,
}

//impl Card {
    //pub fn create<T: Component>(
        //card_type: T,
        //position: Vec2, 
    //) -> impl Bundle {
        //let card_area = Vec2::new(64., 96.);

        //(
            //Card,
            //card_type,
            //Sprite::from_color(Color::linear_rgba(0., 0., 1., 0.2), card_area),
            //TouchArea {
                //area: card_area,
            //},
            //PressArea,
            //ScaleOnTouch(1.2),
            //Transform::from_xyz(position.x, position.y, 0.),
        //)
    //}
//}

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, card_highlight)
            .add_systems(Update, setup_card)
            .add_systems(Update, cards::card_random)
            .add_systems(Update, cards::card_refresh)
            .add_systems(Update, cards::card_river);

    }
}

fn card_highlight(
    cards: Query<(&TouchState, &CardRequirement), (With<Card>, Changed<TouchState>)>,
    mut request: MessageWriter<GridHighlightRequest>
) {
    cards
        .into_iter()
        .for_each(|(state, req)| {
            // race condition? touch_state?
            if state.is_touching() {
                request.write(GridHighlightRequest {
          tiles: req.tiles.clone(),
                });
            } else {
                request.write(GridHighlightRequest {
                    tiles: HashMap::new(),
                });
            }
        });
}

fn setup_card(
    mut commands: Commands,
    cards: Query<Entity, Added<Card>>,
) {
    let card_area = Vec2::new(64., 96.);

    cards
        .into_iter()
        .for_each(|entity| {
            commands
                .entity(entity)
                .try_insert((
                    Name::new("Card"),
                    Sprite::from_color(Color::linear_rgba(0., 0., 1., 0.2), card_area),
                    TouchArea {
                        area: card_area,
                    },
                    PressArea,
                    ScaleOnTouch(1.2),
                ));
        });
}

