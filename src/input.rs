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
        let screen_center = Point2 { x: window_dimensions.0 / 2.0, y: window_dimensions.1 / 2.0 };
        let (x, y): (f32, f32);
        if mouse_pos.x <= screen_center.x {
            x = -(screen_center.x - mouse_pos.x);
        } else {
            x = mouse_pos.x - screen_center.x;
        }

        if mouse_pos.y <= screen_center.y {
            y = -(screen_center.y - mouse_pos.y);
        } else {
            y = mouse_pos.y - screen_center.y;
        }

        self.mouse_location_from_center = Point2 { x, y };
    }
}
