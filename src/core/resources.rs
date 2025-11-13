use bevy::prelude::*;
use rand::{SeedableRng, rngs::StdRng};

use super::*;

#[derive(Resource)]
pub struct GridSize {
    pub size: UVec2,
    pub pixels: u32,
}

impl GridSize {
    pub fn default() -> Self {
        return GridSize {
            size: UVec2::splat(GRID_SIZE),
            pixels: GRID_PIXELS,
        };
    }

    pub fn to_pixels(&self, position: GridPosition, z:f32) -> Vec3 {
        let half_width = (self.size.x as f32 * self.pixels as f32) / 2.0;
        let half_height = (self.size.y as f32 * self.pixels as f32) / 2.0;

        Vec3::new(
            (position.x as f32 + 0.5) * self.pixels as f32 - half_width,
            (position.y as f32 + 0.5) * self.pixels as f32 - half_height,
            z,
        )
    }
}

#[derive(Resource)]
pub struct RandomSource {
    pub rng: StdRng,
}

impl Default for RandomSource {
    fn default() -> Self {
        // rand 0.9: use from_os_rng() instead of deprecated/removed from_entropy()
        Self { rng: StdRng::from_os_rng() }
    }
}

