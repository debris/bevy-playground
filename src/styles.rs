use bevy::prelude::*;

pub struct StylePlugin;

#[derive(Resource)]
pub struct UiStyles {
    pub tooltip: TooltipStyle,
}

pub struct TooltipStyle {
    pub background_color: Color,
    pub text_color: Color,
    pub font: TextFont,
    pub appear_delay: f32,
    pub appear_animation_time: f32,

}

impl Plugin for StylePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let font = asset_server.load("fonts/fragmentcore.otf");
	let small_text_font = TextFont {
    	font,
        font_size: 12.0,
    	..default()
    };

    commands.insert_resource(UiStyles {
        tooltip: TooltipStyle { 
            background_color: Color::linear_rgba(0., 0., 0.7, 0.4),
            text_color: Color::WHITE,
            font: small_text_font,
            appear_delay: 0.5,
            appear_animation_time: 0.2,
        }
    });
}

