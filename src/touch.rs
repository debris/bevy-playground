use bevy::prelude::*;

pub struct TouchPlugin;

#[derive(Component)]
pub struct Touchable {
    pub area: Vec2,
}

impl Default for Touchable {
    fn default() -> Self {
        Touchable { 
            area: Vec2::splat(0.0),
        }
    }
}

#[derive(Component, PartialEq, Clone)]
pub enum TouchState {
    None,
    JustTouched,
    Touching {
        duration: f32,
    },
}

impl TouchState {
    pub fn is_just_touched(&self) -> bool {
        *self == TouchState::JustTouched
    }

    pub fn is_touching(&self) -> bool {
        match *self {
            TouchState::JustTouched | TouchState::Touching { .. } => true,
            _ => false,
        }
    }
}

/// Returns true any component of gives types has been just touched.
pub fn just_touched<T: Component>(
    entities: Query<&TouchState, (With<T>, Changed<TouchState>)>
) -> bool {
    entities.iter()
        .map(|t| *t == TouchState::JustTouched)
        .find(|f| *f)
        .unwrap_or(false)
}

impl Plugin for TouchPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, add_touch_state)
            .add_systems(Update, detect_touch);
    }
}

fn add_touch_state(
    mut commands: Commands,
    entities: Query<Entity, Added<Touchable>>,
) {
    entities 
        .into_iter()
        .for_each(|touchable| {
            commands
                .entity(touchable)
                .insert(TouchState::None);
        });
}

pub fn mouse_position(
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let (camera, camera_transform) = *camera;
    window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
}

fn is_touching(entity_pos: &Vec2, size: &Vec2, mouse_pos: &Vec2) -> bool {
    let half = size * 0.5;

    let min = entity_pos - half;
    let max = entity_pos + half;

    (min.x..=max.x).contains(&mouse_pos.x) && (min.y..=max.y).contains(&mouse_pos.y)
}

fn detect_touch(
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    time: Res<Time>,
    mut entities: Query<(&GlobalTransform, &Touchable, &mut TouchState)>,
) {
    let world_pos = match mouse_position(window, camera) {
        Some(pos) => pos,
        None => return
    };

    for (transform, touchable, mut touch_state) in &mut entities {
        let entity_pos = transform.translation().truncate();
        let is_touching = is_touching(&entity_pos, &touchable.area, &world_pos);

        match (is_touching, touch_state.clone()) {
            (false, _) => {
                touch_state.set_if_neq(TouchState::None);
            },
            (true, TouchState::None) => {
                touch_state.set_if_neq(TouchState::JustTouched);
            }
            (true, TouchState::JustTouched) => {
                let state = TouchState::Touching {
                    duration: time.delta_secs(),
                };
                touch_state.set_if_neq(state);
            },
            (true, TouchState::Touching { duration }) => {
                let state = TouchState::Touching { 
                    duration: duration + time.delta_secs(),
                };
                touch_state.set_if_neq(state);
            },
        };
    }
}

