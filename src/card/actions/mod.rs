use bevy::prelude::*;
use bevy_write_after::{self, MessagePool, GlobalMessagePool};

use crate::{enemy::Enemy, grid::{GridTile, GridTileByIndex, GridTileColor}, healthbar::Health};
use crate::score::Score;

use super::{CardIndex, CardRequirement};

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
            .add_message::<ActionMessagePoolEmpty>()
            .add_message::<DamageEnemy>()
            .add_message::<ExecuteActions>()
            .add_message::<FinishedExecution>()
            .add_systems(Startup, setup_message_pool)
            .add_systems(Update, action_combine.in_set(ActionSet::Main).run_if(on_message::<ExecuteActions>))
            .add_systems(Update, damage_enemy.in_set(ActionSet::Main))
            .add_systems(Update, finish_execution.in_set(ActionSet::Finish).run_if(on_message::<ActionMessagePoolEmpty>));
    }
}

#[derive(Message, Default)]
pub struct ExecuteActions;

#[derive(Message)]
pub struct FinishedExecution;

/// Combine the total value of all matching squares.
#[derive(Component)]
pub struct ActionCombine;

#[derive(Component)]
pub struct ActionMessagePool;

#[derive(Message, Clone)]
pub struct ActionMessagePoolEmpty;

fn setup_message_pool(
    mut commands: Commands,
) {

    let mut pool = MessagePool::default();
    pool.write_when_empty(ActionMessagePoolEmpty);

    // TODO: it's global, make it local per screen?
    commands.spawn((
        ActionMessagePool,
        pool,
    ));
}

#[derive(Message)]
pub struct DamageEnemy(pub u64);

fn action_combine(
    mut pool: Single<&mut MessagePool, With<ActionMessagePool>>,
    tiles_by_index: Single<&GridTileByIndex>,
    tiles: Query<&GridTileColor, With<GridTile>>,
    query: Query<(&CardIndex, &CardRequirement), With<ActionCombine>>,
) {
    println!("execute requested");
    query
        .into_iter()
        .for_each(|(i, req)| {
            let mut card_points = 0;
            for (index, expected_color) in req.tiles.iter() {
                if let Some(tile_entity) = tiles_by_index.get(index) {
                    if let Some(color) = tiles.get(*tile_entity).ok() {
                        if color.is_matching(expected_color) {
                            card_points += 1;
                        }
                    }
                }
            }
            
            pool.write_after(DamageEnemy(card_points), i.0 as f32);
            println!("action combine!");
        });
}

fn damage_enemy(
    mut health: Single<&mut Health, With<Enemy>>,
    mut reader: MessageReader<DamageEnemy>
) {
    for damage in reader.read() {
        health.0 -= damage.0;
    }
}

fn finish_execution(
    mut pool: Single<&mut MessagePool, With<GlobalMessagePool>>,
) {
    println!("empty message pool");
    pool.write_after(FinishedExecution, 1.0);
}

