use bevy::{math::vec2, prelude::*};

use crate::tilemap::{spawn_tile, TextureName, Tileset, SCALE, TILE_SIZE};

pub const LEVEL_ROWS: usize = 9;
pub const LEVEL_COLS: usize = 16;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup_level);
    }
}

#[derive(Component)]
pub struct Lane {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Level {
    pub lanes_amount: u8,
}

fn setup_level(mut commands: Commands, tileset: Res<Tileset>) {
    let lanes = vec![
        Vec2::X,
        Vec2::NEG_X,
        Vec2::X,
        Vec2::NEG_X,
        Vec2::X,
        Vec2::NEG_X,
    ];

    let scaled_tile_size = TILE_SIZE as f32 * SCALE;
    let first_row_y = ((LEVEL_ROWS as f32 * scaled_tile_size) / 2.0) - scaled_tile_size / 2.;
    let first_col_x = -((LEVEL_COLS as f32 * scaled_tile_size) / 2.0) + scaled_tile_size / 2.;
    let remaining_lanes = LEVEL_ROWS - 1 - lanes.len();

    for col in 0..LEVEL_COLS {
        let y = first_row_y;
        let x = first_col_x + (col as f32 * scaled_tile_size);
        spawn_tile(&mut commands, &tileset, &TextureName::Grass, &vec2(x, y));
    }

    for (idx, _lane) in lanes.iter().enumerate() {
        for col in 0..LEVEL_COLS {
            let y = first_row_y - ((idx as f32 + 1.) * scaled_tile_size);
            let x = first_col_x + (col as f32 * scaled_tile_size);
            spawn_tile(&mut commands, &tileset, &TextureName::Road, &vec2(x, y));
        }
    }

    for lane in 0..remaining_lanes {
        for col in 0..LEVEL_COLS {
            let y = first_row_y - ((lane as f32 + lanes.len() as f32 + 1.) * scaled_tile_size);
            let x = first_col_x + (col as f32 * scaled_tile_size);
            spawn_tile(&mut commands, &tileset, &TextureName::Grass, &vec2(x, y));
        }
    }
}
