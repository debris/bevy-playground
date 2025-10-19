use bevy::{platform::collections::HashMap, prelude::*};

use crate::grid::{GridConfig, GridTile, GridTileColor, Index};

#[derive(Message)]
pub struct GridHighlightRequest {
    pub tiles: HashMap<Index, GridTileColor>,
}

pub struct GridHighlightPlugin;

#[derive(Component)]
pub struct GridTileHighlight;

impl Plugin for GridHighlightPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<GridHighlightRequest>()
            .add_systems(Update, highlight_grid);
    }
}

fn highlight_grid(
    mut commands: Commands,
    config: Res<GridConfig>,
    mut requests: MessageReader<GridHighlightRequest>,
    existing: Query<Entity, With<GridTileHighlight>>,
    tiles: Query<(&GlobalTransform, &Index), With<GridTile>>,
) {
    // only the last highlight really matters
    let request = match requests
        .read()
        .into_iter()
        .last() {
            Some(r) => r,
            None => return,
    };

    // TODO: add tile by index
    existing
        .into_iter()
        .for_each(|entity| {
            commands.entity(entity).despawn();
        });

    for (transform, index) in &tiles {
        if let Some(color) = request.tiles.get(index) {
            let bundle = (
                GridTileHighlight,
                Sprite::from_color(color.color().with_alpha(0.2), config.tile_size),
                Transform::from_translation(transform.translation())
            );
            commands.spawn(bundle);

        }
    }

    requests.clear();
}

