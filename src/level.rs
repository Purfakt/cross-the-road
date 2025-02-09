use std::fmt;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use serde::Deserialize;

use crate::car::spawn_car_spawner;
use crate::tilesheet::{spawn_tile, TextureName, Tileset, SCALED_TILE_SIZE};
use crate::world::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<AssetState>()
            .add_plugins(JsonAssetPlugin::<Levels>::new(&["levels.json"]))
            .add_systems(Startup, load_levels_asset)
            .add_systems(Update, setup_level.run_if(in_state(AssetState::Loading)));
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AssetState {
    #[default]
    Loading,
    Ready,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Resource)]

pub struct LevelHandle(Handle<Levels>);

#[derive(Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct Levels {
    levels: Vec<LevelSettings>,
}

#[derive(Deserialize, Clone)]
pub struct LevelSettings {
    pub level: usize,
    pub lanes: Vec<LaneSettings>,
}

#[derive(Deserialize, Clone)]
pub struct LaneSettings {
    pub direction: Direction,
    pub spawn_delays: Vec<f32>,
    pub speed: f32,
}

#[derive(Debug, Component)]
pub enum LaneType {
    Spawn,
    Car,
    End,
}

pub struct LaneCell {
    pub idx: usize,
}

#[derive(Component, Debug)]
pub struct Lane {
    pub lane_type: LaneType,
    pub idx: usize,
}

impl fmt::Display for LaneType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lane_type = match self {
            LaneType::Spawn => "Spawn",
            LaneType::Car => "Car",
            LaneType::End => "End",
        };
        write!(f, "{}Lane", lane_type)
    }
}

#[derive(Component)]
pub struct SpawnLane;

#[derive(Component)]
pub struct CarLane {
    pub direction: Direction,
}

#[derive(Component)]
pub struct EndLane;

#[derive(Component)]
pub struct Level {
    pub lanes_amount: usize,
}

fn load_levels_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<Levels> = asset_server.load("levels.json");
    commands.insert_resource(LevelHandle(handle));
}

fn setup_level(
    commands: Commands,
    tileset: Res<Tileset>,
    level_handle: Res<LevelHandle>,
    levels: ResMut<Assets<Levels>>,
    mut state: ResMut<NextState<AssetState>>,
) {
    if let Some(level) = levels.get(&level_handle.0) {
        let level0 = level.levels[0].clone();
        spawn_level(commands, tileset, level0);
        state.set(AssetState::Ready)
    }
}

fn spawn_level(mut commands: Commands, tileset: Res<Tileset>, level_settings: LevelSettings) {
    let lanes = commands
        .spawn((
            Visibility::Inherited,
            Transform::from_xyz(CELL_0_X, CELL_0_Y, -100.),
            Name::new("Lanes"),
        ))
        .id();

    let spawn_lanes = insert_spawn_lane(&mut commands, &tileset);

    commands.entity(lanes).add_child(spawn_lanes);

    let car_lanes = commands
        .spawn((
            Visibility::Inherited,
            Transform::from_xyz(0., 0., 0.),
            Name::new("CarLanes"),
        ))
        .id();

    let amount_car_lanes = level_settings.lanes.len();

    let remaining_lanes = LEVEL_ROWS - 1 - amount_car_lanes;

    for (idx, lane) in level_settings.lanes.iter().enumerate() {
        let direction = &lane.direction;
        let spawn_delays = &lane.spawn_delays;
        let speed = &lane.speed;
        let lane_entity = insert_car_lane(
            &mut commands,
            &tileset,
            idx + 1,
            direction,
            spawn_delays,
            *speed,
        );
        commands.entity(car_lanes).add_child(lane_entity);
    }

    commands.entity(lanes).add_child(car_lanes);

    let end_lanes = commands
        .spawn((
            Transform::from_xyz(0., 0., 0.),
            Visibility::Inherited,
            Name::new("EndLanes"),
        ))
        .id();

    for idx in amount_car_lanes + 1..remaining_lanes + amount_car_lanes + 1 {
        let lane_entity = insert_end_lane(&mut commands, &tileset, idx);
        commands.entity(end_lanes).add_child(lane_entity);
    }

    commands.entity(lanes).add_child(end_lanes);
}

fn insert_spawn_lane(commands: &mut Commands, tileset: &Res<Tileset>) -> Entity {
    let lane_entity = commands
        .spawn((
            SpawnLane,
            Transform::from_translation(Vec3::new(0., 0.0, 0.0)),
            Visibility::Inherited,
            Name::new("SpawnLane"),
        ))
        .id();

    for col in 0..LEVEL_COLS {
        let x = col as f32 * SCALED_TILE_SIZE;
        let tile_entity = spawn_tile(
            commands,
            tileset,
            &TextureName::Grass,
            &Vec3::new(x, 0., 100.),
            false,
        );

        commands
            .entity(tile_entity)
            .insert((Visibility::Inherited, Name::new(col.to_string())));
        commands.entity(lane_entity).add_child(tile_entity);
    }

    lane_entity
}

fn insert_car_lane(
    commands: &mut Commands,
    tileset: &Res<Tileset>,
    idx: usize,
    direction: &Direction,
    spawn_delays: &Vec<f32>,
    speed: f32,
) -> Entity {
    let y = -(idx as f32) * SCALED_TILE_SIZE;

    let position = match direction {
        Direction::Left => vec3(SCALED_TILE_SIZE * LEVEL_COLS as f32, 0., 10.),
        Direction::Right => vec3(-(SCALED_TILE_SIZE), 0., 10.),
    };

    let direction_vec = match direction {
        Direction::Left => crate::movement::Direction::left(),
        Direction::Right => crate::movement::Direction::right(),
    };

    let lane_entity = commands
        .spawn((
            CarLane {
                direction: *direction,
            },
            Name::new(format!("CarLane{}", idx.to_string())),
            Transform::from_translation(Vec3::new(0., y, 0.0)),
            Visibility::Inherited,
        ))
        .id();

    for col in 0..LEVEL_COLS {
        let x = col as f32 * SCALED_TILE_SIZE;
        let tile_entity = spawn_tile(
            commands,
            tileset,
            &TextureName::Road,
            &Vec3::new(x, 0., 0.),
            false,
        );

        commands
            .entity(tile_entity)
            .insert((Visibility::Inherited, Name::new(col.to_string())));
        commands.entity(lane_entity).add_child(tile_entity);
    }

    let car_spawner = spawn_car_spawner(commands, spawn_delays, position, direction_vec, speed);

    commands.entity(lane_entity).add_child(car_spawner);

    lane_entity
}

fn insert_end_lane(commands: &mut Commands, tileset: &Res<Tileset>, idx: usize) -> Entity {
    let y = -(idx as f32) * SCALED_TILE_SIZE;
    let lane_entity = commands
        .spawn((
            EndLane,
            Transform::from_translation(Vec3::new(0., y, 0.0)),
            Visibility::Inherited,
            Name::new(format!("EndLane{}", idx.to_string())),
        ))
        .id();

    for col in 0..LEVEL_COLS {
        let x = col as f32 * SCALED_TILE_SIZE;
        let tile_entity = spawn_tile(
            commands,
            tileset,
            &TextureName::Grass,
            &Vec3::new(x, 0., 100.),
            false,
        );

        commands
            .entity(tile_entity)
            .insert((Visibility::Inherited, Name::new(col.to_string())));
        commands.entity(lane_entity).add_child(tile_entity);
    }

    lane_entity
}
