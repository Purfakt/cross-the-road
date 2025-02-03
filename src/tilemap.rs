use bevy::{math::vec3, prelude::*};

pub const TILE_SIZE: usize = 32;
pub const TILE_ROWS: u32 = 10;
pub const TILE_COLUMNS: u32 = 10;

pub const SCALE: f32 = 2.;

pub struct TilesetPlugin;

#[derive(Resource)]
pub struct Tileset {
    pub layout: Handle<TextureAtlasLayout>,
    pub image: Handle<Image>,
}

pub enum TextureName {
    Grass,
    Road,
    Car1,
    Car2,
    Car3,
    Duck,
}

impl TextureName {
    pub fn index(&self) -> usize {
        match *self {
            TextureName::Grass => 0,
            TextureName::Road => 10,
            TextureName::Car1 => 1,
            TextureName::Car2 => 2,
            TextureName::Car3 => 3,
            TextureName::Duck => 13,
        }
    }
}

impl Plugin for TilesetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_texture);
    }
}

pub fn spawn_tile(
    commands: &mut Commands,
    tileset: &Tileset,
    texture_name: &TextureName,
    position: &Vec2,
) -> Entity {
    commands
        .spawn((
            Sprite::from_atlas_image(
                tileset.image.clone(),
                TextureAtlas {
                    layout: tileset.layout.clone(),
                    index: texture_name.index(),
                },
            ),
            Transform {
                translation: vec3(position.x, position.y, 0.),
                scale: vec3(SCALE, SCALE, 1.),
                ..Default::default()
            },
        ))
        .id()
}

fn load_texture(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load::<Image>("tiles.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE as u32),
        TILE_COLUMNS,
        TILE_ROWS,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlas_layouts.add(layout);

    commands.insert_resource(Tileset {
        image: texture,
        layout: texture_atlas_handle,
    });
}
