// modules
mod assets;
mod config;
mod map;

// ggez namespacing
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::GameResult as Result;
use ggez::{Context, ContextBuilder};
// Ulg namespacing
use crate::assets::Assets;
use crate::config::Config;
use crate::map::Map;

// wrap main to throw errors easier
pub fn main_wrapper() -> Result<()> {
    let cb = ContextBuilder::new("ULG", "Isabelle L.").add_resource_path("./assets");
    let (mut ctx, mut event_loop) = cb.build()?;
    let mut ulg = Ulg::new(&mut ctx)?;
    event::run(&mut ctx, &mut event_loop, &mut ulg)
}

struct Ulg {
    config: Config,
    assets: Assets,
    map: Map,
}

impl Ulg {
    fn new(ctx: &mut Context) -> Result<Self> {
        let map = Map::new(5);
        let config = Config::load("game_conf.toml");
        let assets = Assets::load(ctx, &config)?;
        Ok(Self { map, config, assets })
    }
}

impl EventHandler for Ulg {
    fn update(&mut self, _ctx: &mut Context) -> Result<()> {
        // nothing here yet
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<()> {
        graphics::clear(ctx, graphics::WHITE);
        graphics::draw(
            ctx,
            &self.assets.tiles[0],
            graphics::DrawParam::new().dest(mint::Point2 { x: 0.0, y: 0.0 }),
        )?;
        graphics::present(ctx)?;
        Ok(())
    }
}
