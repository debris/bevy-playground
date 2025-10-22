pub mod actions;
mod cards;

use maplit::hashmap;
use rand::prelude::IndexedRandom;

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_rand::{global::GlobalRng, prelude::WyRand};

use cards::{CardCrocodile, CardRiver};
use crate::{grid::{GridTileColor, Index}, grid_highlight::{GridHighlightRequest, GridHighlightsState, GridTileHighlightSide}, press::PressArea, scale_on_touch::ScaleOnTouch, touch::{TouchArea, TouchState}};
use crate::tooltip::Tooltip;

#[derive(Message, Default)]
pub struct CardRedrawRequest;

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
            .add_message::<CardRedrawRequest>()
            .add_systems(Startup, setup_all_cards_collection)
            .add_systems(Update, card_highlight2)
            .add_systems(Update, setup_card)
            .add_systems(Update, card_random)
            .add_systems(Update, setup_cards_view)
            .add_systems(Update, redraw_cards.run_if(on_message::<CardRedrawRequest>))
            .add_systems(Update, card_system::<CardRiver>)
            .add_systems(Update, card_system::<CardCrocodile>);
    }
}

fn card_highlight2(
    mut state: Single<&mut GridHighlightsState>,
    cards: Query<(&CardIndex, &CardRequirement), Added<CardRequirement>>,
    mut request: MessageWriter<GridHighlightRequest>
) {
    for (index, req) in cards {
        let side = match **index {
            0 => GridTileHighlightSide::Left,
            1 => GridTileHighlightSide::Bottom,
            _ => GridTileHighlightSide::Right,
        };

        state.highlights_by_side.insert(side, req.tiles.clone());
    }

    if !cards.is_empty() {
        request.write(GridHighlightRequest);
    }
}

//fn card_highlight(
    //cards: Query<(&TouchState, &CardRequirement, &CardIndex), (With<Card>, Changed<TouchState>)>,
    //mut request: MessageWriter<GridHighlightRequest>
//) {
    //let card = cards
        //.iter()
        //.find(|(t, _, _)| t.is_touching());

    //if let Some((state, req, index)) = card {
        //if state.is_just_touched() {
            //let side = match **index {
                //0 => GridTileHighlightSide::Left,
                //1 => GridTileHighlightSide::Bottom,
                //_ => GridTileHighlightSide::Right,
            //};
            
                
            //request.write(GridHighlightRequest {
                //tiles: req.tiles.clone(),
                //side,
            //});
        //}
    //} else {
        //request.write(GridHighlightRequest {
            //tiles: HashMap::new(),
            //side: GridTileHighlightSide::Left,
        //});
    //}
//}

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

#[derive(Component, Deref, DerefMut)]
pub struct CardIndex(usize);

#[derive(Component)]
pub struct CardsView;

fn setup_cards_view(
    commands: Commands,
    cards_view: Single<Entity, Added<CardsView>>,
) {
    redraw_cards_impl(commands, *cards_view);
}

/// Should be called on_event only
pub fn redraw_cards(
    mut commands: Commands,
    cards_view: Single<(Entity, Option<&Children>), With<CardsView>>, 
) {
    let (cards_view, children) = *cards_view;
    if let Some(children) = children {
        for child in children.iter() {
            commands.entity(child).despawn();
        }
    }

    redraw_cards_impl(commands, cards_view);
}

fn redraw_cards_impl(
    mut commands: Commands,
    cards_view: Entity,
) {
    commands 
        .entity(cards_view)
        .with_children(|view| {
            view.spawn((
                Card,
                CardIndex(0),
                CardRandom,
                Transform::from_xyz(-96., 0., 0.),
                Visibility::Inherited,
            ));
            view.spawn((
                Card,
                CardIndex(1),
                CardRandom,
                Transform::from_xyz(0., 0., 0.),
                Visibility::Inherited,
            ));
            view.spawn((
                Card,
                CardIndex(2),
                CardRandom,
                Transform::from_xyz(96., 0., 0.),
                Visibility::Inherited,
            ));
        });
}

