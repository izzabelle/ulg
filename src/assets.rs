use crate::config::Config;
use ggez::graphics::Image;
use ggez::Context;
use ggez::GameResult as Result;

pub struct Assets {
    pub tiles: Vec<Image>,
}

impl Assets {
    pub fn load(ctx: &mut Context, config: &Config) -> Result<Self> {
        let mut tiles: Vec<Image> = Vec::new();
        for path in config.tile_assets.clone() {
            tiles.push(Image::new(ctx, &path)?);
        }
        Ok(Assets { tiles })
    }
}
