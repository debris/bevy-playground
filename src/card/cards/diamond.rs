use bevy::prelude::*;
use bevy_rand::prelude::WyRand;
use maplit::hashmap;
use rand::Rng;

use crate::card::CardTrait;
use crate::card::{actions::ActionCombine, CardRequirement};
use crate::grid::{Index, GridTileColor, GridConfig};

#[derive(Component, Default)]
pub struct CardDiamond;

impl CardTrait for CardDiamond {
    fn background_sprite_name() -> String {
        "blue_card.png".into()
    }

    fn sprite_name() -> String {
        "diamond.png".into()
    }

    fn actions() -> impl Bundle {
        ActionCombine
    }

    fn requirements(
        rng: &mut WyRand,
        config: &GridConfig,
    ) -> CardRequirement {
        let x = rng.random_range(0usize..config.dimensions.0);
        let y = rng.random_range(0usize..config.dimensions.1);
            
        CardRequirement {
            tiles: hashmap! {
                Index::new(x, y) => GridTileColor::Blue,
            }.into_iter().collect()
        }
    }

    fn card_name() -> String {
        "Diamond".into()
    }
}

