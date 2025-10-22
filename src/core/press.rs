use bevy::prelude::*;

use super::touch::{self, TouchState};

pub struct PressPlugin;

#[derive(Component)]
pub struct PressArea;

#[derive(Component, PartialEq)]
pub enum PressState {
    None,
    JustPressed,
    Pressed {
        duration: f32,
    },
    JustReleased,
}

impl Plugin for PressPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, add_state)
            .add_systems(PreUpdate, detect_press.after(touch::detect_touch));
    }
}

impl PressState {
    pub fn is_just_pressed(&self) -> bool {
        *self == PressState::JustPressed
    }

    pub fn is_pressed(&self) -> bool {
        match *self {
            PressState::JustPressed | PressState::Pressed { .. } => true,
            _ => false,
        }
    }
}

fn add_state(
    mut commands: Commands,
    entities: Query<Entity, Added<PressArea>>,
) {
    entities
        .into_iter()
        .for_each(|e| {
            commands.entity(e).try_insert(PressState::None);
        });
}

fn detect_press(
    mbi: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    entities: Query<(Entity, &TouchState, &mut PressState), Changed<TouchState>>,
) {
    entities
        .into_iter()
        .for_each(|(_, touch_state, mut press_state)| {
            match (touch_state.is_touching(), &mut *press_state, mbi.pressed(MouseButton::Left) || mbi.just_pressed(MouseButton::Left)) {
                (false, PressState::None, _) |
                (true, PressState::None, false) => {
                    // outside
                    // inside, not pressing
                },
                (false, PressState::JustPressed, _) |
                (false, PressState::Pressed { .. }, _) => {
                    // dragged press outside of bounds, cancel it
                    *press_state = PressState::None;
                },
                (false, PressState::JustReleased, _) => {
                    // out of bounds, does not matter
                    *press_state = PressState::None;
                }
                (true, PressState::Pressed { duration }, true) => {
                    // continue the press
                    let d = *duration + time.delta_secs();
                    press_state.set_if_neq(PressState::Pressed { 
                        duration: d,
                    });
                },
                (true, PressState::JustPressed, true) => {
                    // new press
                    *press_state = PressState::Pressed { 
                        duration: time.delta_secs(),
                    };
                },
                (true, _, true) => {
                    *press_state = PressState::JustPressed;
                },
                (true, PressState::JustPressed, false) |
                (true, PressState::Pressed { .. }, false) => {
                    *press_state = PressState::JustReleased;
                },
                (true, PressState::JustReleased, false) => {
                    *press_state = PressState::None;
                },
            }
        });
}

