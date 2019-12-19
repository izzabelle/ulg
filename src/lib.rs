// modules
mod assets;
mod config;
mod input;
mod map;

// ggez namespacing
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::input::mouse::MouseButton;
use ggez::GameResult as Result;
use ggez::{Context, ContextBuilder};
// Ulg namespacing
use crate::assets::Assets;
use crate::config::Config;
use crate::input::ButtonState;
use crate::input::InputHandler;
use crate::map::Map;

// wrap main to throw errors easier
pub fn main_wrapper() -> Result<()> {
    let cb = ContextBuilder::new("ULG", "Isabelle L.").add_resource_path("./resources");
    let (mut ctx, mut event_loop) = cb.build()?;
    let mut ulg = Ulg::new(&mut ctx)?;
    event::run(&mut ctx, &mut event_loop, &mut ulg)
}

struct Ulg {
    _config: Config,
    input_handler: InputHandler,
    view_position_x: isize,
    view_position_y: isize,
    assets: Assets,
    map: Map,
}

impl Ulg {
    fn new(ctx: &mut Context) -> Result<Self> {
        let map = Map::new(3);
        let config = Config::load("game_conf.toml");
        let assets = Assets::load(ctx, &config)?;
        let (view_position_x, view_position_y) = (400, 300);
        Ok(Self {
            map,
            _config: config,
            assets,
            view_position_x,
            view_position_y,
            input_handler: InputHandler::new(),
        })
    }
}

impl EventHandler for Ulg {
    fn update(&mut self, _ctx: &mut Context) -> Result<()> {
        // nothing here yet
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.map.render(ctx, &self.assets, (self.view_position_x, self.view_position_y))?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.input_handler.mouse_button_input(button, ButtonState::Up);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        self.input_handler.mouse_button_input(button, ButtonState::Down);
    }
}
