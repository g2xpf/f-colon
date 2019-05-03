use crate::utils::KeyInput;

pub struct WorldStatus {
    pub window_should_close: bool,
    pub key: KeyInput,
}

impl WorldStatus {
    pub fn new() -> Self {
        WorldStatus {
            window_should_close: false,
            key: KeyInput::new(),
        }
    }

    pub fn reset(&mut self) {
        self.window_should_close = false;
    }
}
