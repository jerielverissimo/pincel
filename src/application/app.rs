use super::*;
use crate::commands::{
    draw_command::*, exit_command::*, left_click_command::*, left_release_command::*,
    motion_command::*, right_click_command::*, Command,
};
use crate::domain::error::PincelError;
use crate::domain::*;
use app_initializer::*;
use entities::{color::CurrentColorSingleton, movement::Movement};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::CURRENT_TIME;

pub struct Application<C> {
    pub is_running: bool,
    pub skip_frame: bool,
    pub stack: Vec<Option<Movement>>,
    pub brush_color: CurrentColorSingleton,
    pub conn: C,
    pub win_id: u32,
    pub gc_id: u32,
    pub current: usize,
    pub atoms: AtomCollection,
}

impl<C: Connection + Send + Sync> Application<C> {
    pub fn run(&mut self) -> Result<(), PincelError> {
        while self.is_running {
            self.reset_frame();
            let event = self.conn.wait_for_event()?;
            self.dispatch(event)?;
            if self.skip_frame {
                continue;
            };
        }
        Ok(())
    }
    pub fn reset_frame(&mut self) {
        self.skip_frame = false;
    }

    pub fn skip(&mut self) {
        self.skip_frame = true;
    }

    pub fn dispatch(&mut self, event: Event) -> Result<(), PincelError> {
        match event {
            Event::KeyPress(e) => {
                ExitCommand::new(self, e).execute()?;
            }
            Event::KeyRelease(_) => {}
            Event::Expose(e) => {
                DrawCommand::new(self, e).execute()?;
            }
            Event::ButtonPress(event) => {
                LeftClickCommand::new(self, event).execute()?;
                RightClickCommand::new(self, event).execute()?;
            }
            Event::ButtonRelease(event) => {
                if self.stack.is_empty() {
                    self.skip();
                }
                LeftReleaseCommand::new(self, event).execute()?;
            }
            Event::MotionNotify(event) => {
                if self.stack.is_empty() {
                    self.skip();
                }
                MotionCommand::new(self, event).execute()?;
            }
            Event::EnterNotify(_) => {
                self.conn
                    .set_input_focus(InputFocus::PointerRoot, self.win_id, CURRENT_TIME)?;
            }
            Event::ClientMessage(event) => {
                let data = event.data.as_data32();
                if event.format == 32
                    && event.window == self.win_id
                    && data[0] == self.atoms.WM_DELETE_WINDOW
                {
                    println!("Window was asked to close");
                    return Ok(());
                }
            }
            Event::Error(e) => return Err(PincelError::XlibError(e)),
            _ => println!("Got an unknown event"),
        }
        Ok(())
    }
}
