use bevy::prelude::*;
use maplit::hashmap;

use crate::card::CardTrait;
use crate::card::{actions::ActionCombine, CardRequirement};
use crate::grid::{Index, GridTileColor};

#[derive(Component, Default)]
pub struct CardRiver;

impl CardTrait for CardRiver {
    fn background_sprite_name() -> String {
        "blue_card.png".into()
    }

    fn sprite_name() -> String {
        "river.png".into()
    }

    fn actions() -> impl Bundle {
        ActionCombine
    }

    fn requirements() -> CardRequirement {
        CardRequirement {
            tiles: hashmap! {
                Index::new(0, 0) => GridTileColor::Blue,
                Index::new(1, 0) => GridTileColor::Blue,
                Index::new(2, 0) => GridTileColor::Blue,
                Index::new(3, 0) => GridTileColor::Blue,
                Index::new(4, 0) => GridTileColor::Blue,
            }.into_iter().collect()
        }
    }

    fn card_name() -> String {
        "Card River".into()
    }
}

