use crate::Assets;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::Context;
use ggez::GameResult as Result;
use mint::Point2;
use rand::Rng;
use std::collections::HashMap;

const TILE_DIMENSIONS: (f32, f32) = (120.0, 140.0);

#[derive(Debug)]
pub struct Map {
    data: HashMap<CubeIndex, Tile>,
    mouse_over: Option<CubeIndex>,
}

impl Map {
    pub fn new(radius: isize) -> Self {
        let mut data: HashMap<CubeIndex, Tile> = HashMap::new();
        let mut rng = rand::thread_rng();
        for x in -radius..radius {
            for y in -radius..radius {
                for z in -radius..radius {
                    data.insert((x, y, z).into(), Tile::new(rng.gen_range(0, 3)));
                    //data.insert((x, y, z).into(), Tile::new(Tile { terrain_texture_idx: 0 }));
                }
            }
        }
        Self { data, mouse_over: None }
    }

    pub fn render_outline(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        offset: &Point2<f32>,
    ) -> Result<()> {
        match self.mouse_over {
            Some(location) => {
                let location = AxialIndex::from(location);
                let (x, y) = location.hex_draw_location(&offset);
                let img = &assets.outline;
                let param = DrawParam::new().dest(Point2 { x: x.floor(), y: y.floor() });
                graphics::draw(ctx, img, param)
            }
            None => Ok(()),
        }
    }

    pub fn render_tiles(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        offset: &Point2<f32>,
        window_dimensions: (f32, f32),
    ) -> Result<()> {
        // render tile data
        let (win_x, win_y) = window_dimensions;

        for (location, tile) in self.data.iter() {
            let img = &assets.tiles[tile.terrain_texture_idx];
            let location = AxialIndex::from(*location);

            let (x, y) = location.hex_draw_location(&offset);

            // if the hex would be visible on screen, draw it
            if x > -140.0 && x < win_x && y > -140.0 && y < win_y {
                let param = DrawParam::new().dest(mint::Point2 { x: x.floor(), y: y.floor() });
                graphics::draw(ctx, img, param)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
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

impl AxialIndex {
    pub fn hex_draw_location(&self, offset: &Point2<f32>) -> (f32, f32) {
        let x: f32 = offset.x
            + self.q as f32 * TILE_DIMENSIONS.0
            + if self.r % 2 != 0 { TILE_DIMENSIONS.0 / 2.0 } else { 0.0 };
        let y: f32 = offset.y + self.r as f32 * TILE_DIMENSIONS.1 as f32 + self.r as f32 * -35.0;
        (x, y)
    }
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
