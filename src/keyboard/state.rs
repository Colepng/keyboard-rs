use crate::keycode::Keycode;

struct Layout<'a> {
    layout: &'a [&'a [&'a [Keycode]]],
}

impl<'a> Layout<'a> {
    fn new(layout: &'a [&[&[Keycode]]]) -> Self {
        Self { layout }
    }
}

pub(super) struct State<'a, const NUM_OF_COLS: usize, const NUM_OF_ROWS: usize> {
    layout: Layout<'a>,
    layer: usize,
    override_keys: [[Option<u8>; NUM_OF_COLS]; NUM_OF_ROWS],
}

impl<'a, const NUM_OF_COLS: usize, const NUM_OF_ROWS: usize> State<'a, NUM_OF_COLS, NUM_OF_ROWS> {
    pub(super) fn new(layout: &'a [&[&[Keycode]]]) -> Self {
        Self {
            layout: Layout::new(layout),
            layer: 0,
            override_keys: [[None; NUM_OF_COLS]; NUM_OF_ROWS],
        }
    }

    pub(super) fn get_key(&self, row: usize, col: usize) -> Keycode {
        if let Some(layer) = self.override_keys[row][col] {
            self.layout.layout[layer as usize][row][col]
        } else {
            self.layout.layout[self.layer][row][col]
        }
    }

    // handles special press actions
    pub(super) fn on_press(&mut self, keycode: Keycode, row: usize, col: usize) {
        match keycode {
            Keycode::KC_MO(layer) => {
                self.override_keys[row][col] = Some(self.layer.clone() as u8);
                self.layer = layer;
            }
            Keycode::KC_LAYER(layer) => self.layer = layer,
            _ => {}
        }
    }

    // handles special release actions
    pub(super) fn on_release(&mut self, keycode: Keycode, row: usize, col: usize) {
        match keycode {
            Keycode::KC_MO(_) => {
                self.layer = self.override_keys[row][col].unwrap() as usize;
                self.override_keys[row][col] = None;
            }
            _ => {}
        }
    }

    pub(super) fn layer(&self) -> usize {
        self.layer
    }
}
