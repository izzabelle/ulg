use crate::utils::AxialIndex;
use crate::Assets;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::Context;
use ggez::GameResult as Result;
use mint::Point2;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Map {
    pub data: HashMap<AxialIndex, Tile>,
    pub mouse_over: Option<AxialIndex>,
    pub view_offset: Point2<f32>,
    pub hex_size: f32,
}

impl Map {
    pub fn new(hex_size: f32, radius: isize, offset: Point2<f32>) -> Self {
        let mut data: HashMap<AxialIndex, Tile> = HashMap::new();
        let mut rng = rand::thread_rng();

        for q in -radius..radius {
            for r in -radius..radius {
                data.insert(AxialIndex { q, r }, Tile::new(rng.gen_range(0, 3)));
            }
        }

        Self { data, mouse_over: None, view_offset: offset, hex_size }
    }

    pub fn render_tiles(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        window_dimensions: (f32, f32),
    ) -> Result<()> {
        // render tile data
        let (win_x, win_y) = window_dimensions;

        for (location, tile) in self.data.iter() {
            let img = &assets.tiles[tile.terrain_texture_idx];
            let location =
                AxialIndex::from(*location).hex_to_pixel(self.hex_size, Some(&self.view_offset));

            // if the hex would be visible on screen, draw it
            if location.x > -140.0
                && location.x < win_x
                && location.y > -140.0
                && location.y < win_y
            {
                let param = DrawParam::new().dest(location);
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
