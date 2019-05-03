bitflags! {
    pub struct KeyCode: u64 {
        const NONE  = 0;
        const ENTER = 1 << 0;
        const RIGHT = 1 << 1;
        const LEFT  = 1 << 2;
        const UP    = 1 << 3;
        const DOWN  = 1 << 4;
        const ESC   = 1 << 5;
        const SPACE = 1 << 6;
        const Z     = 1 << 7;
    }
}
