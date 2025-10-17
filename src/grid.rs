use bevy::{input::common_conditions::{input_just_pressed, input_just_released, input_pressed}, prelude::*, sprite::Anchor, text::TextBounds};
use bevy_rand::prelude::*;
use rand::{Rng, distr::{Distribution, StandardUniform}};

use crate::{scale_on_touch, styles::UiStyles, tooltip, touch::{self, TouchState}};

#[derive(Message)]
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
}

#[derive(Component)]
pub struct GridTile;

impl GridTile {
    fn create(
        asset_server: &AssetServer,
        tile_color: GridTileColor,
        position: Vec2, 
        size: Vec2,
        index: Index,

    ) -> impl Bundle {
        let mut sprite = Sprite::from_image(asset_server.load(tile_color.sprite_name()));
        sprite.custom_size = Some(size);

        (
            GridTile,
            Name::new("Grid Tile"),
            sprite,
            Transform::from_xyz(position.x, position.y, 0.),
            tile_color,
            index,
            touch::Touchable {
                area: size,
            },
            scale_on_touch::ScaleOnTouch(2.0),
            tooltip::Tooltip {
                text: tile_color.tooltip_text(),
                area: Vec2::new(128., 64.),
            }
        )
    }
}

#[derive(Component, Clone, Copy)]
pub enum GridTileColor {
    Green,
    Red,
    Blue,
    Brown,
    Multicolor,
}

#[derive(Resource)]
struct PickedGridTile(Option<Entity>);

#[derive(Component)]
pub struct Grid;

impl Grid {
    pub fn create(
        position: Vec2,
    ) -> impl Bundle {
        (
            Name::new("Grid"),
            Grid,
            Transform::from_xyz(position.x, position.y, 0.),
            Visibility::Inherited,
            GridData {
                moves_made: vec![],
                moves_limit: 3,
            }
        )
    }
}

#[derive(Component)]
pub struct GridData {
    moves_made: Vec<GridMove>,
    moves_limit: usize,
}

pub struct GridMove {
    tile_a: (Index, GridTileColor),
    tile_b: (Index, GridTileColor),
}

#[derive(Component)]
pub struct GridSwapLimitLabel;

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

    pub fn tooltip_text(&self) -> &'static str {
        match *self {
            GridTileColor::Green => "Green Tile",
            GridTileColor::Red => "Red Tile",
            GridTileColor::Blue => "Blue Tile",
            GridTileColor::Brown => "Brown Tile",
            GridTileColor::Multicolor => "Multicolor Tile",
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
            .add_systems(Update, add_grid_moves_limit_label)
            .add_systems(Update, handle_refresh_request)
            .add_systems(Update, handle_pick.run_if(input_just_pressed(MouseButton::Left)))
            .add_systems(Update, handle_drag.run_if(input_pressed(MouseButton::Left)))
            .add_systems(Update, handle_release.run_if(input_just_released(MouseButton::Left)))
            .add_systems(Update, update_positions)
            .add_systems(Update, swap.run_if(is_picked).run_if(touch::just_touched::<GridTile>))
            .add_systems(Update, update_swap_limit_label)
            .insert_resource(self.config)
            .insert_resource(PickedGridTile(None));
    }
}


fn xy_position(dimensions: (usize, usize), i: usize, j: usize, tile_size: Vec2) -> Vec2 {
    return vec2((dimensions.0 - 1) as f32, (dimensions.1 - 1) as f32) * tile_size * (-0.5) + vec2(i as f32, j as f32) * tile_size
}

fn add_grid_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<GridConfig>,
    grids: Query<Entity, Added<Grid>>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    for grid in grids {
        commands
            .entity(grid)
            .with_children(|parent| {
                for i in 0..config.dimensions.0 {
                    for j in 0..config.dimensions.1 {
                        let xy = xy_position(config.dimensions, i, j, config.tile_size);
                        let tile_color: GridTileColor = rng.random();
                        let mut sprite = Sprite::from_image(asset_server.load(tile_color.sprite_name()));
                        sprite.custom_size = Some(config.tile_size);
                        let index = Index::new(i, j);
                        
                        parent.spawn(
                            GridTile::create(
                                &asset_server, 
                                tile_color, 
                                xy, 
                                config.tile_size, 
                                index
                            )
                        );
                    }
                }
            });
    }
}

fn add_grid_moves_limit_label(
    mut commands: Commands,
    config: Res<GridConfig>,
    style: Res<UiStyles>,
    grids: Query<Entity, Added<Grid>>,
) {
    for grid in grids {
        commands
            .entity(grid)
            .with_children(|parent| {
                parent.spawn((
                    Name::new("Grid Swap Limit Label"),
                    GridSwapLimitLabel,
                    Text2d::new(""),
                    style.body_font.clone(),
                    TextColor(Color::WHITE),
                    TextLayout::new(Justify::Right, LineBreak::WordBoundary),
                    TextBounds::from(Vec2::new(config.grid_width(), style.body_font.font_size)),
                    Transform::from_translation(Vec3::new(0., config.grid_height() / 2., 0.)),
                    Anchor::BOTTOM_CENTER,
                ));
            });
    }
}

fn handle_refresh_request(
    mut commands: Commands,
    mut refresh: MessageReader<GridRefreshRequest>,
    grids: Query<(Entity, &Transform), With<Grid>>,
) {
    if refresh.is_empty() {
        return;
    }

    // need to clear messages
    refresh.clear();

    for (grid, transform) in grids {
        commands.entity(grid).despawn();
        // TODO: this works for root entity
        // what if the grid entity is not root?
        commands.spawn(Grid::create(transform.translation.truncate()));
    }
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
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    grid: Query<&GlobalTransform, With<Grid>>,
    mut tiles: Query<(&mut Transform, &ChildOf), With<GridTile>>,
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
    mut grids: Query<&mut GridData>,
    mut tiles: Query<(Entity, &touch::TouchState, &mut Index, &ChildOf, &GridTileColor), (With<GridTile>, Changed<TouchState>)>,
    mut picked: ResMut<PickedGridTile>,
) {
    println!("swap");
    // get a sprite below cursor which is not our current Dragged
    let entity = || -> Option<Entity> {
        for (entity, touch_state, _, _, _) in &tiles {
            if touch_state.is_just_touched() && !is_this_picked(&entity, &picked) {
                return Some(entity)
            }
        }
        return None
    }();

    match (entity, picked.0) {
        (Some(entity), Some(d)) => {
            if let Ok(mut ok) = tiles.get_many_mut([entity, d]) {
                let mut grid = grids.get_mut(ok[0].3.parent()).expect("grid to be there");
                if grid.moves_made.len() == grid.moves_limit {
                    return
                }
                //grid.moves_made += 1;

                let index_a = ok[0].2.clone();
                let index_b = ok[1].2.clone();
                ok[0].2.assign(&index_b);
                ok[1].2.assign(&index_a);

                let grid_move = GridMove {
                    tile_a: (index_a, ok[0].4.clone()),
                    tile_b: (index_b, ok[1].4.clone())
                };
                grid.moves_made.push(grid_move);
                
                picked.0 = None;

            }
        },
        _ => (),
    }
}

// TODO: modify so it doesn't use ChildOf
fn update_swap_limit_label(
    grids: Query<&GridData>,
    mut labels: Query<(&mut Text2d, &ChildOf), With<GridSwapLimitLabel>>,
) {
    for (mut text, child_of) in &mut labels {
        let grid = grids.get(child_of.parent()).expect("grid to be there");
        *text = Text2d::new(format!("moves {}/{}", grid.moves_made.len(), grid.moves_limit));
    }
}

