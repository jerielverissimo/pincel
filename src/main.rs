mod color;
mod movement;
mod window;

use color::*;
use movement::*;
use window::*;

use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::wrapper::ConnectionExt as _;
use x11rb::CURRENT_TIME;

use std::thread;

fn main() {
    let (conn, screen_num) = x11rb::connect(None).unwrap();

    let screen = &conn.setup().roots[screen_num];
    let win_id = conn.generate_id().unwrap();
    let gc_id = conn.generate_id().unwrap();

    let wm_protocols = conn.intern_atom(false, b"WM_PROTOCOLS").unwrap();
    let wm_delete_window = conn.intern_atom(false, b"WM_DELETE_WINDOW").unwrap();
    let wm_protocols = wm_protocols.reply().unwrap().atom;
    let wm_delete_window = wm_delete_window.reply().unwrap().atom;

    // fetch all depths
    let depths = &screen.allowed_depths;
    let mut alpha_depths = depths.iter().filter(|d| d.depth == 32u8).peekable();
    if alpha_depths.peek().is_none() {
        panic!("Alpha channel not found!");
    }

    // fetch a visual supporting alpha channels
    let visual = alpha_depths.next().unwrap().visuals.get(1).unwrap();

    let win_start_x = 0;
    let win_start_y = 0;

    let colormap = conn.generate_id().unwrap();
    create_colormap(
        &conn,
        ColormapAlloc::None,
        colormap,
        screen.root,
        visual.visual_id,
    )
    .unwrap();

    let win_aux = CreateWindowAux::new()
        .event_mask(
            EventMask::Exposure
                | EventMask::StructureNotify
                | EventMask::ButtonPress
                | EventMask::ButtonMotion
                | EventMask::ButtonRelease
                | EventMask::Button1Motion
                | EventMask::KeyPress
                | EventMask::KeyRelease,
        )
        .backing_pixel(0x00)
        .border_pixel(0x00)
        .background_pixel(0x00)
        .override_redirect(1u32)
        .colormap(colormap);

    let (width, height) = (
        (screen.width_in_pixels as f32 / 1.5) as u16,
        (screen.height_in_pixels as f32 / 1.5) as u16,
    );

    conn.create_window(
        32,
        win_id,
        screen.root,
        win_start_x,
        win_start_y,
        width,
        height,
        0,
        WindowClass::InputOutput,
        visual.visual_id,
        &win_aux,
    )
    .unwrap();

    let brush_color = CurrentColorSingleton::new();

    let gc_aux = CreateGCAux::new()
        .foreground(brush_color.value())
        .graphics_exposures(1)
        .line_width(1);

    free_colormap(&conn, colormap).unwrap();

    let win_type_atom = intern_atom(&conn, true, b"_NET_WM_WINDOW_TYPE")
        .unwrap()
        .reply()
        .unwrap()
        .atom;

    let win_menu_atom = intern_atom(&conn, true, b"_NET_WM_WINDOW_TYPE_SPLASH")
        .unwrap()
        .reply()
        .unwrap()
        .atom;

    let win_state_atom = intern_atom(&conn, true, b"_NET_WM_STATE")
        .unwrap()
        .reply()
        .unwrap()
        .atom;

    let win_on_top_atom = intern_atom(&conn, true, b"_NET_WM_STATE_STAYS_ON_TOP")
        .unwrap()
        .reply()
        .unwrap()
        .atom;

    conn.change_property32(
        PropMode::Replace,
        win_id,
        win_type_atom,
        AtomEnum::ATOM,
        &[win_menu_atom],
    )
    .unwrap();

    conn.change_property32(
        PropMode::Append,
        win_id,
        win_state_atom,
        AtomEnum::ATOM,
        &[win_on_top_atom],
    )
    .unwrap();

    conn.change_property32(
        PropMode::Replace,
        win_id,
        wm_protocols,
        AtomEnum::ATOM,
        &[wm_delete_window],
    )
    .unwrap();

    conn.create_gc(gc_id, win_id, &gc_aux).unwrap();

    thread::spawn(move || {
        init_gtk_window();
    });

    conn.map_window(win_id).unwrap();
    conn.set_input_focus(InputFocus::PointerRoot, win_id, CURRENT_TIME)
        .unwrap();

    conn.flush().unwrap();

    let mut stack: Vec<Option<Movement>> = Vec::new();
    let mut current = stack.len();

    let mut id = 0;

    loop {
        let event = conn.wait_for_event().unwrap();
        match event {
            Event::KeyPress(e) => {
                println!("Key {} pressed", e.detail);
                if e.detail == 24 || e.detail == 9 {
                    return;
                }
            }
            Event::KeyRelease(_) => {}
            Event::Expose(e) => {
                for mov in &stack {
                    if let Some(mov) = &mov {
                        let new_gc = ChangeGCAux::new()
                            .foreground(mov.color.value())
                            .graphics_exposures(1)
                            .line_width(1);

                        conn.change_gc(gc_id, &new_gc).unwrap();

                        conn.flush().unwrap();
                        mov.expose(&conn, win_id, gc_id, &e);
                    }
                }
                conn.flush().unwrap();
            }
            Event::ButtonPress(event) => {
                if event.detail == 1 {
                    let new_gc = ChangeGCAux::new()
                        .foreground(brush_color.value())
                        .graphics_exposures(1)
                        .line_width(1);

                    conn.change_gc(gc_id, &new_gc).unwrap();

                    conn.flush().unwrap();

                    let temp = Some(Movement::new(event, brush_color.clone().into()));
                    stack.push(temp);
                    current = stack.len() - 1;

                    continue;
                }

                // right button
                if event.detail == 3 {
                    if stack.is_empty() {
                        continue;
                    }
                    stack.pop();
                    current = if !stack.is_empty() {
                        stack.len() - 1
                    } else {
                        0
                    };
                    conn.clear_area(true, win_id, 0, 0, 0, 0).unwrap();
                    conn.flush().unwrap();
                    continue;
                }
            }
            Event::ButtonRelease(event) => {
                if stack.is_empty() {
                    continue;
                }
                if event.detail == 1 {
                    stack[current]
                        .as_mut()
                        .unwrap()
                        .finish(&conn, win_id, gc_id, &event)
                        .unwrap();
                }
            }
            Event::MotionNotify(event) => {
                if stack.is_empty() {
                    continue;
                }

                if let Some(current) = &mut stack[current] {
                    current
                        .motion(
                            &conn,
                            win_id,
                            gc_id,
                            (event.event_x, event.event_y, event.time),
                        )
                        .unwrap();
                }
            }
            Event::EnterNotify(_) => {
                conn.set_input_focus(InputFocus::PointerRoot, win_id, CURRENT_TIME)
                    .unwrap();
            }
            Event::ClientMessage(event) => {
                let data = event.data.as_data32();
                if event.format == 32 && event.window == win_id && data[0] == wm_delete_window {
                    println!("Window was asked to close");
                    return;
                }
            }
            Event::Error(_) => println!("Got an unexpected error"),
            _ => println!("Got an unknown event"),
        }
    }
}
