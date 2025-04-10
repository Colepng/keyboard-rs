use crate::keycode::Keycode;

struct Layout<'a> {
    layout: &'a [&'a [&'a [Keycode]]],
}

impl<'a> Layout<'a> {
    const fn new(layout: &'a [&[&[Keycode]]]) -> Self {
        Self { layout }
    }
}

pub(super) struct State<'a, const NUM_OF_COLS: usize, const NUM_OF_ROWS: usize> {
    layout: Layout<'a>,
    layer: usize,
    override_keys: [[Option<usize>; NUM_OF_COLS]; NUM_OF_ROWS],
}

impl<'a, const NUM_OF_COLS: usize, const NUM_OF_ROWS: usize> State<'a, NUM_OF_COLS, NUM_OF_ROWS> {
    pub(super) const fn new(layout: &'a [&[&[Keycode]]]) -> Self {
        Self {
            layout: Layout::new(layout),
            layer: 0,
            override_keys: [[None; NUM_OF_COLS]; NUM_OF_ROWS],
        }
    }

    pub(super) fn get_key(&self, row: usize, col: usize) -> Keycode {
        self.override_keys[row][col].map_or_else(
            || {
                let mut keycode = self.layout.layout[self.layer][row][col];
                let mut layer = self.layer;
                while keycode == Keycode::KC_TRANS {
                    layer -= 1;
                    keycode = self.layout.layout[layer][row][col];
                }
                keycode
            },
            |layer| self.layout.layout[layer][row][col],
        )
    }

    // handles special press actions
    pub(super) const fn on_press(&mut self, keycode: Keycode, row: usize, col: usize) {
        match keycode {
            Keycode::KC_MO(layer) | Keycode::KC_LAYER(layer) => {
                self.override_keys[row][col] = Some(self.layer);
                self.layer = layer;
            }
            _ => {}
        }
    }

    // handles special release actions
    pub(super) const fn on_release(&mut self, keycode: Keycode, row: usize, col: usize) {
        #[allow(clippy::single_match)]
        match keycode {
            Keycode::KC_MO(_) => {
                self.layer = self.override_keys[row][col].unwrap();
                self.override_keys[row][col] = None;
            }
            Keycode::KC_LAYER(_) => {
                self.override_keys[row][col] = None;
            }
            _ => {}
        }
    }

    #[cfg(feature = "encoders")]
    pub(super) const fn layer(&self) -> usize {
        self.layer
    }
}
