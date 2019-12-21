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
use mint::Point2;

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
    view_offset: Point2<f32>,
    assets: Assets,
    map: Map,
}

impl Ulg {
    fn new(ctx: &mut Context) -> Result<Self> {
        let map = Map::new(10);
        let config = Config::load("game_conf.toml");
        let assets = Assets::load(ctx, &config)?;
        let view_offset = Point2 { x: 400.0, y: 300.0 };
        Ok(Self { map, _config: config, assets, view_offset, input_handler: InputHandler::new() })
    }
}

impl EventHandler for Ulg {
    fn update(&mut self, ctx: &mut Context) -> Result<()> {
        if self.input_handler.left_click_down {
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

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.map.render(ctx, &self.assets, &self.view_offset)?;
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
