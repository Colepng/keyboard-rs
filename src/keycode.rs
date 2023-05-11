use self::Keycodes::*;
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum Keycodes {
    KC_A = 0x04,
    KC_B = 0x05,
    KC_C = 0x06,
    KC_D = 0x07,
    KC_E = 0x08,
    KC_F = 0x09,
    KC_G = 0x0a,
    KC_H = 0x0b,
    KC_I = 0x0c,
    KC_J = 0x0d,
    KC_K = 0x0e,
    KC_L = 0x0f,
    KC_M = 0x10,
    KC_N = 0x11,
    KC_O = 0x12,
    KC_P = 0x13,
    KC_Q = 0x14,
    KC_R = 0x15,
    KC_S = 0x16,
    KC_T = 0x17,
    KC_U = 0x18,
    KC_V = 0x19,
    KC_W = 0x1a,
    KC_X = 0x1b,
    KC_Y = 0x1c,
    KC_Z = 0x1d,

    KC_1 = 0x1e,
    KC_2 = 0x1f,
    KC_3 = 0x20,
    KC_4 = 0x21,
    KC_5 = 0x22,
    KC_6 = 0x23,
    KC_7 = 0x24,
    KC_8 = 0x25,
    KC_9 = 0x26,
    KC_0 = 0x27,

    KC_MUTE = 0x7f,
    KC_VOLUP = 0x80,
    KC_VOLDOWN = 0x81,

    KC_LAYER(u8),
}

// impl From<u8> for Keycodes {
//     fn from(value: u8) -> Self {
//         match value {
//         }
//     }
// }

impl TryInto<u8> for Keycodes {
    type Error = &'static str;
    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            KC_A => Ok(0x04),
            KC_B => Ok(0x05),
            KC_C => Ok(0x06),
            KC_D => Ok(0x07),
            KC_E => Ok(0x08),
            KC_F => Ok(0x09),
            KC_G => Ok(0x0a),
            KC_H => Ok(0x0b),
            KC_I => Ok(0x0c),
            KC_J => Ok(0x0d),
            KC_K => Ok(0x0e),
            KC_L => Ok(0x0f),
            KC_M => Ok(0x10),
            KC_N => Ok(0x11),
            KC_O => Ok(0x12),
            KC_P => Ok(0x13),
            KC_Q => Ok(0x14),
            KC_R => Ok(0x15),
            KC_S => Ok(0x16),
            KC_T => Ok(0x17),
            KC_U => Ok(0x18),
            KC_V => Ok(0x19),
            KC_W => Ok(0x1a),
            KC_X => Ok(0x1b),
            KC_Y => Ok(0x1c),
            KC_Z => Ok(0x1d),

            KC_1 => Ok(0x1e),
            KC_2 => Ok(0x1f),
            KC_3 => Ok(0x20),
            KC_4 => Ok(0x21),
            KC_5 => Ok(0x22),
            KC_6 => Ok(0x23),
            KC_7 => Ok(0x24),
            KC_8 => Ok(0x25),
            KC_9 => Ok(0x26),
            KC_0 => Ok(0x27),

            KC_MUTE => Ok(0x7f),
            KC_VOLUP => Ok(0x80),
            KC_VOLDOWN => Ok(0x81),

            _ =>Err("Can't convert non usb key code"),
        }
    }
}
