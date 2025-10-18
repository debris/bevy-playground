use bevy::prelude::*;

use crate::{grid::GridRefreshRequest, press::PressState};

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

#[derive(Component)]
pub struct ActionCombine;

fn action_combine(
    query: Query<&PressState, (With<ActionCombine>, Changed<PressState>)>,
) {
    query
        .into_iter()
        .filter(|state| {
            *state == &PressState::JustReleased
        })
        .for_each(|_| {
            println!("action combine!");
        });
}

