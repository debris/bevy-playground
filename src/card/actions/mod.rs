use bevy::prelude::*;

use crate::core::prelude::*;
use crate::{grid::{GridRefreshRequest, GridTile, GridTileByIndex, GridTileColor}};
use crate::score::Score;

use super::CardRequirement;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, action_refresh)
            .add_systems(Update, action_combine);
    }
}

#[derive(Component)]
pub struct ActionRefresh;

fn action_refresh(
    query: Query<&PressState, (With<ActionRefresh>, Changed<PressState>)>,
    mut refresh: MessageWriter<GridRefreshRequest>,
) {
    query
        .into_iter()
        .filter(|state| {
            *state == &PressState::JustReleased
        })
        .for_each(|_| {
            refresh.write(GridRefreshRequest);
        });
}

/// Combine the total value of all matching squares.
#[derive(Component)]
pub struct ActionCombine;

fn action_combine(
    mut score: Single<&mut Score>,
    tiles_by_index: Single<&GridTileByIndex>,
    tiles: Query<&GridTileColor, With<GridTile>>,
    query: Query<(&PressState, &CardRequirement), (With<ActionCombine>, Changed<PressState>)>,
) {
    query
        .into_iter()
        .filter(|(state, _)| {
            *state == &PressState::JustReleased
        })
        .for_each(|(_, req)| {
            for (index, expected_color) in req.tiles.iter() {
                if let Some(tile_entity) = tiles_by_index.get(index) {
                    if let Some(color) = tiles.get(*tile_entity).ok() {
                        if color.is_matching(expected_color) {
                            score.0 += 1;
                        }
                    }
                }
            }
            println!("action combine!");
            // TODO: proper scoring
        });
}

