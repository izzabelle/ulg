use crate::utils::point_from_center;
use crate::Ulg;
use ggez::graphics::{self, DrawParam, Scale, Text, TextFragment};
use ggez::Context;
use ggez::GameResult as Result;
use mint::Point2;

pub fn debug_print(game_state: &Ulg, ctx: &mut Context) -> Result<()> {
    let debug_info = format!(
        "
debug info:
============
fps: {:?}
mouse coord (from_center): {:?}
map offset (from_center): {:?}",
        ggez::timer::fps(ctx),
        game_state.input_handler.mouse_location_from_center,
        point_from_center(game_state.map.view_offset, game_state.window_dimensions_tuple_f32()),
    );
    let text = Text::new(TextFragment {
        text: debug_info,
        font: Some(game_state.assets.font),
        color: Some(graphics::BLACK),
        scale: Some(Scale::uniform(20.0)),
        ..Default::default()
    });
    let param = DrawParam::new().dest(Point2 { x: 20.0, y: 20.0 });
    graphics::draw(ctx, &text, param)
}
