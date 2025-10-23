use bevy::prelude::*;

use crate::{card::{actions::{ExecuteActions, FinishedExecution}, CardRedrawRequest}, grid::GridRefreshRequest};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<StartCast>()
            .init_state::<CastState>()
            .add_systems(PostUpdate, start_cast
                .run_if(on_message::<StartCast>)
                .run_if(in_state(CastState::None))
            )
            .add_systems(PostUpdate, post_execute
                .run_if(on_message::<FinishedExecution>)
                .run_if(in_state(CastState::ExecuteActions))
            );
    }
}

#[derive(Message, Default)]
pub struct StartCast;

#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum CastState {
    #[default]
    None,
    ExecuteActions,
}

/// PostUpdate call that changes that moves to the next state with every update.
fn start_cast(
    mut next_cast_state: ResMut<NextState<CastState>>,
    mut writer: MessageWriter<ExecuteActions>,
) {
    next_cast_state.set(CastState::ExecuteActions);
    writer.write(ExecuteActions);
}

fn post_execute(
    mut next_cast_state: ResMut<NextState<CastState>>,
    mut grid_writer: MessageWriter<GridRefreshRequest>,
    mut card_writer: MessageWriter<CardRedrawRequest>,
) {
    next_cast_state.set(CastState::None);
    grid_writer.write(GridRefreshRequest);
    card_writer.write(CardRedrawRequest);
}

