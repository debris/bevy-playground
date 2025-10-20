pub mod actions;
mod cards;

use rand::prelude::IndexedRandom;

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_rand::{global::GlobalRng, prelude::WyRand};

use cards::{CardCrocodile, CardRiver};
use crate::{grid::{GridTileColor, Index}, grid_highlight::GridHighlightRequest, press::PressArea, scale_on_touch::ScaleOnTouch, touch::{TouchArea, TouchState}};
use crate::tooltip::Tooltip;

pub struct CardPlugin;

#[derive(Component)]
pub struct Card;

#[derive(Component)]
pub struct CardRequirement {
    pub tiles: HashMap<Index, GridTileColor>,
}

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_all_cards_collection)
            .add_systems(Update, card_highlight)
            .add_systems(Update, setup_card)
            .add_systems(Update, card_random)
            .add_systems(Update, card_system::<CardRiver>)
            .add_systems(Update, card_system::<CardCrocodile>);
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
                    TouchArea {
                        area: card_area,
                    },
                    PressArea,
                    ScaleOnTouch(1.2),
                ));
        });
}

#[derive(Component, Default)]
pub struct CardCollection {
    // TODO: store by name etc. 
    // or as a tuple of name + spawner so it's possible to identify the card before spawning
    pub spawners: Vec<fn(&mut Commands, Entity)>,
}

impl CardCollection {
    pub fn add<T: CardTrait + Default>(&mut self) {
        self.spawners.push(|commands, entity| {
            commands.entity(entity).insert(T::default());
        });
    }
}

/// Identifier of CardCollection that contains all cards.
#[derive(Component)]
pub struct AllCards;

pub fn setup_all_cards_collection(
    mut commands: Commands,
) {
    let mut card_collection = CardCollection::default();
    card_collection.add::<CardCrocodile>();
    card_collection.add::<CardRiver>();
    commands.spawn((
        Name::new("All Cards"),
        card_collection,
        AllCards,
    ));
}

#[derive(Component)]
pub struct PlayerCards;

#[derive(Component)]
pub struct CardRandom;

pub fn card_random(
    mut commands: Commands,
    collection: Single<&CardCollection, With<AllCards>>,
    query: Query<Entity, Added<CardRandom>>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    query
        .into_iter()
        .for_each(|e| {
            if let Some(spawner) = collection.spawners.choose(&mut rng) {
                spawner(&mut commands, e);
            }
            
            let mut entity_commands = commands.entity(e);
            entity_commands.try_remove::<CardRandom>();
        });
}

pub trait CardTrait: Component {
    fn background_sprite_name() -> String;
    fn sprite_name() -> String;
    fn actions() -> impl Bundle;
    fn requirements() -> CardRequirement;
    fn card_name() -> String;
}

pub fn card_system<T: CardTrait>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, Added<T>>
) {
    let card_area = Vec2::new(64., 96.);
    query
        .into_iter()
        .for_each(|e| {
            let mut bg_sprite = Sprite::from_image(asset_server.load(T::background_sprite_name()));
            let mut sprite = Sprite::from_image(asset_server.load(T::sprite_name()));
            bg_sprite.custom_size = Some(card_area);
            sprite.custom_size = Some(card_area);

            commands.entity(e)
                .try_insert((
                    T::actions(),
                    T::requirements(),
                    Tooltip::with_text(&T::card_name())
                ))
                .with_children(|e| {
                    e.spawn((
                        bg_sprite,
                        Transform::from_xyz(0., 0., 0.)
                    ));
                    e.spawn((
                        sprite,
                        Transform::from_xyz(0., 0., 1.)
                    ));
                });
        });
}

