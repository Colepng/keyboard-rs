use crate::keycode::Keycode;

pub struct Key {
    pub col: Option<usize>,
    pub row: Option<usize>,
    pub keycode: Keycode,
    pub encoder: bool,
}
