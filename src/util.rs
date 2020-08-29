use bevy::math::Vec2;

pub trait IsometricOperations {
    const MAP_TILE_WIDTH: f32;
    const MAP_TILE_HEIGHT: f32;

    fn to_isometric(&self) -> Self;
    fn from_isometric(&self) -> Self;
}

impl IsometricOperations for Vec2 {
    const MAP_TILE_WIDTH: f32 = 128.0;
    const MAP_TILE_HEIGHT: f32 = 64.0;

    fn to_isometric(&self) -> Self {
        let isometric_x = (self.y() * Self::MAP_TILE_WIDTH / 2.0)
            + (self.x() as f32 * Self::MAP_TILE_WIDTH / 2.0);
        let isometric_y = (self.x() as f32 * Self::MAP_TILE_HEIGHT / 2.0)
            - (self.y() as f32 * Self::MAP_TILE_HEIGHT / 2.0);

        Self::new(isometric_x, isometric_y)
    }

    fn from_isometric(&self) -> Self {
        let flat_x = (0.5
            * (self.x() / (Self::MAP_TILE_WIDTH / 2.0) + self.y() / (Self::MAP_TILE_HEIGHT / 2.0)))
            .floor();
        let flat_y = (0.5
            * (-self.x() / (Self::MAP_TILE_WIDTH / 2.0)
                + self.y() / (Self::MAP_TILE_HEIGHT / 2.0)))
            .floor();

        Self::new(flat_x, flat_y)
    }
}
