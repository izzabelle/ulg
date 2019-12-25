use crate::utils::point_from_center;
use ggez::input::mouse::MouseButton;
use ggez::Context;
use mint::Point2;

pub enum ButtonState {
    Down,
    Up,
}

pub struct InputHandler {
    pub left_click_down: bool,
    pub mouse_location_from_center: Point2<f32>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self { left_click_down: false, mouse_location_from_center: Point2 { x: 0.0, y: 0.0 } }
    }

    pub fn mouse_button_input(&mut self, button: MouseButton, button_state: ButtonState) {
        match button {
            MouseButton::Left => match button_state {
                ButtonState::Down => self.left_click_down = false,
                ButtonState::Up => self.left_click_down = true,
            },
            _ => {}
        }
    }

    pub fn mouse_location_from_center(&mut self, ctx: &mut Context, window_dimensions: (f32, f32)) {
        let mouse_pos = ggez::input::mouse::position(ctx);
        self.mouse_location_from_center = point_from_center(mouse_pos, window_dimensions);
    }
}
