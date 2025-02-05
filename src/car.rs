use bevy::{math::vec3, prelude::*};

use crate::{
    movement::{Direction, Movable, Speed},
    tilesheet::{spawn_tile, TextureName, Tileset},
};

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnCarEvent>()
            .add_systems(Update, (timer_tick, car_spawn));
    }
}

pub struct CarSettings {
    direction: Direction,
    speed: f32,
    texture: TextureName,
}

impl Default for CarSettings {
    fn default() -> Self {
        Self {
            direction: Direction::left(),
            speed: 100.,
            texture: TextureName::Car1,
        }
    }
}

#[derive(Component)]
pub struct Car;

#[derive(Component, Debug, Clone)]
pub struct CarSpawner {
    pub current_timer_index: usize,
    pub timings: Vec<f32>,
    pub position: Vec3,
    pub direction: Direction,
    pub speed: f32,
}

#[derive(Event, Debug)]
pub struct SpawnCarEvent {
    pub car_spawner: CarSpawner,
    pub entity: Entity,
}

#[derive(Component, Deref, DerefMut)]
pub struct SpawnDelay(pub Timer);

pub fn spawn_car_spawner(
    commands: &mut Commands,
    timings: &Vec<f32>,
    position: Vec3,
    direction: Direction,
    speed: f32,
) -> Entity {
    commands
        .spawn((
            CarSpawner {
                current_timer_index: 0,
                timings: timings.clone(),
                position,
                direction,
                speed,
            },
            Transform::from_translation(vec3(0., 0., 0.)),
            Visibility::Inherited,
            SpawnDelay(Timer::from_seconds(2., TimerMode::Once)),
        ))
        .id()
}

fn timer_tick(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpawnDelay, &mut CarSpawner)>,
    mut spawn_car_event: EventWriter<SpawnCarEvent>,
) {
    for (entity, mut delay, mut car_spawner) in &mut query {
        if delay.tick(time.delta()).just_finished() {
            let car = &car_spawner;

            spawn_car_event.send(SpawnCarEvent {
                car_spawner: car_spawner.clone(),
                entity,
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
    mut spawn_car_events: EventReader<SpawnCarEvent>,
) {
    for event in spawn_car_events.read() {
        let texture = match rand::random_range(0..=2) {
            0 => TextureName::Car1,
            1 => TextureName::Car2,
            _ => TextureName::Car3,
        };

        let car = spawn_car(
            &mut commands,
            &tileset,
            CarSettings {
                texture,
                direction: event.car_spawner.direction,
                speed: event.car_spawner.speed,
            },
            &event.car_spawner.position,
        );

        commands.entity(event.entity).add_child(car);
    }
}

fn spawn_car(
    commands: &mut Commands,
    tileset: &Tileset,
    settings: CarSettings,
    position: &Vec3,
) -> Entity {
    let flip_x = settings.direction.x > 0.;
    let car_entity = spawn_tile(commands, tileset, &settings.texture, position, flip_x);

    commands.entity(car_entity).insert((
        Car,
        Movable,
        settings.direction,
        Speed(settings.speed),
        Name::new("Car"),
    ));

    car_entity
}
