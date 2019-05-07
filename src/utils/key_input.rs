use super::KeyCode;
use std::fmt;

pub struct KeyInput {
    key_code: KeyCode,
}

impl fmt::Debug for KeyInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KeyInput: {:?}", self.key_code)
    }
}

impl KeyInput {
    pub fn new() -> Self {
        KeyInput {
            key_code: KeyCode::NONE,
        }
    }

    pub fn contains(&self, code: KeyCode) -> bool {
        self.key_code.contains(code)
    }

    fn convert(glutin_keycode: u32) -> Option<KeyCode> {
        let keycode = match glutin_keycode {
            1 => KeyCode::ESC,
            28 => KeyCode::ENTER,
            57 => KeyCode::SPACE,
            44 => KeyCode::Z,
            103 => KeyCode::UP,
            105 => KeyCode::LEFT,
            106 => KeyCode::RIGHT,
            108 => KeyCode::DOWN,
            _ => KeyCode::NONE,
        };

        if keycode == KeyCode::NONE {
            None
        } else {
            Some(keycode)
        }
    }

    pub fn insert(&mut self, glutin_keycode: u32) {
        if let Some(key) = KeyInput::convert(glutin_keycode) {
            self.key_code.insert(key);
        }
    }

    pub fn remove(&mut self, glutin_keycode: u32) {
        if let Some(key) = KeyInput::convert(glutin_keycode) {
            self.key_code.remove(key);
        }
    }
}
