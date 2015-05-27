use std::process::Command;
use std::fs::File;
use std::io::Read;

// Constants, structs, and arrays derived from /linux/include/linux/input.h

const EV_KEY: u16 = 1;

const KEY_RELEASE: i32 = 0;
const KEY_PRESS: i32 = 1;

const KEY_LEFTSHIFT: u16 = 42;
const KEY_RIGHTSHIFT: u16 = 43;

const MAX_KEYS: u16 = 112;


#[derive(Debug)]
#[repr(C)]
struct InputEvent {
    tv_sec: isize, // from timeval struct
    tv_usec: isize, // from timeval struct
    type_: u16,
    code: u16,
    value: i32
}

// Unknown key string
const UK: &'static str = "";

const key_names: [&'static str; MAX_KEYS as usize] = [
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

const shift_key_names: [&'static str; MAX_KEYS as usize] = [
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

// Determines whether the given key code is a shift
fn is_shift(code: u16) -> bool {
    code == KEY_LEFTSHIFT || code == KEY_RIGHTSHIFT
}

// Converts a key code to it's ascii representation. Some unprintable keys like escape are printed
// as a name between angled brackets, i.e. <ESC>
fn get_key_text(code: u16, shift_pressed: u8) -> &'static str {
    let arr = if shift_pressed != 0 {
        shift_key_names
    } else {
        key_names
    };

    if code < MAX_KEYS {
        return arr[code as usize];
    } else {
        println!("Unknown key: {}", code); // TODO
        return UK;
    }
}


// Detects and returns the name of the keyboard device file. This function uses
// the fact that all device information is shown in /proc/bus/input/devices and
// the keyboard device file should always have an EV of 120013
fn get_keyboard_device_filenames() -> Vec<String> {
    let mut command_str = "grep -E 'Handlers|EV' /proc/bus/input/devices".to_string();
    command_str.push_str("| grep -B1 120013");
    command_str.push_str("| grep -Eo event[0-9]+");

    let res = Command::new("sh").arg("-c").arg(command_str).output().unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let res_str = std::str::from_utf8(&res.stdout).unwrap();

    let mut filenames = Vec::new();
    for file in res_str.trim().split('\n') {
        let mut filename = "/dev/input/".to_string();
        filename.push_str(file);
        filenames.push(filename);
    }
    filenames
}

fn main() {
    let filenames = get_keyboard_device_filenames();
    println!("{}", filenames[1]);
    let mut f = File::open(&filenames[1]).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let mut buf: [u8; 24] = unsafe { std::mem::zeroed() };

    // We use a u8 here instead of a bool to handle the rare case that both shift keys are pressed
    // and then one is released
    let mut shift_pressed = 0;
    loop {
        f.read(&mut buf).unwrap();
        let event: InputEvent = unsafe { std::mem::transmute(buf) };
        if event.type_ == EV_KEY {
            if event.value == KEY_PRESS {
                if is_shift(event.code) {
                    shift_pressed += 1;
                }
                println!("{:?}", get_key_text(event.code, shift_pressed));
            } else if event.value == KEY_RELEASE {
                if is_shift(event.code) {
                    shift_pressed -= 1;
                }
            }
        }
    }
}
