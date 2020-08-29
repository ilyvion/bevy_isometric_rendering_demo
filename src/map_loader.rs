use crate::Map;
use bevy::asset::{AssetLoadError, AssetLoader};
use bevy::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};
use std::path::Path;

#[derive(Default)]
pub struct MapLoaderPlugin;

impl Plugin for MapLoaderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset_loader::<Map, MapLoader>();
    }
}

#[derive(Default)]
pub struct MapLoader;

impl AssetLoader<Map> for MapLoader {
    fn from_bytes(&self, _: &Path, bytes: Vec<u8>) -> Result<Map, anyhow::Error> {
        let cursor = Cursor::new(bytes);
        load_map(cursor)
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["map"];
        EXTENSIONS
    }

    fn load_from_file(&self, asset_path: &Path) -> Result<Map, AssetLoadError> {
        let file = BufReader::new(File::open(asset_path)?);
        Ok(load_map(file)?)
    }
}

fn load_map(mut buf_read: impl BufRead) -> Result<Map, anyhow::Error> {
    let mut line = String::new();

    buf_read.read_line(&mut line)?;
    let mut line_values = split_line(&line).map(|v| v.parse::<usize>());
    let width = line_values
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing width"))??;
    let height = line_values
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing height"))??;
    if line_values.next().is_some() {
        return Err(anyhow::anyhow!(
            "Unexpected extraneous value on line 1 of map file"
        ));
    }
    drop(line_values);
    line.clear();

    let mut tiles = Vec::with_capacity(width * height);
    loop {
        let read = buf_read.read_line(&mut line)?;
        if read == 0 {
            break;
        }

        for value in split_line(&line).map(|v| v.parse::<usize>()) {
            tiles.push(value?);
        }
        line.clear();
    }

    if tiles.len() != width * height {
        return Err(anyhow::anyhow!(
            "Incorrect number of tiles in map; expected {}, got {}",
            width * height,
            tiles.len()
        ));
    }

    Ok(Map {
        width,
        height,
        tiles,
    })
}

fn split_line(line: &str) -> impl Iterator<Item = &str> {
    line.split(',').map(|i| i.trim())
}
