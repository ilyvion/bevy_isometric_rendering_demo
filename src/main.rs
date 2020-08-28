use crate::camera_movement::CameraMouseMovementPlugin;
use crate::map_render::MapRenderPlugin;
use crate::map_sprites::MapSpritesPlugin;
use bevy::prelude::*;

mod camera_movement;
mod map_render;
mod map_sprites;

fn main() {
    App::build()
        .add_default_plugins()
        .init_resource::<GameState>()
        .add_asset::<Map>()
        .add_plugin(MapSpritesPlugin::default())
        .add_plugin(CameraMouseMovementPlugin::default())
        .add_plugin(MapRenderPlugin::default())
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut maps: ResMut<Assets<Map>>, mut game_state: ResMut<GameState>) {
    game_state.current_map = maps.add(Map {
        width: 16,
        height: 16,
        #[rustfmt::skip]
        tiles: vec![
            75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
            75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
            75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
            75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
            75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
            75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
            75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
            75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
            75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
            75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
            75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
            75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
            75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
            75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
            75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
            75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
        ],
    });

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
