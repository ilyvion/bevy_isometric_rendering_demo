use crate::map_sprites::MapSprites;
use crate::util::IsometricOperations;
use crate::{GameState, Map};
use bevy::prelude::*;
use bevy::render::camera::Camera;

#[derive(Default)]
pub struct MapRenderPlugin;

impl Plugin for MapRenderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MapRenderData>()
            .add_system(render_map.system());
    }
}

#[derive(Default)]
struct MapRenderData(Handle<Map>);

struct RenderedMap;

fn render_map(
    mut commands: Commands,
    maps: Res<Assets<Map>>,
    map_sprites: Res<MapSprites>,
    game_state: Res<GameState>,
    mut map_render_data: ResMut<MapRenderData>,
    mut map_query: Query<(&RenderedMap, Entity)>,
    mut camera_query: Query<(&Camera, &mut Translation)>,
) {
    if !map_sprites.is_ready() {
        return;
    }

    // We only need to generate a new map render when the map changes
    if map_render_data.0 == game_state.current_map {
        return;
    }

    // Despawn the old map render entity
    for (_, entity) in &mut map_query.iter() {
        commands.despawn(entity);
    }

    // Create a new map render entity and generate all the associated map sprites
    // as children
    let map = maps.get(&game_state.current_map).unwrap();
    let screen_x_max = (map.height as f32 * Vec2::MAP_TILE_WIDTH / 2.0)
        + (map.width as f32 * Vec2::MAP_TILE_WIDTH / 2.0);
    let screen_y_min = -(map.height as f32 * Vec2::MAP_TILE_HEIGHT / 2.0);
    let screen_y_max = map.width as f32 * Vec2::MAP_TILE_HEIGHT / 2.0;
    commands
        .spawn((RenderedMap, Transform::default()))
        .with_children(|parent| {
            for map_x in 0..map.width as isize {
                for map_y in (0..map.height as isize).rev() {
                    let map_position = Vec2::new(map_x as f32, map_y as f32);
                    let screen_position = map_position.to_isometric();

                    let tile = map.tiles[(map_y * map.width as isize + map_x) as usize];

                    // If a tile is taller than TILE_HEIGHT; it needs to be shifted up accordingly to
                    // be at the same baseline as a regular height tile.
                    let excess_height = (map_sprites.tile_sprite_height(tile).y()
                        - Vec2::MAP_TILE_HEIGHT as f32)
                        .max(0.0);

                    parent.spawn(SpriteSheetComponents {
                        draw: Draw {
                            is_transparent: true,
                            ..Default::default()
                        },
                        translation: Translation(Vec3::new(
                            screen_position.x(),
                            screen_position.y() + excess_height / 2.,
                            (screen_y_max - screen_position.y() - screen_y_min) / screen_y_max,
                        )),
                        sprite: TextureAtlasSprite {
                            index: map_sprites.tile_sprite_index(tile),
                            ..Default::default()
                        },
                        texture_atlas: map_sprites.texture_atlas,
                        ..Default::default()
                    });
                }
            }
        });

    // Update the render map handle so we don't re-render it until it next
    // changes
    map_render_data.0 = game_state.current_map;

    // Center the camera on the loaded map
    for (_, mut translation) in &mut camera_query.iter() {
        *translation.x_mut() = screen_x_max / 2.0 - Vec2::MAP_TILE_WIDTH / 2.0;
        *translation.y_mut() = (screen_y_max + screen_y_min) / 2.0 - Vec2::MAP_TILE_HEIGHT / 2.0;
    }
}
