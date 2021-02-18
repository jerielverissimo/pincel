use super::{app_initializer, cli, Config};
use crate::commands::{
    Command, DrawCommand, KeyPressCommand, LeftClickCommand, LeftReleaseCommand,
    MiddleClickCommand, MotionCommand, RightClickCommand,
};
use crate::domain::error::PincelError;
use crate::domain::{entities, Result};
use app_initializer::AtomCollection;
use cli::Cli;
use entities::{color::CurrentColorSingleton, movement::Movement};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{
    ButtonPressEvent, ButtonReleaseEvent, ClientMessageEvent, ConnectionExt, EnterNotifyEvent,
    ExposeEvent, InputFocus, KeyPressEvent, MotionNotifyEvent,
};
use x11rb::protocol::Event;
use x11rb::CURRENT_TIME;

pub struct Application<C> {
    pub is_running: bool,
    pub skip_frame: bool,
    pub stack: Vec<Option<Movement>>,
    pub brush_color: CurrentColorSingleton,
    pub conn: C,
    pub screen_num: usize,
    pub win_id: u32,
    pub gc_id: u32,
    pub current: usize,
    pub atoms: AtomCollection,
    pub cli: Cli,
    pub config: Config,
}

impl<C: Connection + Send + Sync> Application<C> {
    pub fn run(&mut self) -> Result {
        while self.is_running {
            self.reset_frame();
            let event = self.conn.wait_for_event()?;
            // FIXME: handle input focus lost
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

    pub fn dispatch(&mut self, event: Event) -> Result {
        match event {
            Event::KeyPress(e) => self.handle_key_press(e)?,
            Event::Expose(e) => self.handle_expose(e)?,
            Event::ButtonPress(e) => self.handle_button_press(e)?,
            Event::ButtonRelease(e) => self.handle_button_release(e)?,
            Event::MotionNotify(e) => self.handle_motion_notify(e)?,
            Event::EnterNotify(e) => self.handle_enter_notify(e)?,
            Event::ClientMessage(e) => self.handle_client_message(e)?,
            Event::Error(e) => return Err(PincelError::XlibError(e)),
            _ => println!("Got an unknown event"),
        }
        Ok(())
    }

    fn handle_key_press(&mut self, e: KeyPressEvent) -> Result {
        KeyPressCommand::new(self, e).execute()?;
        Ok(())
    }

    fn handle_expose(&mut self, e: ExposeEvent) -> Result {
        DrawCommand::new(self, e).execute()?;
        Ok(())
    }

    fn handle_button_press(&mut self, e: ButtonPressEvent) -> Result {
        self.conn
            .set_input_focus(InputFocus::Parent, self.win_id, CURRENT_TIME)?;
        LeftClickCommand::new(self, e).execute()?;
        MiddleClickCommand::new(self, e).execute()?;
        RightClickCommand::new(self, e).execute()?;
        Ok(())
    }

    fn handle_button_release(&mut self, e: ButtonReleaseEvent) -> Result {
        if self.stack.is_empty() {
            self.skip();
        }
        LeftReleaseCommand::new(self, e).execute()?;
        Ok(())
    }

    fn handle_motion_notify(&mut self, e: MotionNotifyEvent) -> Result {
        if self.stack.is_empty() {
            self.skip();
        }
        MotionCommand::new(self, e).execute()?;
        Ok(())
    }

    fn handle_enter_notify(&mut self, _: EnterNotifyEvent) -> Result {
        self.conn
            .set_input_focus(InputFocus::Parent, self.win_id, CURRENT_TIME)?;
        Ok(())
    }

    fn handle_client_message(&mut self, event: ClientMessageEvent) -> Result {
        let data = event.data.as_data32();
        if event.format == 32
            && event.window == self.win_id
            && data[0] == self.atoms.WM_DELETE_WINDOW
        {
            println!("Window was asked to close");
            return Ok(());
        }
        Ok(())
    }
}
