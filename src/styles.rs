use bevy::prelude::*;

use crate::simple_button::SimpleButtonLabel;

pub struct StylePlugin;

#[derive(Resource)]
pub struct UiStyles {
    pub tooltip: TooltipStyle,
    pub body_font: TextFont,
}

pub struct TooltipStyle {
    pub background_color: Color,
    pub text_color: Color,
    pub font: TextFont,
}

impl Plugin for StylePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, style_for_simple_button);
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let font = asset_server.load("fonts/fragmentcore.otf");
	let small_text_font = TextFont {
    	font: font.clone(),
        font_size: 12.0,
    	..default()
    };

    let medium_text_font = TextFont {
        font,
        font_size: 18.0,
        ..default()
    };

    commands.insert_resource(UiStyles {
        tooltip: TooltipStyle { 
            background_color: Color::linear_rgba(0., 0., 0.7, 0.4),
            text_color: Color::WHITE,
            font: small_text_font,
        },
        body_font: medium_text_font,
    });
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
                font_size: 33.0,
                 ..default()
            };

            commands.entity(label).try_insert(font);
        });
}

