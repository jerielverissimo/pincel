use crate::domain::error::PincelError;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;

const BLACK: u32 = 0x00;
const TRUE: u32 = 1;

pub struct WindowBuilder<'wp> {
    win_id: u32,
    screen: Option<&'wp Screen>,
    pos: (&'wp i16, &'wp i16),
    size: (&'wp u16, &'wp u16),
    visual: Option<&'wp Visualtype>,
    win_params: Option<CreateWindowAux>,
}

impl<'wp> WindowBuilder<'wp> {
    pub fn new() -> Self {
        WindowBuilder {
            win_id: 0,
            screen: None,
            pos: (&0, &0),
            size: (&0, &0),
            visual: None,
            win_params: None,
        }
    }
    pub fn build(&self, conn: &(impl Connection + Send + Sync)) -> Result<(), PincelError> {
        let (win_start_x, win_start_y) = self.pos;
        let (width, height) = self.size;
        conn.create_window(
            32,
            self.win_id,
            self.screen?.root,
            *win_start_x,
            *win_start_y,
            *width,
            *height,
            0,
            WindowClass::InputOutput,
            self.visual?.visual_id,
            &self.win_params?,
        )?;
        Ok(())
    }
    pub fn with_win_id(&mut self, id: u32) -> &mut Self {
        self.win_id = id;
        self
    }
    pub fn with_screen(&mut self, screen: &'wp Screen) -> &mut Self {
        self.screen = Some(screen);
        self
    }
    pub fn with_pos(&mut self, pos: (&'wp i16, &'wp i16)) -> &mut Self {
        self.pos = pos;
        self
    }
    pub fn with_size(&mut self, size: (&'wp u16, &'wp u16)) -> &mut Self {
        self.size = size;
        self
    }
    pub fn with_visual(&mut self, visual: &'wp Visualtype) -> &mut Self {
        self.visual = Some(visual);
        self
    }
    pub fn with_win_params(&mut self, colormap: u32) -> &mut Self {
        let win_params = CreateWindowAux::new()
            .event_mask(
                EventMask::Exposure
                    | EventMask::StructureNotify
                    | EventMask::ButtonPress
                    | EventMask::ButtonMotion
                    | EventMask::ButtonRelease
                    | EventMask::Button1Motion
                    | EventMask::EnterWindow
                    | EventMask::KeyPress,
            )
            .backing_pixel(BLACK)
            .border_pixel(BLACK)
            .background_pixel(BLACK)
            .override_redirect(TRUE)
            .colormap(colormap);
        self.win_params = Some(win_params);
        self
    }
}
