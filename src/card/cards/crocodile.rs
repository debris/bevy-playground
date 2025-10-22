use bevy::prelude::*;
use maplit::hashmap;

use crate::card::CardTrait;
use crate::card::{actions::ActionCombine, CardRequirement};
use crate::grid::{Index, GridTileColor};

#[derive(Component, Default)]
pub struct CardCrocodile;

impl CardTrait for CardCrocodile {
    fn background_sprite_name() -> String {
        "green_card.png".into()
    }

    fn sprite_name() -> String {
        "crocodile.png".into()
    }

    fn actions() -> impl Bundle {
        ActionCombine
    }

    fn requirements() -> CardRequirement {
        CardRequirement {
            tiles: hashmap! {
                Index::new(1, 1) => GridTileColor::Green,
                Index::new(1, 2) => GridTileColor::Green,
            }.into_iter().collect()
        }
    }

    fn card_name() -> String {
        "Card Crocodile".into()
    }
}

