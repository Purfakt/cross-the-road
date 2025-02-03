use bevy::{math::vec3, prelude::*};

use crate::{
    movement::{Direction, Movable, Speed},
    tilesheet::{spawn_tile, TextureName, Tileset},
};

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_event::<SpawnCar>()
            .add_systems(Update, (timer_tick, car_spawn));
    }
}

#[derive(Component)]
pub struct Car;

#[derive(Component)]
pub struct CarSpawner {
    current_timer_index: usize,
    timings: Vec<f32>,
    position: Vec3,
    direction: Direction,
}

#[derive(Event, Debug)]
pub struct SpawnCar {
    position: Vec3,
    direction: Direction,
}

#[derive(Component, Deref, DerefMut)]
pub struct SpawnDelay(pub Timer);

fn setup(mut commands: Commands) {
    commands.spawn((
        CarSpawner {
            current_timer_index: 1,
            timings: vec![2., 1., 3.],
            position: vec3(0., 0., 0.),
            direction: Direction::left(),
        },
        SpawnDelay(Timer::from_seconds(2., TimerMode::Once)),
    ));
}

fn timer_tick(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpawnDelay, &mut CarSpawner)>,
    mut spawn_car_event: EventWriter<SpawnCar>,
) {
    for (entity, mut delay, mut car_spawner) in &mut query {
        if delay.tick(time.delta()).just_finished() {
            let car = &car_spawner;

            spawn_car_event.send(SpawnCar {
                position: car.position,
                direction: car.direction,
            });

            let timings = &car.timings;
            let current_timer_index = &car.current_timer_index;

            commands
                .entity(entity)
                .remove::<SpawnDelay>()
                .insert(SpawnDelay(Timer::from_seconds(
                    timings[*current_timer_index],
                    TimerMode::Once,
                )));

            let new_index = if *current_timer_index < timings.len() - 1 {
                *current_timer_index + 1
            } else {
                0
            };

            car_spawner.current_timer_index = new_index;
        }
    }
}

fn car_spawn(
    mut commands: Commands,
    tileset: Res<Tileset>,
    mut spawn_car_events: EventReader<SpawnCar>,
) {
    for event in spawn_car_events.read() {
        spawn_car(
            &mut commands,
            &tileset,
            &TextureName::Car1,
            &event.position,
            event.direction,
        );
    }
}

fn spawn_car(
    commands: &mut Commands,
    tileset: &Tileset,
    texture_name: &TextureName,
    position: &Vec3,
    direction: Direction,
) -> Entity {
    let car_entity = spawn_tile(commands, tileset, texture_name, position);

    commands
        .entity(car_entity)
        .insert((Car, Movable, direction, Speed(100.), Name::new("Car")));

    car_entity
}
