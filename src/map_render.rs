use crate::map_sprites::MapSprites;
use crate::{GameState, Map};
use bevy::prelude::*;

const TILE_WIDTH: isize = 128;
const TILE_HEIGHT: isize = 64;

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
    mut query: Query<(&RenderedMap, Entity)>,
) {
    if !map_sprites.is_ready() {
        return;
    }

    // We only need to generate a new map render when the map changes
    if map_render_data.0 == game_state.current_map {
        return;
    }

    // Despawn the old map render entity
    for (_, entity) in &mut query.iter() {
        commands.despawn(entity);
    }

    // Create a new map render entity and generate all the associated map sprites
    // as children
    let map = maps.get(&game_state.current_map).unwrap();
    commands
        .spawn((RenderedMap, Transform::default()))
        .with_children(|parent| {
            let screen_y_min = -(map.height as isize * TILE_HEIGHT / 2) as f32;
            let screen_y_max = (map.width as isize * TILE_HEIGHT / 2) as f32;
            for map_x in 0..map.width as isize {
                for map_y in (0..map.height as isize).rev() {
                    // Translate map coordinates to screen coordinates
                    let screen_x = ((map_y * TILE_WIDTH / 2) + (map_x * TILE_WIDTH / 2)) as f32;
                    let screen_y = ((map_x * TILE_HEIGHT / 2) - (map_y * TILE_HEIGHT / 2)) as f32;

                    let tile = map.tiles[(map_y * map.width as isize + map_x) as usize];

                    // If a tile is taller than TILE_HEIGHT; it needs to be shifted up accordingly to
                    // be at the same baseline as a regular height tile.
                    let excess_height =
                        (map_sprites.tile_sprite_height(tile).y() - TILE_HEIGHT as f32).max(0.0);

                    parent.spawn(SpriteSheetComponents {
                        draw: Draw {
                            is_transparent: true,
                            ..Default::default()
                        },
                        translation: Translation(Vec3::new(
                            screen_x,
                            screen_y + excess_height / 2.,
                            (screen_y_max - screen_y - screen_y_min) / screen_y_max,
                        )),
                        sprite: TextureAtlasSprite::new(map_sprites.tile_sprite_index(tile)),
                        texture_atlas: map_sprites.texture_atlas,
                        ..Default::default()
                    });
                }
            }
        });

    // Update the render map handle so we don't re-render it until it next
    // changes
    map_render_data.0 = game_state.current_map;
}