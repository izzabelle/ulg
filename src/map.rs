use crate::Assets;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::Context;
use ggez::GameResult as Result;
use rand::Rng;
use std::collections::HashMap;

const TILE_DIMENSIONS: (f32, f32) = (120.0, 140.0);

pub struct Map {
    data: HashMap<CubeIndex, Tile>,
}

impl Map {
    pub fn new(radius: isize) -> Self {
        let mut data: HashMap<CubeIndex, Tile> = HashMap::new();
        let mut rng = rand::thread_rng();
        for x in -radius..radius {
            for y in -radius..radius {
                for z in -radius..radius {
                    data.insert((x, y, z).into(), Tile::new(rng.gen_range(0, 2)));
                }
            }
        }
        Self { data }
    }

    pub fn render(&self, ctx: &mut Context, assets: &Assets, offset: (isize, isize)) -> Result<()> {
        for (location, tile) in self.data.iter() {
            let img = &assets.tiles[tile.terrain_texture_idx];
            let location = AxialIndex::from(*location);
            let x: f32 = offset.0 as f32
                + location.q as f32 * TILE_DIMENSIONS.0
                + if location.r % 2 != 0 { TILE_DIMENSIONS.0 / 2.0 } else { 0.0 };
            let y: f32 = offset.1 as f32
                + location.r as f32 * TILE_DIMENSIONS.1 as f32
                + location.r as f32 * -35.0;
            let param = DrawParam::new().dest(mint::Point2 { x, y });
            graphics::draw(ctx, img, param)?;
        }
        Ok(())
    }
}

pub struct Tile {
    terrain_texture_idx: usize,
}

impl Tile {
    pub fn new(terrain_texture_idx: usize) -> Self {
        Self { terrain_texture_idx }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct CubeIndex {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct AxialIndex {
    pub q: isize,
    pub r: isize,
}

impl std::convert::From<(isize, isize, isize)> for CubeIndex {
    fn from((x, y, z): (isize, isize, isize)) -> Self {
        Self { x, y, z }
    }
}

impl std::convert::From<(isize, isize)> for AxialIndex {
    fn from((q, r): (isize, isize)) -> Self {
        Self { q, r }
    }
}

impl std::convert::From<AxialIndex> for CubeIndex {
    fn from(axial_index: AxialIndex) -> Self {
        CubeIndex { x: axial_index.q, y: -axial_index.q - axial_index.r, z: axial_index.r }
    }
}

impl std::convert::From<CubeIndex> for AxialIndex {
    fn from(cube_index: CubeIndex) -> Self {
        AxialIndex { q: cube_index.x, r: cube_index.z }
    }
}
