use bevy::prelude::*;

use crate::simple_button::SimpleButtonLabel;

pub struct StylePlugin;

impl Plugin for StylePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, style_for_simple_button);
    }
}

fn style_for_simple_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    labels: Query<Entity, (Added<SimpleButtonLabel>, Without<TextFont>)>,
) {
    labels
        .iter()
        .for_each(|label| {
            let font = TextFont {
                font: asset_server.load("fonts/fragmentcore.otf"),
                font_size: 20.0,
                 ..default()
            };

            commands.entity(label).try_insert(font);
        });
}

