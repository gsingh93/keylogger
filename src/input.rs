// Constants, structs, and arrays derived from /linux/include/linux/input.h

const MAX_KEYS: u16 = 112;

const EV_KEY: u16 = 1;

const KEY_RELEASE: i32 = 0;
const KEY_PRESS: i32 = 1;

const KEY_LEFTSHIFT: u16 = 42;
const KEY_RIGHTSHIFT: u16 = 43;

#[derive(Debug)]
#[repr(C)]
pub struct InputEvent {
    tv_sec: isize, // from timeval struct
    tv_usec: isize, // from timeval struct
    pub type_: u16,
    pub code: u16,
    pub value: i32
}

// Unknown key string
const UK: &'static str = "<UK>";

const KEY_NAMES: [&'static str; MAX_KEYS as usize] = [
    UK, "<ESC>",
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=",
    "<Backspace>", "<Tab>",
    "q", "w", "e", "r", "t", "y", "u", "i", "o", "p",
    "[", "]", "<Enter>", "<LCtrl>",
    "a", "s", "d", "f", "g", "h", "j", "k", "l", ";",
    "'", "`", "<LShift>",
    "\\", "z", "x", "c", "v", "b", "n", "m", ",", ".", "/",
    "<RShift>",
    "<KP*>",
    "<LAlt>", " ", "<CapsLock>",
    "<F1>", "<F2>", "<F3>", "<F4>", "<F5>", "<F6>", "<F7>", "<F8>", "<F9>", "<F10>",
    "<NumLock>", "<ScrollLock>",
    "<KP7>", "<KP8>", "<KP9>",
    "<KP->",
    "<KP4>", "<KP5>", "<KP6>",
    "<KP+>",
    "<KP1>", "<KP2>", "<KP3>", "<KP0>",
    "<KP.>",
    UK, UK, UK,
    "<F11>", "<F12>",
    UK, UK, UK, UK, UK, UK, UK,
    "<KPEnter>", "<RCtrl>", "<KP/>", "<SysRq>", "<RAlt>", UK,
    "<Home>", "<Up>", "<PageUp>", "<Left>", "<Right>", "<End>", "<Down>",
    "<PageDown>", "<Insert>", "<Delete>"
];

const SHIFT_KEY_NAMES: [&'static str; MAX_KEYS as usize] = [
    UK, "<ESC>",
    "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "_", "+",
    "<Backspace>", "<Tab>",
    "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P",
    "{", "}", "<Enter>", "<LCtrl>",
    "A", "S", "D", "F", "G", "H", "J", "K", "L", ":",
    "\"", "~", "<LShift>",
    "|", "Z", "X", "C", "V", "B", "N", "M", "<", ">", "?",
    "<RShift>",
    "<KP*>",
    "<LAlt>", " ", "<CapsLock>",
    "<F1>", "<F2>", "<F3>", "<F4>", "<F5>", "<F6>", "<F7>", "<F8>", "<F9>", "<F10>",
    "<NumLock>", "<ScrollLock>",
    "<KP7>", "<KP8>", "<KP9>",
    "<KP->",
    "<KP4>", "<KP5>", "<KP6>",
    "<KP+>",
    "<KP1>", "<KP2>", "<KP3>", "<KP0>",
    "<KP.>",
    UK, UK, UK,
    "<F11>", "<F12>",
    UK, UK, UK, UK, UK, UK, UK,
    "<KPEnter>", "<RCtrl>", "<KP/>", "<SysRq>", "<RAlt>", UK,
    "<Home>", "<Up>", "<PageUp>", "<Left>", "<Right>", "<End>", "<Down>",
    "<PageDown>", "<Insert>", "<Delete>"
];

// Converts a key code to it's ascii representation. Some unprintable keys like escape are printed
// as a name between angled brackets, i.e. <ESC>
pub fn get_key_text(code: u16, shift_pressed: u8) -> &'static str {
    let arr = if shift_pressed != 0 {
        SHIFT_KEY_NAMES
    } else {
        KEY_NAMES
    };

    if code < MAX_KEYS {
        return arr[code as usize];
    } else {
        debug!("Unknown key: {}", code);
        return UK;
    }
}

// Determines whether the given key code is a shift
pub fn is_shift(code: u16) -> bool {
    code == KEY_LEFTSHIFT || code == KEY_RIGHTSHIFT
}

pub fn is_key_event(type_: u16) -> bool {
    type_ == EV_KEY
}

pub fn is_key_press(value: i32) -> bool {
    value == KEY_PRESS
}

pub fn is_key_release(value: i32) -> bool {
    value == KEY_RELEASE
}
