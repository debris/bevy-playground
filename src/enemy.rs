use bevy::prelude::*;
use crate::{core::prelude::*, healthbar::{Health, Healthbar, HealthbarYOffset, MaxHealth}, tooltip_on_touch::TooltipOnTouch};

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, setup_enemy);
    }
}

fn setup_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemies: Query<Entity, Added<Enemy>>,
) {
    enemies
        .into_iter()
        .for_each(|enemy| {
            let mut sprite = Sprite::from_image(asset_server.load("enemies/enemy0.png"));
            sprite.custom_size = Some(Vec2::splat(128.));
            commands
                .entity(enemy)
                .try_insert((
                    Health(100),
                    MaxHealth(100),
                    sprite,
                    TouchArea {
                        area: Vec2::splat(128.)
                    },
                    TooltipOnTouch("Enemy".to_owned()),
                    Healthbar,
                    HealthbarYOffset(-64.),
                ));
        });
}

