use bevy::prelude::*;

use crate::{grid::{GridTile, GridTileByIndex, GridTileColor}};
use crate::score::Score;

use super::CardRequirement;

#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
enum ActionSet {
    Main,
    Finish,
}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_sets(Update, (
                ActionSet::Main,
                ActionSet::Finish.after(ActionSet::Main)
            ))
            .add_message::<ExecuteActions>()
            .add_message::<FinishedExecution>()
            .add_systems(Update, action_combine.in_set(ActionSet::Main).run_if(on_message::<ExecuteActions>))
            .add_systems(Update, finish_execution.in_set(ActionSet::Finish).run_if(on_message::<ExecuteActions>));
    }
}

#[derive(Message, Default)]
pub struct ExecuteActions;

#[derive(Message)]
pub struct FinishedExecution;

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
    println!("execute requested");
    query
        .into_iter()
        .for_each(|req| {
            for (index, expected_color) in req.tiles.iter() {
                if let Some(tile_entity) = tiles_by_index.get(index) {
                    if let Some(color) = tiles.get(*tile_entity).ok() {
                        if color.is_matching(expected_color) {
                            println!("score += 1");
                            score.0 += 1;
                        }
                    }
                }
            }
            println!("action combine!");
            // TODO: proper scoring
        });
}

fn finish_execution(
    mut writer: MessageWriter<FinishedExecution>
) {
    writer.write(FinishedExecution);
}

