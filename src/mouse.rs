use bevy::prelude::*;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, on_mouse_move)
            .insert_resource(MouseState::Idle { 
                duration: 0. 
            });
    }
}

#[derive(Resource, Debug)]
pub enum MouseState {
    Moving,
    Idle {
        duration: f32,
    }
}

impl MouseState {
    pub fn is_idle_at_least(&self, time: f32) -> bool {
        match self {
            MouseState::Idle { duration } if *duration >= time => {
                true
            },
            _ => false
        }
    }
}

fn on_mouse_move(
    time: Res<Time>,
    mut state: ResMut<MouseState>,
    mut cursor_moves: MessageReader<CursorMoved>,
) {

    if cursor_moves.is_empty() {
        match &mut *state {
            MouseState::Moving => {
                *state = MouseState::Idle { 
                    duration: time.delta_secs(),
                }
            },
            MouseState::Idle { duration } => {
                *duration += time.delta_secs();
            }
        }
        return 
    }

    cursor_moves.clear();

    *state = MouseState::Moving;
}

