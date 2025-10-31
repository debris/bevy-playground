use std::collections::VecDeque;

use bevy::prelude::*;

pub enum Task {
    DealDamage {
        damage: u32,
    }
}

pub struct TaskPlugin;

#[derive(Resource)]
pub struct Tasks {
    tasks: VecDeque<(Timer, Task)>,
}

fn execute_task(
    time: Res<Time>,
    tasks: ResMut<Tasks>,
) {

}

