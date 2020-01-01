// a whole fuck load of includes
use crate::input::InputHandler;
use crate::utils::AxialIndex;
use crate::Assets;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::Context;
use ggez::GameResult as Result;
use mint::Point2;
use std::collections::HashMap;

/// map struct
#[derive(Debug)]
pub struct Map {
    pub data: HashMap<AxialIndex, Tile>,
    pub view_offset: Point2<f32>,
    pub hovered: Option<AxialIndex>,
    pub _selected: Option<AxialIndex>,
    pub hex_size: f32,
}

impl Map {
    /// create a new map and fill it with random tiles
    pub fn new(hex_size: f32, radius: isize, offset: Point2<f32>) -> Self {
        let mut data: HashMap<AxialIndex, Tile> = HashMap::new();

        for q in -radius..radius {
            for r in -radius..radius {
                data.insert(
                    AxialIndex { q, r },
                    Tile { terrain_texture_idx: None, tile_type: TileType::None },
                );
            }
        }

        Self { data, view_offset: offset, hex_size, _selected: None, hovered: None }
    }

    /// to be called in the ggez update function to do most updates to the map
    pub fn update(&mut self, ctx: &mut Context, input_handler: &InputHandler) {
        // move offset when left click button pressed down
        if input_handler.left_click_down {
            ggez::input::mouse::set_cursor_hidden(ctx, true);
            let m_delta = ggez::input::mouse::delta(ctx);
            if m_delta.x > 5.0 || m_delta.y > 5.0 || m_delta.x < -5.0 || m_delta.y < -5.0 {
                self.view_offset = Point2 {
                    x: self.view_offset.x + m_delta.x / 2.0,
                    y: self.view_offset.y + m_delta.y / 2.0,
                };
            }
        } else {
            ggez::input::mouse::set_cursor_hidden(ctx, false);
        }
        // find hovered
        self.find_hovered(&input_handler);
    }

    // finds out if the mouse is over a hex
    fn find_hovered(&mut self, input_handler: &InputHandler) {
        let hex = AxialIndex::from_pixel(input_handler.mouse_location_from_center, self.hex_size);
        match self.data.contains_key(&hex) {
            true => {
                self.hovered = Some(hex);
            }
            false => {
                self.hovered = None;
            }
        }
    }

    /// runs all rendering functions
    pub fn render(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        window_dimensions: (f32, f32),
    ) -> Result<()> {
        self.render_tiles(ctx, assets, window_dimensions)
    }

    /// renders the map tiles
    fn render_tiles(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        window_dimensions: (f32, f32),
    ) -> Result<()> {
        // render tile data
        let (win_x, win_y) = window_dimensions;

        for (location, tile) in self.data.iter() {
            /*let img = &assets.tiles[tile.terrain_texture_idx];*/
            let texture_idx = match tile.tile_type {
                TileType::None => continue,
                _ => tile.terrain_texture_idx.unwrap(),
            };
            let img = &assets.tiles[texture_idx];
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
pub enum TileType {
    None,
    Grass,
    Stone,
    Dirt,
}

/// a single tile in the map
#[derive(Debug)]
pub struct Tile {
    tile_type: TileType,
    terrain_texture_idx: Option<usize>,
}
