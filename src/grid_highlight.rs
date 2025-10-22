use bevy::{platform::collections::HashMap, prelude::*, transform};

use crate::{animated_sprite::AnimatedSprite, grid::{GridConfig, GridTile, GridTileByIndex, GridTileColor, Index}, layout::ContentView};

#[derive(Message)]
pub struct GridHighlightRequest;

/// Single state component
#[derive(Component, Default)]
pub struct GridHighlightsState {
    pub highlights_by_side: HashMap<GridTileHighlightSide, HashMap<Index, GridTileColor>>,
}

pub struct GridHighlightPlugin;

#[derive(Component)]
pub struct GridTileHighlight;

#[derive(PartialEq, Hash, Eq)]
pub enum GridTileHighlightSide {
    Left,
    Bottom,
    Right,
}

impl GridTileHighlightSide {
    fn rotation(&self) -> f32 {
        match *self {
            GridTileHighlightSide::Left => -90.0f32,
            GridTileHighlightSide::Bottom => 0.0f32,
            GridTileHighlightSide::Right => 90.0f32,
        }.to_radians()
    }
}

impl Plugin for GridHighlightPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_grid_highlight)
            .add_message::<GridHighlightRequest>()
            .add_systems(Update, highlight_grid.run_if(on_message::<GridHighlightRequest>));
    }
}

fn setup_grid_highlight(
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("Grid Highlights"),
        GridHighlightsState::default(),
    ));
}

fn highlight_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    content_view: Single<Entity, With<ContentView>>,
    config: Res<GridConfig>,
    state: Single<&GridHighlightsState>,
    tile_by_index: Single<&GridTileByIndex>,
    tiles: Query<(&GlobalTransform, &GridTileColor), With<GridTile>>,
    existing: Query<Entity, With<GridTileHighlight>>,
) {
    existing
        .into_iter()
        .for_each(|entity| {
            commands.entity(entity).despawn();
        });

    for (side, indexes) in &state.highlights_by_side {
        for (index, expected_color) in indexes {
            if let Some(tile_entity) = tile_by_index.get(index) {
                if let Some((_transform, tile_color)) = tiles.get(*tile_entity).ok() {

                    let t = config.xy_position(index);
                    let mut transform = Transform::from_xyz(t.x, t.y, 0.);
                    transform.rotate_z(side.rotation());

                    let filename = if expected_color.is_matching(tile_color) {
                        expected_color.highlight_tile_filled()
                    } else {
                        expected_color.highlight_tile_empty()
                    };
        
                    let mut sprite = Sprite::from_image(asset_server.load(filename));
                    sprite.custom_size = Some(config.tile_size);

                    let bundle = (
                        GridTileHighlight,
                        sprite,
                        transform,
                    );


                    //let bundle = (
                        //GridTileHighlight,
                        //AnimatedSprite {
                            //filename: filename.into(),
                            //tilesize: UVec2::splat(32),
                            //frames: 4,
                            //custom_size: Some(config.tile_size),
                            //start_frame: None //Some((index.x + index.y) % 4),
                        //},
                        //transform,
                    //);

                    // TODO: wrap it in with_children
                    commands
                        .entity(*content_view)
                        .with_child(bundle);

                }
                
            }
        }
    }
}

