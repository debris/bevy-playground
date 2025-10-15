use bevy::{input::common_conditions::{input_just_pressed, input_pressed, input_just_released}, prelude::*};
use bevy_rand::prelude::*;
use rand::{Rng, distr::{Distribution, StandardUniform}};

use crate::{tooltip, touch};


pub struct GridPlugin {
    pub config: GridConfig
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct GridConfig {
    pub dimensions: (usize, usize),
    pub tile_size: Vec2,
    pub movement_speed: f32,
}

#[derive(Component)]
pub struct GridTile;

#[derive(Component, Clone, Copy)]
pub enum GridTileColor {
    Green,
    Red,
}

#[derive(Resource)]
struct PickedGridTile(Option<Entity>);


#[derive(Component, Clone, Copy)]
pub struct Index {
    pub x: usize,
    pub y: usize,
}

impl Index {
    pub fn new(x: usize, y: usize) -> Self {
        Index {
            x,
            y,
        }
    }

    pub fn assign(&mut self, other: &Self) {
        self.x = other.x;
        self.y = other.y;
    }
}


fn is_picked(picked: Res<PickedGridTile>) -> bool {
    picked.0.is_some()
}

impl Distribution<GridTileColor> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GridTileColor {
        match rng.random_range(0u32..2) {
            0 => GridTileColor::Green,
            _ => GridTileColor::Red,
        }
    }
}

impl GridTileColor {
    pub fn sprite_name(&self) -> &'static str {
        match *self {
            GridTileColor::Green => "green_tile.png",
            GridTileColor::Red => "red_tile.png",
        }
    }

    pub fn tooltip_text(&self) -> &'static str {
        match *self {
            GridTileColor::Green => "Green Tile",
            GridTileColor::Red => "Red Tile",
        }
    }
}

impl GridPlugin {
    pub fn new(config: GridConfig) -> Self {
        GridPlugin {
            config
        }
    }
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, handle_pick.run_if(input_just_pressed(MouseButton::Left)))
            .add_systems(Update, handle_drag.run_if(input_pressed(MouseButton::Left)))
            .add_systems(Update, handle_release.run_if(input_just_released(MouseButton::Left)))
            .add_systems(Update, update_positions)
            .add_systems(Update, swap.run_if(is_picked))
            .insert_resource(self.config)
            .insert_resource(PickedGridTile(None));
    }
}


fn xy_position(dimensions: (usize, usize), i: usize, j: usize, tile_size: Vec2) -> Vec2 {
    return vec2((dimensions.0 - 1) as f32, (dimensions.1 - 1) as f32) * tile_size * (-0.5) + vec2(i as f32, j as f32) * tile_size
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<GridConfig>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {

    for i in 0..config.dimensions.0 {
        for j in 0..config.dimensions.1 {
            let xy = xy_position(config.dimensions, i, j, config.tile_size);
            let tile_color: GridTileColor = rng.random();
            commands.spawn((
                Name::new("Grid Tile"),
                tile_color,
                Sprite::from_image(asset_server.load(tile_color.sprite_name())),
                Transform::from_xyz(xy.x, xy.y, 0.),
                touch::Touchable {
                    area: config.tile_size,
                    scale: Some(2.0),
                    ..default()
                },
                Index::new(i, j),
                GridTile,
                tooltip::TooltipData {
                    text: tile_color.tooltip_text(),
                    area: Vec2::new(128., 64.),
                }
            ));
        }
    }
}

fn handle_pick(
    tiles: Query<(Entity, &touch::Touchable), With<GridTile>>,
    mut picked: ResMut<PickedGridTile>,
) {

    for (entity, touchable) in &tiles {
        if touchable.touched {
            picked.0 = Some(entity);
            return
        }
    }
    
}

fn handle_drag(
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut tiles: Query<&mut Transform, With<GridTile>>,
    picked: ResMut<PickedGridTile>,
) {
    let entity = match picked.0 {
        Some(entity) => entity,
        None => return
    };

    let world_pos = match touch::mouse_position(window, camera) {
        Some(pos) => pos,
        None => return
    };
    

    if let Ok(mut transform) = tiles.get_mut(entity) {
        transform.translation.x = world_pos.x;
        transform.translation.y = world_pos.y;
    }
}

fn handle_release(
    mut picked: ResMut<PickedGridTile>,
) {
    picked.0 = None;
}

fn is_this_picked(
    entity : &Entity,
    picked: &PickedGridTile,
) -> bool {
    match picked.0 {
        Some(e) => &e == entity,
        None => false
    }
}

fn update_positions(
    time: Res<Time>,
    mut tiles: Query<(Entity, &mut Transform, &mut Index), With<GridTile>>,
    config: Res<GridConfig>,
    picked: Res<PickedGridTile>,
) {
    for (entity, mut transform, index) in &mut tiles {
        let delta = time.delta_secs();

        if is_this_picked(&entity, &picked) {
            continue;
        }

        let current_pos = transform.translation.truncate();

        let target_pos = xy_position(config.dimensions, index.x, index.y, config.tile_size);


        let direction = target_pos - current_pos;
        let distance = direction.length();

        if distance <= config.movement_speed * delta {
            transform.translation.x = target_pos.x;
            transform.translation.y = target_pos.y;
        } else {
            let step = direction.normalize() * config.movement_speed * delta;
            assert!(!step.is_nan(), "step should not be NaN");
            transform.translation.x += step.x;
            transform.translation.y += step.y;
        }
    }
}

fn swap(
    mut tiles: Query<(Entity, &touch::Touchable, &mut Index), With<GridTile>>,
    mut picked: ResMut<PickedGridTile>,
) {
    // get a sprite below cursor which is not our current Dragged
    let entity = || -> Option<Entity> {
        for (entity, touchable, _) in &tiles {
            if touchable.touched && !is_this_picked(&entity, &picked) {
                return Some(entity)
            }
        }
        return None
    }();

    match (entity, picked.0) {
        (Some(entity), Some(d)) => {
            if let Ok(mut ok) = tiles.get_many_mut([entity, d]) {
                let index_a = ok[0].2.clone();
                let index_b = ok[1].2.clone();
                ok[0].2.assign(&index_b);
                ok[1].2.assign(&index_a);

                picked.0 = None;
            }
        },
        _ => (),
    }

}

