use bevy::prelude::*;
use bevy_rand::{global::GlobalRng, prelude::WyRand};
use maplit::hashmap;
use rand::Rng;

use crate::{card::{actions::ActionCombine, CardRequirement}, grid::{GridTileColor, Index}};
use crate::tooltip::Tooltip;

use super::actions::ActionRefresh;

#[derive(Component)]
pub struct CardRandom;

pub fn card_random(
    mut commands: Commands,
    query: Query<Entity, Added<CardRandom>>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    query
        .into_iter()
        .for_each(|e| {
            let mut entity_commands = commands.entity(e);
            let n: u32 = rng.random::<u32>() % 2;

            match n {
                0 => entity_commands.try_insert(CardRefresh),
                1 => entity_commands.try_insert(CardRiver),
                _ => {
                    unreachable!("should not be here");
                }
            };

            entity_commands.try_remove::<CardRandom>();
        });
}

#[derive(Component)]
pub struct CardRefresh;

pub fn card_refresh(
    mut commands: Commands,
    query: Query<Entity, Added<CardRefresh>>
) {
    query
        .into_iter()
        .for_each(|e| {
            commands.entity(e)
                .try_insert(ActionRefresh)
                .try_insert(CardRequirement {
                    tiles: hashmap! {
                        Index::new(0, 0) => GridTileColor::Blue,
                        Index::new(1, 1) => GridTileColor::Red,
                        Index::new(1, 2) => GridTileColor::Green,
                    }.into_iter().collect(),
                })
                .try_insert(Tooltip::with_text("Card Refresh"));
        
        });
}

#[derive(Component)]
pub struct CardRiver;

pub fn card_river(
    mut commands: Commands,
    query: Query<Entity, Added<CardRiver>>
) {
    query
        .into_iter()
        .for_each(|e| {
            commands.entity(e)
                .try_insert(ActionCombine)
                .try_insert(CardRequirement {
                    tiles: hashmap! {
                        Index::new(0, 0) => GridTileColor::Blue,
                        Index::new(1, 0) => GridTileColor::Blue,
                        Index::new(2, 0) => GridTileColor::Blue,
                        Index::new(3, 0) => GridTileColor::Blue,
                        Index::new(4, 0) => GridTileColor::Blue,
                    }.into_iter().collect(),
                })
                .try_insert(Tooltip::with_text("Card River"));
        });
}

