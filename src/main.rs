use crate::camera_movement::CameraMovementPlugin;
use crate::map_loader::MapLoaderPlugin;
use crate::map_render::MapRenderPlugin;
use crate::map_sprites::MapTextureLoadingPlugin;
use bevy::prelude::*;

mod camera_movement;
mod map_loader;
mod map_render;
mod map_sprites;
mod util;

fn main() {
    App::build()
        .add_default_plugins()
        .init_resource::<GameState>()
        .add_asset::<Map>()
        .add_plugin(MapLoaderPlugin::default())
        .add_plugin(MapTextureLoadingPlugin::default())
        .add_plugin(MapRenderPlugin::default())
        .add_plugin(CameraMovementPlugin::default())
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut maps: ResMut<Assets<Map>>,
    mut game_state: ResMut<GameState>,
    asset_server: Res<AssetServer>,
) {
    game_state.current_map = asset_server
        .load_sync(&mut maps, "assets/maps/every.map")
        .unwrap();

    commands.spawn(Camera2dComponents::default());
}

#[derive(Default)]
pub struct GameState {
    current_map: Handle<Map>,
}

pub struct Map {
    width: usize,
    height: usize,
    tiles: Vec<usize>,
}
