use bevy::prelude::*;

use crate::core::prelude::*;

pub struct ScaleOnTouchPlugin;

#[derive(Component)]
pub struct ScaleOnTouch(pub f32);

impl Default for ScaleOnTouch {
    fn default() -> Self {
        ScaleOnTouch(0.)
    }
}

impl Plugin for ScaleOnTouchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, scale_on_touch);
    }
}

fn scale_on_touch(
    entities: Query<(&mut Transform, &ScaleOnTouch, &TouchState), Changed<TouchState>>,
) {
    entities.into_iter()
        .for_each(|(mut transform, scale, state)| {
            match state {
                TouchState::JustTouched => {
                    transform.scale = Vec3::splat(scale.0);
                },
                TouchState::None => {
                    transform.scale = Vec3::splat(1.0);
                },
                _ => {},
            }
        });
}

