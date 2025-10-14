use bevy::prelude::*;

#[derive(Component)]
pub struct Touchable {
    pub touched: bool,
    pub area: Vec2,
    pub scale: Option<f32>,
}

impl Default for Touchable {
    fn default() -> Self {
        Touchable { 
            touched: false,
            area: Vec2::splat(0.0),
            scale: None,
        }
    }
}

pub fn mouse_position(
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let (camera, camera_transform) = *camera;
    window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
}

pub fn is_touching(entity_pos: &Vec2, size: &Vec2, mouse_pos: &Vec2) -> bool {
    let half = size * 0.5;

    let min = entity_pos - half;
    let max = entity_pos + half;

    (min.x..=max.x).contains(&mouse_pos.x) && (min.y..=max.y).contains(&mouse_pos.y)
}

pub fn touch(
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut entities: Query<(&Transform, &mut Touchable)>,
) {
    let world_pos = match mouse_position(window, camera) {
        Some(pos) => pos,
        None => return
    };

    for (transform, mut touchable) in &mut entities {
        let entity_pos = transform.translation.truncate();
        touchable.touched = is_touching(&entity_pos, &touchable.area, &world_pos);
    }
}

pub fn scale_on_hover(
    mut entities: Query<(&mut Transform, &Touchable)>,
) {
   
    for (mut transform, touchable) in &mut entities {
        if touchable.touched && let Some(scale) = touchable.scale {
            transform.scale = Vec3::splat(scale);
        } else {
            transform.scale = Vec3::splat(1.0);
        }
    }
}
