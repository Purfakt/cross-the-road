use bevy::{math::vec3, prelude::*};
use std::fmt::Display;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, apply_velocity);
    }
}

#[derive(Component, Deref, Debug, Clone, Copy)]
pub struct Direction(Vec3);

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Vec3> for Direction {
    fn from(value: Vec3) -> Self {
        Direction(value.normalize_or_zero())
    }
}

impl Direction {
    pub fn new() -> Self {
        Vec3::ZERO.into()
    }

    pub fn left() -> Self {
        Vec3::NEG_X.into()
    }

    pub fn right() -> Self {
        Vec3::X.into()
    }

    pub fn set(&mut self, vec: Vec3) {
        self.0 = vec.normalize_or_zero().into();
    }

    pub fn set_xy(&mut self, x: f32, y: f32, z: f32) {
        self.set(vec3(x, y, z));
    }
}

#[derive(Component, Deref)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Movable;

fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&Direction, &Speed, &mut Transform), With<Movable>>,
) {
    let delta = time.delta_secs();

    for (direction, speed, mut transform) in &mut query {
        let translation = &mut transform.translation;
        translation.x += direction.x * delta * **speed;
        translation.y += direction.y * delta * **speed;
    }
}
