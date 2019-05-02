use glium::{Display, Frame};

pub struct RenderContext {
    pub display: Display,
    pub frame: Option<Frame>,
}

impl RenderContext {
    pub fn new(display: Display) -> Self {
        RenderContext {
            display,
            frame: None,
        }
    }
}
