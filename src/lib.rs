// modules
mod assets;
mod config;
mod debug;
mod input;
mod map;

// ggez namespacing
use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{self, EventHandler},
    graphics,
    input::mouse::MouseButton,
    Context, ContextBuilder, GameResult as Result,
};
// Ulg namespacing
use crate::assets::Assets;
use crate::config::Config;
use crate::input::ButtonState;
use crate::input::InputHandler;
use crate::map::Map;
use mint::Point2;

// wrap main to throw errors easier
pub fn main_wrapper() -> Result<()> {
    let cb = ContextBuilder::new("ULG", "Isabelle L.")
        .add_resource_path("./resources")
        .window_setup(WindowSetup { title: "U.L.G.".to_owned(), ..Default::default() })
        .window_mode(WindowMode { width: 1600.0, height: 900.0, ..Default::default() });
    let (mut ctx, mut event_loop) = cb.build()?;
    let mut ulg = Ulg::new(&mut ctx)?;
    event::run(&mut ctx, &mut event_loop, &mut ulg)
}

pub struct Ulg {
    pub _config: Config,
    pub input_handler: InputHandler,
    pub window_dimension_x: f32,
    pub window_dimension_y: f32,
    pub view_offset: Point2<f32>,
    pub assets: Assets,
    pub map: Map,
}

impl Ulg {
    fn new(ctx: &mut Context) -> Result<Self> {
        let map = Map::new(5);
        let config = Config::load("game_conf.toml");
        let assets = Assets::load(ctx, &config)?;
        let view_offset = Point2 { x: 0.0, y: 0.0 };
        Ok(Self {
            map,
            _config: config,
            assets,
            view_offset,
            input_handler: InputHandler::new(),
            window_dimension_x: 1600.0,
            window_dimension_y: 900.0,
        })
    }

    fn window_dimensions(&self) -> (f32, f32) {
        (self.window_dimension_x, self.window_dimension_y)
    }
}

impl EventHandler for Ulg {
    fn update(&mut self, ctx: &mut Context) -> Result<()> {
        // handle input

        // mouse location
        self.input_handler.mouse_location_from_center(ctx, self.window_dimensions());

        // move offset when left click button pressed down
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
        self.map.render_tiles(ctx, &self.assets, &self.view_offset, self.window_dimensions())?;
        self.map.render_outline(ctx, &self.assets, &self.view_offset)?;
        // dbg
        crate::debug::debug_print(&self, ctx)?;
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
