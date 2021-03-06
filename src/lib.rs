#![forbid(clippy::pedantic)]

// modules
mod assets;
mod config;
mod debug;
mod input;
mod map;
mod utils;

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

// consts
pub const TILE_DIMENSIONS: Point2<f32> = Point2 { x: 120.0, y: 140.0 };

// wrap main to throw errors easier
pub fn main_wrapper() -> Result<()> {
    let cb = ContextBuilder::new("Fucking Hexagons", "Isabelle L.")
        .add_resource_path("./resources")
        .window_setup(WindowSetup { title: "Fucking Hexagons".to_owned(), ..Default::default() })
        .window_mode(WindowMode { width: 1600.0, height: 900.0, ..Default::default() });
    let (mut ctx, mut event_loop) = cb.build()?;
    let mut ulg = Ulg::new(&mut ctx)?;
    event::run(&mut ctx, &mut event_loop, &mut ulg)
}

// primary game struct
pub struct Ulg {
    pub _config: Config,
    pub input_handler: InputHandler,
    pub window_dimension_x: f32,
    pub window_dimension_y: f32,
    pub assets: Assets,
    pub map: Map,
}

#[allow(dead_code)]
impl Ulg {
    fn new(ctx: &mut Context) -> Result<Self> {
        let map = Map::new(TILE_DIMENSIONS.x / 3.0_f32.sqrt(), 3, Point2 { x: 800.0, y: 450.0 });
        let config = Config::load("game_conf.toml");
        let assets = Assets::load(ctx, &config)?;
        Ok(Self {
            map,
            _config: config,
            assets,
            input_handler: InputHandler::new(),
            window_dimension_x: 1600.0,
            window_dimension_y: 900.0,
        })
    }

    fn window_dimensions_point2(&self) -> Point2<f32> {
        Point2 { x: self.window_dimension_x, y: self.window_dimension_y }
    }

    fn window_dimensions_tuple_f32(&self) -> (f32, f32) {
        (self.window_dimension_x, self.window_dimension_y)
    }
}

impl EventHandler for Ulg {
    fn update(&mut self, ctx: &mut Context) -> Result<()> {
        // mouse location
        self.input_handler.mouse_location_from_center(ctx, self.window_dimensions_tuple_f32());
        // map update
        self.map.update(ctx, &self.input_handler);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.map.render(ctx, &self.assets, self.window_dimensions_tuple_f32())?;
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
