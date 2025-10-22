use std::time::Duration;

use bevy::prelude::*;

pub struct AnimatedSpritePlugin;

impl Plugin for AnimatedSpritePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, setup_animated_sprite)
            .add_systems(Update, animate_sprite);
    }
}

#[derive(Component)]
pub struct AnimatedSprite {
    pub filename: String,
    pub tilesize: UVec2,
    // only horizontal supported
    pub frames: u32,
    pub custom_size: Option<Vec2>,
    pub start_frame: Option<usize>,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup_animated_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    animated: Query<(Entity, &AnimatedSprite), Added<AnimatedSprite>>,
) {
    animated
        .into_iter()
        .for_each(|(entity, animated_sprite)| {
            let texture = asset_server.load(&animated_sprite.filename);
            let layout = TextureAtlasLayout::from_grid(animated_sprite.tilesize, animated_sprite.frames, 1, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);

            let mut sprite = Sprite::from_atlas_image(
                texture, 
                TextureAtlas { 
                    layout: texture_atlas_layout,
                    index: animated_sprite.start_frame.unwrap_or(0),
                }
            );

            sprite.custom_size = animated_sprite.custom_size;

            let timer = Timer::from_seconds(0.15, TimerMode::Repeating);

            commands
                .entity(entity)
                .try_insert((
                    sprite,
                    AnimationTimer(timer),
                ));
        });
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut Sprite, &AnimatedSprite)>,
) {
    for (mut timer, mut sprite, animated_sprite) in &mut query {
        timer.tick(time.delta());

        if timer.is_finished() && let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = if atlas.index == animated_sprite.frames as usize - 1 {
                0
            } else {
                atlas.index + 1
            };
        }
    }
}
