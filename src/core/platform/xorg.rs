use std::sync::Arc;

use x11::{
    keysym::*,
    xlib::{Display, KeyCode, ShiftMask, XOpenDisplay, XkbKeycodeToKeysym},
};
use x11rb::{
    connection::Connection,
    protocol::{xproto::*, Event},
    rust_connection::RustConnection,
    wrapper::ConnectionExt as _,
};

use crate::core::{
    event::ChannelSender,
    input::{InputState, Keys},
};

pub(super) struct Xorg {
    pub(super) connection: Arc<RustConnection>,
    pub(super) screen: Screen,
    pub(super) window: Window,
    pub(super) wm_protocols: u32,
    pub(super) wm_delete_window: u32,
    pub(super) display: *mut Display,
}

impl Xorg {
    pub(super) fn create_window(x: i16, y: i16, width: u16, height: u16) -> Xorg {
        let (conn, screen_num) = x11rb::connect(None).unwrap();
        let conn1 = std::sync::Arc::new(conn);
        let conn = &conn1;
        let screen = conn.setup().roots[screen_num].clone();
        let window = conn.generate_id().unwrap();

        let gc_id = conn.generate_id().unwrap();

        let wm_protocols = conn.intern_atom(false, b"WM_PROTOCOLS").unwrap();
        let wm_delete_window = conn.intern_atom(false, b"WM_DELETE_WINDOW").unwrap();
        let net_wm_name = conn.intern_atom(false, b"_NET_WM_NAME").unwrap();
        let utf8_string = conn.intern_atom(false, b"UTF8_STRING").unwrap();
        let wm_protocols = wm_protocols.reply().unwrap().atom;
        let wm_delete_window = wm_delete_window.reply().unwrap().atom;
        let net_wm_name = net_wm_name.reply().unwrap().atom;
        let utf8_string = utf8_string.reply().unwrap().atom;

        let win_aux = CreateWindowAux::new()
            .event_mask(
                EventMask::EXPOSURE
                    | EventMask::STRUCTURE_NOTIFY
                    | EventMask::BUTTON_PRESS
                    | EventMask::BUTTON_RELEASE
                    | EventMask::BUTTON_MOTION
                    | EventMask::POINTER_MOTION
                    | EventMask::POINTER_MOTION_HINT
                    | EventMask::KEY_PRESS
                    | EventMask::KEY_RELEASE
                    | EventMask::LEAVE_WINDOW
                    | EventMask::FOCUS_CHANGE
                    | EventMask::ENTER_WINDOW,
            )
            .background_pixel(screen.white_pixel)
            .win_gravity(Gravity::NORTH_WEST);

        let gc_aux = CreateGCAux::new().foreground(screen.black_pixel);

        conn.create_window(
            screen.root_depth,
            window,
            screen.root,
            x,
            y,
            width,
            height,
            0,
            WindowClass::INPUT_OUTPUT,
            0,
            &win_aux,
        )
        .unwrap();

        let title = "Pincel";
        conn.change_property8(
            PropMode::REPLACE,
            window,
            AtomEnum::WM_NAME,
            AtomEnum::STRING,
            title.as_bytes(),
        )
        .unwrap();
        conn.change_property8(
            PropMode::REPLACE,
            window,
            net_wm_name,
            utf8_string,
            title.as_bytes(),
        )
        .unwrap();
        conn.change_property32(
            PropMode::REPLACE,
            window,
            wm_protocols,
            AtomEnum::ATOM,
            &[wm_delete_window],
        )
        .unwrap();

        conn.change_property8(
            PropMode::REPLACE,
            window,
            AtomEnum::WM_CLASS,
            AtomEnum::STRING,
            b"pincel\0pincel\0",
        )
        .unwrap();

        let reply = conn
            .get_property(false, window, AtomEnum::WM_NAME, AtomEnum::STRING, 0, 1024)
            .unwrap();
        let reply = reply.reply().unwrap();
        assert_eq!(reply.value, title.as_bytes());

        conn.create_gc(gc_id, window, &gc_aux).unwrap();

        conn.map_window(window).unwrap();

        if let Err(err) = conn.flush() {
            println!("An error occured when flushing the stream: {}", err);
        }

        // let display = unsafe { XOpenDisplay(title.as_bytes().as_ptr() as *const i8) };
        let display = unsafe { XOpenDisplay(std::ptr::null::<i8>()) };

        Xorg {
            connection: conn.clone(),
            screen,
            window,
            wm_protocols,
            wm_delete_window,
            display,
        }
    }

    pub(super) fn pump_messages(&mut self, channel: ChannelSender, mut input: InputState) -> bool {
        let mut quit_flagged = false;
        let conn = &self.connection;
        let event = conn.wait_for_event().unwrap();
        match event {
            Event::KeyPress(event) => {
                let key_sym = unsafe {
                    let code = event.detail;
                    XkbKeycodeToKeysym(
                        self.display,
                        code as KeyCode, // event.xkey.keycode,
                        0,
                        if (code as u32 & ShiftMask) != 0 { 1 } else { 0 },
                    )
                };
                let key = translate_keycode(key_sym as u32);
                println!("Received key: {:?}", key);
                input.input_process_key(channel.clone(), key, true)
            }
            Event::Expose(_event) => {}
            Event::ConfigureNotify(_event) => {}
            Event::ClientMessage(event) => {
                let data = event.data.as_data32();
                if event.format == 32
                    && event.window == self.window
                    && data[0] == self.wm_delete_window
                {
                    println!("Window was asked to close");
                    quit_flagged = true;
                }
            }
            Event::Error(_) => println!("Got an unexpected error"),
            _ => println!("Got an unknown event"),
        }

        !quit_flagged
    }
}

// Key translation
fn translate_keycode(x_keycode: u32) -> Keys {
    #[allow(non_upper_case_globals)]
    match x_keycode {
        XK_BackSpace => Keys::BACKSPACE,
        XK_Return => Keys::ENTER,
        XK_Tab => Keys::TAB,
        // XK_Shift:  Keys::SHIFT
        // XK_Control:  Keys::CONTROL
        XK_Pause => Keys::PAUSE,
        XK_Caps_Lock => Keys::CAPITAL,

        XK_Escape => Keys::ESCAPE,

        // Not supported
        //  :  Keys::CONVERT
        //  :  Keys::NONCONVERT
        //  :  Keys::ACCEPT
        XK_Mode_switch => Keys::MODECHANGE,

        XK_space => Keys::SPACE,
        XK_Prior => Keys::PRIOR,
        XK_Next => Keys::NEXT,
        XK_End => Keys::END,
        XK_Home => Keys::HOME,
        XK_Left => Keys::LEFT,
        XK_Up => Keys::UP,
        XK_Right => Keys::RIGHT,
        XK_Down => Keys::DOWN,
        XK_Select => Keys::SELECT,
        XK_Print => Keys::PRINT,
        XK_Execute => Keys::EXECUTE,
        //  XK_snapshot=>  Keys::SNAPSHOT // not supported
        XK_Insert => Keys::INSERT,
        XK_Delete => Keys::DELETE,
        XK_Help => Keys::HELP,

        XK_Meta_L => Keys::LWIN, // TODO=> not sure this is right
        XK_Meta_R => Keys::RWIN,
        //  XK_apps=>  Keys::APPS // not supported

        //  XK_sleep=>  Keys::SLEEP //not supported
        XK_KP_0 => Keys::NUMPAD0,
        XK_KP_1 => Keys::NUMPAD1,
        XK_KP_2 => Keys::NUMPAD2,
        XK_KP_3 => Keys::NUMPAD3,
        XK_KP_4 => Keys::NUMPAD4,
        XK_KP_5 => Keys::NUMPAD5,
        XK_KP_6 => Keys::NUMPAD6,
        XK_KP_7 => Keys::NUMPAD7,
        XK_KP_8 => Keys::NUMPAD8,
        XK_KP_9 => Keys::NUMPAD9,
        XK_multiply => Keys::MULTIPLY,
        XK_KP_Add => Keys::ADD,
        XK_KP_Separator => Keys::SEPARATOR,
        XK_KP_Subtract => Keys::SUBTRACT,
        XK_KP_Decimal => Keys::DECIMAL,
        XK_KP_Divide => Keys::DIVIDE,
        XK_F1 => Keys::F1,
        XK_F2 => Keys::F2,
        XK_F3 => Keys::F3,
        XK_F4 => Keys::F4,
        XK_F5 => Keys::F5,
        XK_F6 => Keys::F6,
        XK_F7 => Keys::F7,
        XK_F8 => Keys::F8,
        XK_F9 => Keys::F9,
        XK_F10 => Keys::F10,
        XK_F11 => Keys::F11,
        XK_F12 => Keys::F12,
        XK_F13 => Keys::F13,
        XK_F14 => Keys::F14,
        XK_F15 => Keys::F15,
        XK_F16 => Keys::F16,
        XK_F17 => Keys::F17,
        XK_F18 => Keys::F18,
        XK_F19 => Keys::F19,
        XK_F20 => Keys::F20,
        XK_F21 => Keys::F21,
        XK_F22 => Keys::F22,
        XK_F23 => Keys::F23,
        XK_F24 => Keys::F24,

        XK_Num_Lock => Keys::NUMLOCK,
        XK_Scroll_Lock => Keys::SCROLL,

        XK_KP_Equal => Keys::NUMPAD_EQUAL,

        XK_Shift_L => Keys::LSHIFT,
        XK_Shift_R => Keys::RSHIFT,
        XK_Control_L => Keys::LCONTROL,
        XK_Control_R => Keys::RCONTROL,
        //  XK_Menu=>  Keys::LMENU
        XK_Menu => Keys::RMENU,

        XK_semicolon => Keys::SEMICOLON,
        XK_plus => Keys::PLUS,
        XK_comma => Keys::COMMA,
        XK_minus => Keys::MINUS,
        XK_period => Keys::PERIOD,
        XK_slash => Keys::SLASH,
        XK_grave => Keys::GRAVE,

        XK_a | XK_A => Keys::A,
        XK_b | XK_B => Keys::B,
        XK_c | XK_C => Keys::C,
        XK_d | XK_D => Keys::D,
        XK_e | XK_E => Keys::E,
        XK_f | XK_F => Keys::F,
        XK_g | XK_G => Keys::G,
        XK_h | XK_H => Keys::H,
        XK_i | XK_I => Keys::I,
        XK_j | XK_J => Keys::J,
        XK_k | XK_K => Keys::K,
        XK_l | XK_L => Keys::L,
        XK_m | XK_M => Keys::M,
        XK_n | XK_N => Keys::N,
        XK_o | XK_O => Keys::O,
        XK_p | XK_P => Keys::P,
        XK_q | XK_Q => Keys::Q,
        XK_r | XK_R => Keys::R,
        XK_s | XK_S => Keys::S,
        XK_t | XK_T => Keys::T,
        XK_u | XK_U => Keys::U,
        XK_v | XK_V => Keys::V,
        XK_w | XK_W => Keys::W,
        XK_x | XK_X => Keys::X,
        XK_y | XK_Y => Keys::Y,
        XK_z | XK_Z => Keys::Z,

        _ => Keys::KEYS_MAX_KEYS,
    }
}
