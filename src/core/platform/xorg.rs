use std::{ffi::CString, path::Path, sync::Arc, time::Instant};

use crate::core::{
    event::{ChannelSender, Data, EventContext, Message, SystemEventCode},
    input::{
        Buttons, InputState, Keys, LEFT_MOUSE_BUTTON, MIDDLE_MOUSE_BUTTON, RIGHT_MOUSE_BUTTON,
        SCROLL_WHEEL_DOWN, SCROLL_WHEEL_UP,
    },
    logger::{log_output, LogLevel},
    renderer::opengl::{
        camera::{self, TargetCamera},
        color_buffer::{self, ColorBuffer},
        cube::Cube,
        debug_lines::{DebugLines, PointMarker, Polyline, RayMarker},
        line::{self, Line},
        triangle::{self, Triangle},
        viewport::{self, Viewport},
    },
    resources::Resources,
};
use floating_duration::TimeAsFloat;
use gl::types::GLenum;
use nalgebra as na;
use x11::{
    glx::{
        self, __GLXcontextRec, GLX_BLUE_SIZE, GLX_DEPTH_SIZE, GLX_DOUBLEBUFFER, GLX_GREEN_SIZE,
        GLX_RED_SIZE, GLX_RGBA, GLX_SAMPLES, GLX_SAMPLE_BUFFERS, GLX_STENCIL_SIZE,
    },
    keysym::*,
    xlib::{
        Display, KeyCode, ShiftMask, VisualID, VisualIDMask, XDefaultScreen, XOpenDisplay,
        XVisualInfo, XkbKeycodeToKeysym,
    },
    xlib_xcb::XSetEventQueueOwner,
};
use x11rb::{
    connection::Connection,
    protocol::{xinput::BUTTON_PRESS_EVENT, xproto::*, Event},
    wrapper::ConnectionExt as _,
};
use x11rb::{protocol::xproto::ConnectionExt, xcb_ffi::XCBConnection};

const GL_TRUE: i32 = 1;
const GL_FALSE: i32 = 0;

const GL_DEPTH_TEST: GLenum = 0x0B71;

pub(super) struct Xorg {
    pub(super) connection: Arc<XCBConnection>,
    pub(super) screen: Screen,
    pub(super) window: Window,
    pub(super) wm_protocols: u32,
    pub(super) wm_delete_window: u32,
    pub(super) display: *mut Display,
    pub(super) width: u16,
    pub(super) height: u16,
    pub(super) opengl_context: *mut __GLXcontextRec,
    viewport: Viewport,
    color_buffer: ColorBuffer,
    debug_lines: DebugLines,
    p: Option<Polyline>,
    p2: Option<Polyline>,
    triangle: Triangle,
    line: Line,
    time: Instant,
    camera: TargetCamera,
    camera_target_marker: PointMarker,
    side_cam: bool,
    cube: Cube,
}

fn get_visual_info(display: *mut Display, screen_num: i32, xid: VisualID) -> XVisualInfo {
    let mut glx_attribs = [
        GLX_RGBA,
        GLX_DOUBLEBUFFER,
        GLX_DEPTH_SIZE,
        24,
        GLX_STENCIL_SIZE,
        8,
        GLX_RED_SIZE,
        8,
        GLX_GREEN_SIZE,
        8,
        GLX_BLUE_SIZE,
        8,
        GLX_SAMPLE_BUFFERS,
        0,
        GLX_SAMPLES,
        0,
        0,
    ];
    unsafe {
        glx::glXChooseVisual(display, screen_num, glx_attribs.as_mut_ptr())
            .as_ref()
            .unwrap()
            .to_owned()
    }
}

impl Xorg {
    pub(super) fn create_window(x: i16, y: i16, width: u16, height: u16) -> Xorg {
        let (conn, screen_num, display) = unsafe {
            let display = XOpenDisplay(std::ptr::null::<i8>());
            let screen_num = XDefaultScreen(display);
            let conn = x11::xlib_xcb::XGetXCBConnection(display);

            XSetEventQueueOwner(display, x11::xlib_xcb::XEventQueueOwner::XCBOwnsEventQueue);
            let conn = x11rb::xcb_ffi::XCBConnection::from_raw_xcb_connection(conn, false).unwrap();
            (conn, screen_num as usize, display)
        };
        let conn1 = std::sync::Arc::new(conn);
        let conn = &conn1;
        let screen = conn.setup().roots[screen_num].clone();
        let mut visual_info =
            get_visual_info(display, screen_num as i32, screen.root_visual as u64);
        check_visual(&screen, screen.root_visual);
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
                    | EventMask::ENTER_WINDOW
                    | EventMask::RESIZE_REDIRECT
                    | EventMask::VISIBILITY_CHANGE,
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

        let opengl_context = unsafe {
            x11::glx::glXCreateContext(
                display,
                &mut visual_info as *mut XVisualInfo,
                ::std::ptr::null_mut(),
                GL_TRUE,
            )
        };
        unsafe { x11::glx::glXMakeCurrent(display, window as u64, opengl_context) };

        gl_loader::init_gl();
        gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);
        unsafe { gl::Enable(GL_DEPTH_TEST) };

        if let Err(err) = conn.flush() {
            log_output(
                LogLevel::Error,
                format!("An error occured when flushing the stream: {}", err),
            );
        }

        let res = Resources::from_relative_exe_path(Path::new("triangle")).unwrap();

        let viewport = viewport::Viewport::for_window(width as i32, height as i32);
        let mut color_buffer = color_buffer::ColorBuffer::new();
        let debug_lines = DebugLines::new(&res).unwrap();
        let cube = Cube::new(&res, &debug_lines).unwrap();
        let mut p = Some(
            debug_lines
                .start_polyline([0.5, -0.5, 0.0].into(), [1.0, 0.0, 0.0, 1.0].into())
                .with_point([0.0, 0.5, 0.0].into(), [0.0, 1.0, 0.0, 1.0].into())
                .with_point([-0.5, -0.5, 0.0].into(), [1.0, 1.0, 0.0, 0.0].into())
                .close_and_finish(),
        );
        let mut p2 = Some(
            debug_lines
                .start_polyline([0.5, 0.0, -0.5].into(), [1.0, 1.0, 0.0, 1.0].into())
                .with_point([0.0, 0.0, 0.5].into(), [1.0, 0.0, 0.0, 1.0].into())
                .with_point([-0.5, 0.0, -0.5].into(), [1.0, 1.0, 0.0, 1.0].into())
                .close_and_finish(),
        );

        let triangle = triangle::Triangle::new(&res).unwrap();
        let line = line::Line::new(&res).unwrap();

        let camera = camera::TargetCamera::new(
            width as f32 / height as f32,
            3.14 / 2.0,
            0.01,
            1000.0,
            3.14 / 4.0,
            3.0,
        );

        let camera_target_marker = debug_lines.marker(camera.target, 0.25);

        color_buffer.set_clear_color(na::Vector3::new(0.3, 0.3, 0.5));

        // unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE) }

        let mut time = Instant::now();
        let mut side_cam = false;

        Xorg {
            connection: conn.clone(),
            screen,
            window,
            wm_protocols,
            wm_delete_window,
            display,
            width,
            height,
            opengl_context,
            viewport,
            color_buffer,
            triangle,
            line,
            debug_lines,
            p,
            p2,
            time,
            camera,
            camera_target_marker,
            side_cam,
            cube,
        }
    }

    pub(super) fn pump_messages(&mut self, channel: ChannelSender, input: &mut InputState) -> bool {
        let mut quit_flagged = false;
        let conn = &self.connection;
        let event = conn.wait_for_event().unwrap();
        match event {
            Event::ResizeRequest(event) => {
                // Fire off an event for immediate processing.
                let mut context = EventContext {
                    data: Data { u16: [0; 8] },
                };
                unsafe {
                    context.data.u16[0] = event.width;
                    context.data.u16[1] = event.height;
                }
                channel.send(Message::Pub {
                    code: SystemEventCode::CODE_RESIZED,
                    sender: None,
                    context,
                    channel: channel.clone(),
                });
                self.viewport
                    .update_size(event.width as i32, event.height as i32);
                self.viewport.set_used();
            }
            Event::KeyPress(event) | Event::KeyRelease(event) => {
                let pressed = event.response_type == KEY_PRESS_EVENT as u8;
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
                input.process_key(channel.clone(), key, pressed)
            }
            Event::ButtonPress(event) | Event::ButtonRelease(event) => {
                let pressed = event.response_type == BUTTON_PRESS_EVENT as u8;
                let mut mouse_button = Buttons::BUTTON_MAX_BUTTONS;
                match event.detail {
                    LEFT_MOUSE_BUTTON => mouse_button = Buttons::BUTTON_LEFT,
                    MIDDLE_MOUSE_BUTTON => mouse_button = Buttons::BUTTON_MIDDLE,
                    RIGHT_MOUSE_BUTTON => mouse_button = Buttons::BUTTON_RIGHT,
                    SCROLL_WHEEL_UP => mouse_button = Buttons::BUTTON_SCROLL_WHEEL_UP,
                    SCROLL_WHEEL_DOWN => mouse_button = Buttons::BUTTON_SCROLL_WHEEL_DOWN,
                    _ => {}
                }
                // Pass over to the input subsystem.
                if mouse_button != Buttons::BUTTON_MAX_BUTTONS {
                    input.process_button(channel.clone(), mouse_button, pressed);
                }
            }
            Event::MotionNotify(event) => {
                // Pass over to the input subsystem.
                input.process_mouse_move(channel.clone(), event.event_x, event.event_y);
            }
            Event::Expose(_event) => {
                // unsafe { x11::glx::glXSwapBuffers(self.display, self.window as u64) };
                /* do drawing here */
                self.viewport.set_used();
                let delta = self.time.elapsed().as_fractional_secs();
                self.time = Instant::now();
                if self.camera.update(delta as f32) {
                    self.camera_target_marker
                        .update_position(self.camera.target);
                }

                let vp_matrix = self.camera.get_vp_matrix();

                unsafe {
                    gl::Enable(gl::CULL_FACE);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    gl::Enable(gl::DEPTH_TEST);
                }

                self.color_buffer.clear();
                // self.cube.render(
                //     &self.camera.get_view_matrix(),
                //     &self.camera.get_p_matrix(),
                //     &self.camera.project_pos().coords,
                // );
                self.debug_lines.render(&self.color_buffer, &vp_matrix);
                self.line.render(&self.color_buffer, &vp_matrix);

                unsafe { x11::glx::glXSwapBuffers(self.display, self.window as u64) };
            }
            Event::ConfigureNotify(event) => {
                // Fire off an event for immediate processing.
                let mut context = EventContext {
                    data: Data { u16: [0; 8] },
                };
                unsafe {
                    context.data.u16[0] = event.width;
                    context.data.u16[1] = event.height;
                }
                self.viewport.w = event.width as i32;
                self.viewport.h = event.height as i32;
                channel.send(Message::Pub {
                    code: SystemEventCode::CODE_RESIZED,
                    sender: None,
                    context,
                    channel: channel.clone(),
                });
                // self.viewport.set_used();
                // unsafe { x11::glx::glXSwapBuffers(self.display, self.window as u64) };
            }
            Event::ClientMessage(event) => {
                let data = event.data.as_data32();
                if event.format == 32
                    && event.window == self.window
                    && data[0] == self.wm_delete_window
                {
                    log_output(LogLevel::Info, "Window was asked to close");
                    unsafe {
                        x11::glx::glXMakeCurrent(
                            self.display,
                            0, /* None ? */
                            ::std::ptr::null_mut(),
                        );
                    };
                    unsafe { x11::glx::glXDestroyContext(self.display, self.opengl_context) };
                    unsafe { x11::xlib::XDestroyWindow(self.display, self.window as u64) };
                    unsafe { x11::xlib::XCloseDisplay(self.display) };
                    quit_flagged = true;
                }
            }
            Event::Error(error) => log_output(
                LogLevel::Error,
                format!("Got an unexpected error: {:?}", error),
            ),
            _ => log_output(LogLevel::Warn, "Got an unknown event"),
        }

        !quit_flagged
    }
}

/// Check that the given visual is "as expected" (pixel values are 0xRRGGBB with RR/GG/BB being the
/// colors). Otherwise, this exits the process.
fn check_visual(screen: &Screen, id: Visualid) {
    // Find the information about the visual and at the same time check its depth.
    let visual_info = screen
        .allowed_depths
        .iter()
        .filter_map(|depth| {
            let info = depth.visuals.iter().find(|depth| depth.visual_id == id);
            info.map(|info| (depth.depth, info))
        })
        .next();
    let (depth, visual_type) = match visual_info {
        Some(info) => info,
        None => {
            log_output(
                LogLevel::Fatal,
                "Did not find the root visual's description?!",
            );
            std::process::exit(1);
        }
    };
    // Check that the pixels have red/green/blue components that we can set directly.
    match visual_type.class {
        VisualClass::TRUE_COLOR | VisualClass::DIRECT_COLOR => {}
        _ => {
            log_output(
                LogLevel::Fatal,
                format!(
                    "The root visual is not true / direct color, but {:?}",
                    visual_type,
                ),
            );
            std::process::exit(1);
        }
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
