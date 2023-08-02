use crate::keycode::Keycodes;

pub struct Key {
    pub col: Option<usize>,
    pub row: Option<usize>,
    pub keycode: Keycodes,
    pub encoder: bool,
}
