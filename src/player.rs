use bevy::prelude::*;

use crate::{
    tilesheet::{spawn_tile, TextureName, Tileset},
    world::CELL_0_X,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, tileset: Res<Tileset>) {
    let entity = spawn_tile(
        &mut commands,
        &tileset,
        &TextureName::Duck,
        &Vec3::new(CELL_0_X, 0., 100.),
    );

    commands.entity(entity).insert(Player);
}
