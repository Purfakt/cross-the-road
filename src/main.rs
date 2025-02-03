use bevy::{math::vec2, prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use tilemap::{spawn_tile, TextureName, Tileset, TilesetPlugin};

pub mod tilemap;

const TILE_SIZE: usize = 32;
const SCALE: f32 = 2.;

const ROWS: usize = 9;
const COLS: usize = 16;

const WINDOW_WIDTH: f32 = (TILE_SIZE * COLS) as f32 * SCALE;
const WINDOW_HEIGHT: f32 = (TILE_SIZE * ROWS) as f32 * SCALE;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum AppState {
    #[default]
    Loading,
    _Game,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            WINDOW_WIDTH as f32,
                            WINDOW_HEIGHT as f32,
                        ),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::new(),
            TilesetPlugin,
        ))
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, tileset: Res<Tileset>) {
    commands.spawn(Camera2d);
    let mut _entity = spawn_tile(&mut commands, &tileset, &TextureName::Grass, &vec2(0., 0.));
}
