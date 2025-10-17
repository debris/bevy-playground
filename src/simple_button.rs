use bevy::prelude::*;

use crate::{press::PressArea, touch::TouchArea};

pub struct SimpleButtonPlugin;

#[derive(Component)]
pub struct SimpleButton {
    text: &'static str,
    position: Vec2,

}

impl SimpleButton {
    pub fn create<T: Component>(marker: T, text: &'static str, position: Vec2) -> impl Bundle {
        let area = Vec2::new(96., 64.);
        (
            marker, 
            SimpleButton {
                text,
                position,
            },
            TouchArea {
                area,
            },
            PressArea,
            Sprite::from_color(Color::linear_rgba(1., 0., 0., 0.2), area),
            Transform::from_xyz(position.x, position.y, 0.),
            children![(
                Transform::from_xyz(0., 0., 0.),
                Text2d::new(text),
                TextFont {
                    //font: asset_server.load("fonts/fragmentcore.otf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            )]
        )
    }
}

