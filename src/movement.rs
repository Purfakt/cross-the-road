use bevy::{math::vec2, prelude::*};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, apply_velocity);
    }
}

#[derive(Component, Deref)]
pub struct Direction(Vec2);

impl From<Vec2> for Direction {
    fn from(value: Vec2) -> Self {
        Direction(value.normalize_or_zero())
    }
}

impl Direction {
    pub fn new() -> Self {
        Vec2::ZERO.into()
    }

    pub fn set(&mut self, vec: Vec2) {
        self.0 = vec.normalize_or_zero().into();
    }

    pub fn set_xy(&mut self, x: f32, y: f32) {
        self.set(vec2(x, y));
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
