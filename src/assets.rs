use crate::config::Config;
use ggez::graphics::{Font, Image};
use ggez::Context;
use ggez::GameResult as Result;

pub struct Assets {
    pub tiles: Vec<Image>,
    pub outline: Image,
    pub font: Font,
}

impl Assets {
    pub fn load(ctx: &mut Context, config: &Config) -> Result<Self> {
        let mut tiles: Vec<Image> = Vec::new();
        for path in config.tile_assets.clone() {
            tiles.push(Image::new(ctx, &path)?);
        }
        let outline = Image::new(ctx, config.outline.clone())?;
        let font = Font::new(ctx, config.font.clone())?;
        Ok(Assets { tiles, outline, font })
    }
}
