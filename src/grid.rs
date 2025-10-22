use bevy::{input::common_conditions::{input_just_pressed, input_just_released, input_pressed}, platform::collections::HashMap, prelude::*};
use bevy_rand::prelude::*;
use rand::{Rng, distr::{Distribution, StandardUniform}};

use crate::{grid_highlight::GridHighlightRequest, mouse::MousePosition, scale_on_touch, tooltip_on_touch::TooltipOnTouch, touch::{self, TouchState}};

#[derive(Message, Default)]
pub struct GridRefreshRequest;

pub struct GridPlugin {
    pub config: GridConfig
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct GridConfig {
    pub dimensions: (usize, usize),
    pub tile_size: Vec2,
    pub movement_speed: f32,
}

impl GridConfig {
    pub fn grid_width(self) -> f32 {
        self.tile_size.x * self.dimensions.0 as f32
    }

    pub fn grid_height(&self) -> f32 {
        self.tile_size.y * self.dimensions.1 as f32
    }

    pub fn xy_position(&self, index: &Index) -> Vec2 {
        return vec2((self.dimensions.0 - 1) as f32, (self.dimensions.1 - 1) as f32) * self.tile_size * (-0.5) + vec2(index.x as f32, index.y as f32) * self.tile_size
    }
}

#[derive(Component)]
pub struct GridTile;

#[derive(Component, Clone, Copy, PartialEq)]
pub enum GridTileColor {
    Green,
    Red,
    Blue,
    Brown,
    Multicolor,
}

impl GridTileColor {
    pub fn is_matching(&self, other: &Self) -> bool {
        self == other ||
        *self == GridTileColor::Multicolor || 
        *other == GridTileColor::Multicolor
    }
}

#[derive(Resource)]
struct PickedGridTile(Option<Entity>);

#[derive(Component)]
pub struct Grid;

#[derive(Component)]
pub struct GridData {
    moves_made: Vec<GridMove>,
    moves_limit: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct GridTileByIndex(pub HashMap<Index, Entity>);

pub struct GridMove {
    tile_a: (Index, GridTileColor),
    tile_b: (Index, GridTileColor),
}

#[derive(Component)]
pub struct GridMovesLabel;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
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
        match rng.random_range(0u32..5) {
            0 => GridTileColor::Green,
            1 => GridTileColor::Red,
            2 => GridTileColor::Blue,
            3 => GridTileColor::Brown,
            _ => GridTileColor::Multicolor,
        }
    }
}

impl GridTileColor {
    pub fn sprite_name(&self) -> &'static str {
        match *self {
            GridTileColor::Green => "green_tile.png",
            GridTileColor::Red => "red_tile.png",
            GridTileColor::Blue => "blue_tile.png",
            GridTileColor::Brown => "brown_tile.png",
            GridTileColor::Multicolor => "multicolor_tile.png",
        }
    }

    pub fn highlight_tile_empty(&self) -> &'static str {
        match *self {
            GridTileColor::Blue => "blue_expect_empty_na.png",
            GridTileColor::Green => "green_expect_empty_na.png",
            _ => unimplemented!(),
        }
    }

    pub fn highlight_tile_filled(&self) -> &'static str {
        match *self {
            GridTileColor::Blue => "blue_expect_filled_na.png",
            GridTileColor::Green => "green_expect_filled_na.png",
            _ => unimplemented!(),
        }
    }

    pub fn tooltip_text(&self) -> &'static str {
        match *self {
            GridTileColor::Green => "Green Tile",
            GridTileColor::Red => "Red Tile",
            GridTileColor::Blue => "Blue Tile",
            GridTileColor::Brown => "Brown Tile",
            GridTileColor::Multicolor => "Multicolor Tile",
        }
    }

    pub fn color(&self) -> Color {
        match *self {
            GridTileColor::Green => Color::linear_rgb(0., 1., 0.),
            GridTileColor::Red => Color::linear_rgb(1., 0., 0.),
            GridTileColor::Blue => Color::linear_rgb(0., 0., 1.),
            GridTileColor::Brown => Color::linear_rgb(0.5, 0.5, 1.),
            GridTileColor::Multicolor => Color::linear_rgb(0., 0., 0.),
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
            .add_message::<GridRefreshRequest>()
            .add_systems(Update, add_grid_tiles)
            .add_systems(Update, handle_refresh_request.run_if(on_message::<GridRefreshRequest>))
            .add_systems(Update, handle_pick.run_if(input_just_pressed(MouseButton::Left)))
            .add_systems(Update, handle_drag.run_if(input_pressed(MouseButton::Left)))
            .add_systems(Update, handle_release.run_if(input_just_released(MouseButton::Left)))
            .add_systems(Update, update_positions)
            .add_systems(Update, swap.run_if(is_picked).run_if(touch::just_touched::<GridTile>))
            .add_systems(Update, update_grid_moves_label)
            .add_systems(Update, update_grid_tile_color)
            .insert_resource(self.config)
            .insert_resource(PickedGridTile(None));
    }
}

fn add_grid_tiles(
    mut commands: Commands,
    config: Res<GridConfig>,
    grids: Query<Entity, Added<Grid>>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    for grid in grids {
        let mut tile_by_index = HashMap::new();
        commands
            .entity(grid)
            .try_insert((
                Name::new("Grid"),
                GridData {
                    moves_made: vec![],
                    moves_limit: 3,
                }
            ))
            .with_children(|parent| {
                for i in 0..config.dimensions.0 {
                    for j in 0..config.dimensions.1 {
                        let index = Index::new(i, j);
                        let position = config.xy_position(&index);
                        let tile_color: GridTileColor = rng.random();
                        
                        let entity = parent.spawn((
                            GridTile,
                            Name::new("Grid Tile"),
                            Transform::from_xyz(position.x, position.y, 0.),
                            tile_color,
                            index,
                            touch::TouchArea {
                                area: config.tile_size,
                            },
                            scale_on_touch::ScaleOnTouch(2.0),
                            TooltipOnTouch(tile_color.tooltip_text().to_string())
                        )).id();

                        tile_by_index.insert(index, entity);
                    }
                }
            })
            .try_insert(GridTileByIndex(tile_by_index));
    }
}

fn update_grid_tile_color(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<GridConfig>,
    tiles: Query<(Entity, &GridTileColor), (With<GridTile>, Changed<GridTileColor>)>,
) {
    tiles
        .into_iter()
        .for_each(|(entity, tile_color)| {
            let mut sprite = Sprite::from_image(asset_server.load(tile_color.sprite_name()));
            sprite.custom_size = Some(config.tile_size);
            commands
                .entity(entity)
                .try_insert((
                    sprite,
                    TooltipOnTouch(tile_color.tooltip_text().to_string())
                ));
        });
}

fn handle_refresh_request(
    grids: Query<&mut GridData, With<Grid>>,
    tiles: Query<&mut GridTileColor, With<GridTileColor>>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    mut request: MessageWriter<GridHighlightRequest>,
) {
    println!("refreshed grid");

    tiles
        .into_iter()
        .for_each(|mut tile_color| {
            tile_color.set_if_neq(rng.random());
        });

    grids
        .into_iter()
        .for_each(|mut data| {
            data.moves_made.clear();
            data.moves_limit = 3;
            
        });

    request.write(GridHighlightRequest);
}

fn handle_pick(
    tiles: Query<(Entity, &touch::TouchState), With<GridTile>>,
    mut picked: ResMut<PickedGridTile>,
) {

    for (entity, state) in &tiles {
        if state.is_touching() {
            picked.0 = Some(entity);
            return
        }
    }
    
}

fn handle_drag(
    mouse_position: Res<MousePosition>,
    grid: Query<&GlobalTransform, With<Grid>>,
    mut tiles: Query<(&mut Transform, &ChildOf), With<GridTile>>,
    picked: ResMut<PickedGridTile>,
) {
    let entity = match picked.0 {
        Some(entity) => entity,
        None => return
    };

    let world_pos = mouse_position.0;
    

    if let Ok((mut transform, child_of)) = tiles.get_mut(entity) {
        let global_transform = grid.get(child_of.parent()).expect("must have a parent");
        transform.translation.x = world_pos.x - global_transform.translation().x;
        transform.translation.y = world_pos.y - global_transform.translation().y;
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
    mut tiles: Query<(Entity, &mut Transform, &Index), With<GridTile>>,
    config: Res<GridConfig>,
    picked: Res<PickedGridTile>,
) {
    for (entity, mut transform, index) in &mut tiles {
        let delta = time.delta_secs();

        if is_this_picked(&entity, &picked) {
            continue;
        }

        let current_pos = transform.translation.truncate();

        let target_pos = config.xy_position(&index);


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
    mut grid: Single<(&mut GridData, &mut GridTileByIndex)>,
    mut tiles: Query<(Entity, &touch::TouchState, &mut Index, &GridTileColor), (With<GridTile>, Changed<TouchState>)>,
    mut picked: ResMut<PickedGridTile>,
    mut request: MessageWriter<GridHighlightRequest>,
) {
    println!("swap");

    // get a sprite below cursor which is not our current Dragged
    let entity = || -> Option<Entity> {
        for (entity, touch_state, _, _) in &tiles {
            if touch_state.is_just_touched() && !is_this_picked(&entity, &picked) {
                return Some(entity)
            }
        }
        return None
    }();

    match (entity, picked.0) {
        (Some(entity), Some(d)) => {
            if let Ok(mut ok) = tiles.get_many_mut([entity, d]) {
                let (ref mut grid, ref mut tiles_by_index) = *grid;

                if grid.moves_made.len() == grid.moves_limit {
                    return
                }
                //grid.moves_made += 1;

                let index_a = ok[0].2.clone();
                let index_b = ok[1].2.clone();
                ok[0].2.assign(&index_b);
                ok[1].2.assign(&index_a);
                tiles_by_index.insert(index_a, ok[1].0);
                tiles_by_index.insert(index_b, ok[0].0);

                let grid_move = GridMove {
                    tile_a: (index_a, ok[0].3.clone()),
                    tile_b: (index_b, ok[1].3.clone())
                };
                grid.moves_made.push(grid_move);
                
                picked.0 = None;
                request.write(GridHighlightRequest);

            }
        },
        _ => (),
    }
}

// TODO: modify so it doesn't use ChildOf
fn update_grid_moves_label(
    grid: Single<&GridData>,
    mut labels: Query<&mut Text2d, With<GridMovesLabel>>,
) {
    for mut text in &mut labels {
        *text = Text2d::new(format!("moves {}/{}", grid.moves_made.len(), grid.moves_limit));
    }
}

