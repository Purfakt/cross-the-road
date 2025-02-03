use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use level::{LevelPlugin, LEVEL_COLS, LEVEL_ROWS};
use tilemap::{TilesetPlugin, SCALE, TILE_SIZE};

pub mod level;
pub mod tilemap;

const WINDOW_WIDTH: f32 = (TILE_SIZE * LEVEL_COLS) as f32 * SCALE;
const WINDOW_HEIGHT: f32 = (TILE_SIZE * LEVEL_ROWS) as f32 * SCALE;

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
            LevelPlugin,
        ))
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
