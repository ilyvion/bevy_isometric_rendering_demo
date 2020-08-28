use bevy::asset::{HandleId, LoadState};
use bevy::prelude::*;
use bevy::sprite::TextureAtlasBuilder;

const MAP_TEXTURES_PATH: &str = "assets/textures/map";
const MAP_TEXTURE_FILE_NAME_PREFIX: &str = "landscapeTiles_";
const MAP_TEXTURE_FILE_NAME_LEN: usize = "landscapeTiles_000.png".len();

#[derive(Default)]
pub struct MapSpritesPlugin;

impl Plugin for MapSpritesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MapSprites>()
            .add_startup_system(setup.system())
            .add_system(load_map_sprites.system());
    }
}

#[derive(Default)]
pub struct MapSprites {
    handles: Option<Vec<HandleId>>,
    sprite_lookup_table: Vec<(u32, Vec2)>,
    pub texture_atlas: Handle<TextureAtlas>,
}

impl MapSprites {
    #[inline]
    pub fn is_ready(&self) -> bool {
        self.handles.is_none()
    }

    #[inline]
    pub fn tile_sprite_index(&self, index: usize) -> u32 {
        self.sprite_lookup_table[index].0
    }

    #[inline]
    pub fn tile_sprite_height(&self, index: usize) -> Vec2 {
        self.sprite_lookup_table[index].1
    }
}

fn setup(mut map_sprite_handles: ResMut<MapSprites>, asset_server: Res<AssetServer>) {
    map_sprite_handles.handles = Some(asset_server.load_asset_folder(MAP_TEXTURES_PATH).unwrap());
}

fn load_map_sprites(
    mut map_sprites: ResMut<MapSprites>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    match map_sprites.handles.as_ref() {
        None => {}
        Some(handles) => {
            if let Some(LoadState::Loaded(_)) = asset_server.get_group_load_state(handles) {
                // Create a texture atlas from the map textures
                let mut texture_atlas_builder = TextureAtlasBuilder::default();
                for texture_id in handles.iter() {
                    let handle = Handle::from_id(*texture_id);
                    let texture = textures.get(&handle).unwrap();
                    texture_atlas_builder.add_texture(handle, &texture);
                }
                let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();

                // Associate the texture atlas handles with the corresponding map tile id
                let mut map_texture_path = MapTexturePath::new();
                for _ in 0..handles.len() {
                    let texture_handle = asset_server
                        .get_handle(map_texture_path.next())
                        .unwrap_or_else(|| panic!("Could not load {}", &map_texture_path.0));
                    let texture_index = texture_atlas.get_texture_index(texture_handle).unwrap();
                    let texture = textures.get(&texture_handle).unwrap();
                    map_sprites
                        .sprite_lookup_table
                        .push((texture_index as u32, texture.size));
                    textures.remove(&texture_handle);
                }
                map_sprites.texture_atlas = texture_atlases.add(texture_atlas);

                // With the texture atlas created, we no longer need the individual texture handles
                map_sprites.handles.take();
            }
        }
    }
}

struct MapTexturePath(String, u8);

impl MapTexturePath {
    #[inline]
    fn new() -> Self {
        let mut map_texture_path =
            String::with_capacity(MAP_TEXTURES_PATH.len() + 1 + MAP_TEXTURE_FILE_NAME_LEN);
        map_texture_path.push_str(MAP_TEXTURES_PATH);
        map_texture_path.push('/');
        map_texture_path.push_str(MAP_TEXTURE_FILE_NAME_PREFIX);

        Self(map_texture_path, 0)
    }

    #[inline]
    fn next(&mut self) -> &str {
        // Ah, the silly lengths we go to to avoid (re)allocation
        self.0
            .truncate(MAP_TEXTURES_PATH.len() + 1 + MAP_TEXTURE_FILE_NAME_PREFIX.len());
        self.0.push((b'0' + (self.1 / 100)) as char);
        self.0.push((b'0' + ((self.1 % 100) / 10)) as char);
        self.0.push((b'0' + (self.1 % 10)) as char);
        self.0.push_str(".png");

        self.1 = self
            .1
            .checked_add(1)
            .unwrap_or_else(|| panic!("MapTexturePath overflow"));

        &self.0
    }
}
