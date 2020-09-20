use crate::domain::error::PincelError;
use x11rb::{connection::Connection, protocol::xproto::*};

pub struct GraphicContext;

impl GraphicContext {
    pub fn create_gc(color: u32) -> CreateGCAux {
        CreateGCAux::new()
            .foreground(color)
            .graphics_exposures(1)
            .line_width(1)
    }
    pub fn change_color(color: u32) -> ChangeGCAux {
        ChangeGCAux::new()
            .foreground(color)
            .graphics_exposures(1)
            .line_width(1)
    }
    pub fn fetch_visual_alpha(depths: &[Depth]) -> Result<Visualtype, PincelError> {
        let mut alpha_depths = depths
            .iter()
            .filter(|d: &&Depth| d.depth == 32u8)
            .peekable();
        if alpha_depths.peek().is_none() {
            panic!("Alpha channel not found!");
        }
        Ok(*alpha_depths.next()?.visuals.get(1)?)
    }

    pub fn genarate_colormap(
        conn: &(impl Connection + Send + Sync),
        screen: &Screen,
        visual: &Visualtype,
    ) -> Result<u32, PincelError> {
        let colormap = conn.generate_id()?;
        create_colormap(
            conn,
            ColormapAlloc::None,
            colormap,
            screen.root,
            visual.visual_id,
        )?;
        Ok(colormap)
    }
}
