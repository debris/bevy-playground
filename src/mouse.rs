use bevy::prelude::*;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, on_mouse_move)
            .add_systems(Update, update_mouse_position)
            .insert_resource(MouseState::Idle { 
                duration: 0. 
            })
            .insert_resource(MousePosition(Vec2::ZERO));
    }
}

#[derive(Resource, Debug)]
pub enum MouseState {
    Moving,
    Idle {
        duration: f32,
    }
}

#[derive(Resource, Deref, DerefMut, PartialEq)]
pub struct MousePosition(pub Vec2);

impl MouseState {
    pub fn is_idle_at_least(&self, time: f32) -> bool {
        match self {
            MouseState::Idle { duration } if *duration >= time => {
                true
            },
            _ => false
        }
    }

    pub fn idle_duration(&self) -> Option<f32> {
        match self {
            MouseState::Idle { duration } => Some(*duration),
            _ => None,
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

pub fn update_mouse_position(
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut mouse: ResMut<MousePosition>,
) {
    let (camera, camera_transform) = *camera;
    let position = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok());

    if let Some(position) = position {
        mouse.set_if_neq(MousePosition(position));
    }
}

