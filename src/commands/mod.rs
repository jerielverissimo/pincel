use super::{app, event_handler, Result};

use app::Application;
use event_handler::EventHandler;
use x11rb::{
    connection::Connection,
    protocol::{
        xproto::{
            ButtonPressEvent, ButtonReleaseEvent, ExposeEvent, KeyPressEvent, MotionNotifyEvent,
        },
        Event,
    },
};

pub trait Command {
    fn execute(&mut self) -> Result;
}

pub mod draw_command;
pub mod key_press_command;
pub mod left_click_command;
pub mod left_release_command;
pub mod middle_click_command;
pub mod motion_command;
pub mod right_click_command;

pub use draw_command::*;
pub use key_press_command::*;
pub use left_click_command::*;
pub use left_release_command::*;
pub use middle_click_command::*;
pub use motion_command::*;
pub use right_click_command::*;
