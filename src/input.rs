use ggez::input::mouse::MouseButton;

pub enum ButtonState {
    Down,
    Up,
}

pub struct InputHandler {
    pub left_click_down: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        Self { left_click_down: false }
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
}
