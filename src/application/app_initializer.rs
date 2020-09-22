use super::{app::Application, cli::Cli, config::Config};
use crate::{domain::*, gui::window_builder::WindowBuilder};

use x11rb::wrapper::ConnectionExt as _;

use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::{atom_manager, CURRENT_TIME};

// A collection of the atoms we will need.
atom_manager! {
    pub AtomCollection: AtomCollectionCookie {
        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
        _NET_WM_STATE,
        _NET_WM_STATE_STAYS_ON_TOP,
        _NET_WM_WINDOW_TYPE_SPLASH,
        _NET_WM_WINDOW_TYPE,
    }
}

pub fn init() -> Result<Application<impl Connection + Send + Sync>, PincelError> {
    let (conn, screen_num) = x11rb::connect(None)?;

    let screen = &conn.setup().roots[screen_num];
    let win_id = conn.generate_id()?;
    let gc_id = conn.generate_id()?;
    let atoms = AtomCollection::new(&conn)?.reply()?;

    // fetch all depths
    let depths = &screen.allowed_depths;

    // fetch a visual supporting alpha channels
    let visual = GraphicContext::fetch_visual_alpha(depths)?;

    let win_start_x = 0;
    let win_start_y = 0;

    let colormap = GraphicContext::genarate_colormap(&conn, screen, &visual)?;

    let (width, height) = (
        (screen.width_in_pixels as f32) as u16,
        (screen.height_in_pixels as f32) as u16,
    );

    WindowBuilder::new()
        .with_win_id(win_id)
        .with_screen(screen)
        .with_pos((&win_start_x, &win_start_y))
        .with_size((&width, &height))
        .with_visual(&visual)
        .with_win_params(colormap)
        .build(&conn)?;

    let brush_color = CurrentColorSingleton::new();

    let gc_aux = GraphicContext::create_gc(brush_color.value());

    free_colormap(&conn, colormap)?;

    conn.change_property32(
        PropMode::Replace,
        win_id,
        atoms._NET_WM_WINDOW_TYPE,
        AtomEnum::ATOM,
        &[atoms._NET_WM_WINDOW_TYPE_SPLASH],
    )?;

    conn.change_property32(
        PropMode::Append,
        win_id,
        atoms._NET_WM_STATE,
        AtomEnum::ATOM,
        &[atoms._NET_WM_STATE_STAYS_ON_TOP],
    )?;

    conn.change_property32(
        PropMode::Replace,
        win_id,
        AtomEnum::WM_CLASS,
        AtomEnum::ATOM,
        &[atoms.WM_DELETE_WINDOW],
    )?;

    conn.create_gc(gc_id, win_id, &gc_aux)?;

    conn.map_window(win_id)?;
    conn.set_input_focus(InputFocus::PointerRoot, win_id, CURRENT_TIME)?;

    conn.flush()?;

    let stack: Vec<Option<Movement>> = Vec::new();
    let current = stack.len();
    println!("Press q to exit.");

    let app = Application {
        is_running: true,
        stack,
        win_id,
        gc_id,
        brush_color,
        conn,
        screen_num,
        skip_frame: false,
        current,
        atoms,
        cli: Cli::new(),
        config: Config::new(),
    };
    Ok(app)
}
