use bevy::prelude::*;

use crate::{press::PressArea, scale_on_touch::ScaleOnTouch, touch::TouchArea};

#[derive(Component)]
pub struct SimpleButton;

#[derive(Component)]
pub struct SimpleButtonLabel;

impl SimpleButton {
    pub fn create<T: Component>(marker: T, text: &'static str, position: Vec2) -> impl Bundle {
        let area = Vec2::new(96., 48.);
        (
            marker, 
            SimpleButton,
            TouchArea {
                area,
            },
            ScaleOnTouch(1.1),
            PressArea,
            Sprite::from_color(Color::linear_rgba(1., 0., 0., 0.2), area),
            Transform::from_xyz(position.x, position.y, 0.),
            children![(
                SimpleButtonLabel,
                Transform::from_xyz(0., 0., 0.),
                Text2d::new(text),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            )]
        )
    }
}

