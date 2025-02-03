use std::fmt;

use bevy::prelude::*;

use crate::tilesheet::{spawn_tile, TextureName, Tileset, SCALED_TILE_SIZE};
use crate::world::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup_level);
    }
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
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

fn setup_level(mut commands: Commands, tileset: Res<Tileset>) {
    let lanes = commands
        .spawn((
            Transform::from_xyz(CELL_0_X, CELL_0_Y, 0.),
            Name::new("Lanes"),
        ))
        .id();

    let spawn_lanes = insert_spawn_lane(&mut commands, &tileset);

    commands.entity(lanes).add_child(spawn_lanes);

    let car_lanes = commands
        .spawn((Name::new("CarLanes"), Transform::from_xyz(0., 0., 0.)))
        .id();

    let amount_car_lanes = 6;

    let remaining_lanes = LEVEL_ROWS - 1 - amount_car_lanes;

    for idx in 1..amount_car_lanes + 1 {
        let lane_entity = insert_car_lane(&mut commands, &tileset, idx, Direction::Left);
        commands.entity(car_lanes).add_child(lane_entity);
    }

    commands.entity(lanes).add_child(car_lanes);

    let end_lanes = commands
        .spawn((Name::new("EndLanes"), Transform::from_xyz(0., 0., 0.)))
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
            Name::new("SpawnLane"),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            Visibility::Inherited,
        ))
        .id();

    for col in 0..LEVEL_COLS {
        let x = col as f32 * SCALED_TILE_SIZE;
        let tile_entity = spawn_tile(
            commands,
            tileset,
            &TextureName::Grass,
            &Vec3::new(x, 0., 100.),
        );

        commands
            .entity(tile_entity)
            .insert(Name::new(col.to_string()));
        commands.entity(lane_entity).add_child(tile_entity);
    }

    lane_entity
}

fn insert_car_lane(
    commands: &mut Commands,
    tileset: &Res<Tileset>,
    idx: usize,
    direction: Direction,
) -> Entity {
    let y = -(idx as f32) * SCALED_TILE_SIZE;
    let lane_entity = commands
        .spawn((
            CarLane { direction },
            Name::new(format!("CarLane{}", idx.to_string())),
            Transform::from_translation(Vec3::new(0.0, y, 0.0)),
            Visibility::Inherited,
        ))
        .id();

    for col in 0..LEVEL_COLS {
        let x = col as f32 * SCALED_TILE_SIZE;
        let tile_entity = spawn_tile(
            commands,
            tileset,
            &TextureName::Road,
            &Vec3::new(x, 0., 100.),
        );

        commands
            .entity(tile_entity)
            .insert(Name::new(col.to_string()));
        commands.entity(lane_entity).add_child(tile_entity);
    }

    lane_entity
}

fn insert_end_lane(commands: &mut Commands, tileset: &Res<Tileset>, idx: usize) -> Entity {
    let y = -(idx as f32) * SCALED_TILE_SIZE;
    let lane_entity = commands
        .spawn((
            EndLane,
            Name::new(format!("EndLane{}", idx.to_string())),
            Transform::from_translation(Vec3::new(0.0, y, 0.0)),
            Visibility::Inherited,
        ))
        .id();

    for col in 0..LEVEL_COLS {
        let x = col as f32 * SCALED_TILE_SIZE;
        let tile_entity = spawn_tile(
            commands,
            tileset,
            &TextureName::Grass,
            &Vec3::new(x, 0., 100.),
        );

        commands
            .entity(tile_entity)
            .insert(Name::new(col.to_string()));
        commands.entity(lane_entity).add_child(tile_entity);
    }

    lane_entity
}
