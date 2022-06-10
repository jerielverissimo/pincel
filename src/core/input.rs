use num_derive::FromPrimitive;

use super::event::{ChannelSender, Data, EventContext, Message, SystemEventCode};

#[derive(Debug, Copy, Clone)]
pub struct KeyboardState {
    keys: [bool; 258],
}

#[derive(Debug, Copy, Clone)]
pub struct MouseState {
    x: i16,
    y: i16,
    buttons: [bool; 5],
}

#[derive(Debug, Copy, Clone)]
pub struct InputState {
    keyboard_current: KeyboardState,
    keyboard_previous: KeyboardState,
    mouse_current: MouseState,
    mouse_previous: MouseState,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            keyboard_current: KeyboardState { keys: [false; 258] },
            keyboard_previous: KeyboardState { keys: [false; 258] },
            mouse_current: MouseState {
                x: 0,
                y: 0,
                buttons: [false; 5],
            },
            mouse_previous: MouseState {
                x: 0,
                y: 0,
                buttons: [false; 5],
            },
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        // Copy current states to previous states.
        self.keyboard_previous = self.keyboard_current;
        self.mouse_previous = self.mouse_current;
    }

    pub fn process_key(&mut self, channel: ChannelSender, key: Keys, pressed: bool) {
        // Only handle this if the state actually changed.
        if self.keyboard_current.keys[key as usize] != pressed {
            // Update internal state.
            self.keyboard_current.keys[key as usize] = pressed;

            // Fire off an event for immediate processing.
            let mut context = EventContext {
                data: Data { u16: [0; 8] },
            };
            unsafe {
                context.data.u16[0] = key as u16;
            }
            channel.send(Message::Pub {
                code: if pressed {
                    SystemEventCode::CODE_KEY_PRESSED
                } else {
                    SystemEventCode::CODE_KEY_RELEASED
                },
                sender: None,
                context,
                channel: channel.clone(),
            });
        }
    }

    pub fn process_button(&mut self, channel: ChannelSender, button: Buttons, pressed: bool) {
        // If the state changed, fire an event.
        if self.mouse_current.buttons[button as usize] != pressed {
            // Update internal state.
            self.mouse_current.buttons[button as usize] = pressed;

            // Fire the event.
            let mut context = EventContext {
                data: Data { u16: [0; 8] },
            };
            unsafe {
                context.data.u16[0] = button as u16;
            }
            channel.send(Message::Pub {
                code: if pressed {
                    SystemEventCode::CODE_BUTTON_PRESSED
                } else {
                    SystemEventCode::CODE_BUTTON_RELEASED
                },
                sender: None,
                context,
                channel: channel.clone(),
            });
        }
    }

    pub fn process_mouse_move(&mut self, channel: ChannelSender, x: i16, y: i16) {
        // If the state changed, fire an event.
        if self.mouse_current.x != x || self.mouse_current.y != y {
            self.mouse_current.x = x;
            self.mouse_current.y = y;
        }

        // Fire the event.
        let mut context = EventContext {
            data: Data { i16: [0; 8] },
        };
        unsafe {
            context.data.i16[0] = x;
            context.data.i16[1] = y;
        }
        channel.send(Message::Pub {
            code: SystemEventCode::CODE_MOUSE_MOVED,
            sender: None,
            context,
            channel: channel.clone(),
        });
    }
}

pub const LEFT_MOUSE_BUTTON: u8 = 1;
pub const MIDDLE_MOUSE_BUTTON: u8 = 2;
pub const RIGHT_MOUSE_BUTTON: u8 = 3;
pub const SCROLL_WHEEL_UP: u8 = 4;
pub const SCROLL_WHEEL_DOWN: u8 = 5;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, FromPrimitive, PartialEq, Eq, Debug)]
pub enum Buttons {
    BUTTON_LEFT,
    BUTTON_RIGHT,
    BUTTON_MIDDLE,
    BUTTON_SCROLL_WHEEL_UP,
    BUTTON_SCROLL_WHEEL_DOWN,
    BUTTON_MAX_BUTTONS,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, FromPrimitive, PartialEq, Eq, Debug)]
pub enum Keys {
    BACKSPACE = 0x08,
    ENTER = 0x0D,
    TAB = 0x09,
    SHIFT = 0x10,
    CONTROL = 0x11,

    PAUSE = 0x13,
    CAPITAL = 0x14,

    ESCAPE = 0x1B,

    CONVERT = 0x1C,
    NONCONVERT = 0x1D,
    ACCEPT = 0x1E,
    MODECHANGE = 0x1F,

    SPACE = 0x20,
    PRIOR = 0x21,
    NEXT = 0x22,
    END = 0x23,
    HOME = 0x24,
    LEFT = 0x25,
    UP = 0x26,
    RIGHT = 0x27,
    DOWN = 0x28,
    SELECT = 0x29,
    PRINT = 0x2A,
    EXECUTE = 0x2B,
    SNAPSHOT = 0x2C,
    INSERT = 0x2D,
    DELETE = 0x2E,
    HELP = 0x2F,

    A = 0x41,
    B = 0x42,
    C = 0x43,
    D = 0x44,
    E = 0x45,
    F = 0x46,
    G = 0x47,
    H = 0x48,
    I = 0x49,
    J = 0x4A,
    K = 0x4B,
    L = 0x4C,
    M = 0x4D,
    N = 0x4E,
    O = 0x4F,
    P = 0x50,
    Q = 0x51,
    R = 0x52,
    S = 0x53,
    T = 0x54,
    U = 0x55,
    V = 0x56,
    W = 0x57,
    X = 0x58,
    Y = 0x59,
    Z = 0x5A,

    LWIN = 0x5B,
    RWIN = 0x5C,
    APPS = 0x5D,

    SLEEP = 0x5F,

    NUMPAD0 = 0x60,
    NUMPAD1 = 0x61,
    NUMPAD2 = 0x62,
    NUMPAD3 = 0x63,
    NUMPAD4 = 0x64,
    NUMPAD5 = 0x65,
    NUMPAD6 = 0x66,
    NUMPAD7 = 0x67,
    NUMPAD8 = 0x68,
    NUMPAD9 = 0x69,
    MULTIPLY = 0x6A,
    ADD = 0x6B,
    SEPARATOR = 0x6C,
    SUBTRACT = 0x6D,
    DECIMAL = 0x6E,
    DIVIDE = 0x6F,
    F1 = 0x70,
    F2 = 0x71,
    F3 = 0x72,
    F4 = 0x73,
    F5 = 0x74,
    F6 = 0x75,
    F7 = 0x76,
    F8 = 0x77,
    F9 = 0x78,
    F10 = 0x79,
    F11 = 0x7A,
    F12 = 0x7B,
    F13 = 0x7C,
    F14 = 0x7D,
    F15 = 0x7E,
    F16 = 0x7F,
    F17 = 0x80,
    F18 = 0x81,
    F19 = 0x82,
    F20 = 0x83,
    F21 = 0x84,
    F22 = 0x85,
    F23 = 0x86,
    F24 = 0x87,

    NUMLOCK = 0x90,
    SCROLL = 0x91,

    NUMPAD_EQUAL = 0x92,

    LSHIFT = 0xA0,
    RSHIFT = 0xA1,
    LCONTROL = 0xA2,
    RCONTROL = 0xA3,
    LMENU = 0xA4,
    RMENU = 0xA5,

    SEMICOLON = 0xBA,
    PLUS = 0xBB,
    COMMA = 0xBC,
    MINUS = 0xBD,
    PERIOD = 0xBE,
    SLASH = 0xBF,
    GRAVE = 0xC0,

    KEYS_MAX_KEYS,
}
