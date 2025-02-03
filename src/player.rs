use bevy::{math::vec2, prelude::*};

use crate::{
    movement::{Direction, Movable, Speed},
    tilesheet::{spawn_tile, TextureName, Tileset},
    world::CELL_0_Y,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, handle_input);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, tileset: Res<Tileset>) {
    let entity = spawn_tile(
        &mut commands,
        &tileset,
        &TextureName::Duck,
        &Vec3::new(0., CELL_0_Y, 100.),
    );

    commands.entity(entity).insert((
        Player,
        Direction::new(),
        Speed(100.),
        Movable,
        Name::new("Player"),
    ));
}

fn handle_input(mut query: Query<&mut Direction, With<Player>>, input: Res<ButtonInput<KeyCode>>) {
    if let Ok(mut direction) = query.get_single_mut() {
        let mut y = 0.;
        if input.pressed(KeyCode::KeyW) {
            y += 1.;
        }
        if input.pressed(KeyCode::KeyS) {
            y -= 1.;
        };

        let mut x = 0.;
        if input.pressed(KeyCode::KeyD) {
            x += 1.;
        }
        if input.pressed(KeyCode::KeyA) {
            x -= 1.;
        };

        direction.set(vec2(x, y));
    }
}
