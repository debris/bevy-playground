use bevy::prelude::*;

use crate::grid::{GridTile, Index};

#[derive(Message)]
pub struct GridHighlightRequest {
    pub tiles: Vec<Index>,
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

    for index_to_highlight in &request.tiles {
        for (transform, index) in &tiles {
            if index_to_highlight == index {
                let bundle = (
                    GridTileHighlight,
                    Sprite::from_color(Color::linear_rgba(0., 0., 1., 0.5), Vec2::splat(96.)),
                    Transform::from_translation(transform.translation())
                );
                commands.spawn(bundle);
            }
        }
    }

    requests.clear();
}

