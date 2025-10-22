use bevy::prelude::*;

use crate::{grid::{GridTile, GridTileByIndex, GridTileColor}};
use crate::score::Score;

use super::CardRequirement;

#[derive(Message, Default)]
pub struct ExecuteActions;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<ExecuteActions>()
            //.add_systems(Update, action_refresh)
            .add_systems(Update, action_combine.run_if(on_message::<ExecuteActions>));
    }
}

//#[derive(Component)]
//pub struct ActionRefresh;

//fn action_refresh(
    //query: Query<&PressState, (With<ActionRefresh>, Changed<PressState>)>,
    //mut refresh: MessageWriter<GridRefreshRequest>,
//) {
    //query
        //.into_iter()
        //.filter(|state| {
            //*state == &PressState::JustReleased
        //})
        //.for_each(|_| {
            //refresh.write(GridRefreshRequest);
        //});
//}

/// Combine the total value of all matching squares.
#[derive(Component)]
pub struct ActionCombine;

fn action_combine(
    mut score: Single<&mut Score>,
    tiles_by_index: Single<&GridTileByIndex>,
    tiles: Query<&GridTileColor, With<GridTile>>,
    query: Query<&CardRequirement, With<ActionCombine>>,
) {
    query
        .into_iter()
        .for_each(|req| {
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

