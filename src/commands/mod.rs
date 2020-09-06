use super::*;

use app::Application;
use event_handler::EventHandler;
use x11rb::{
    connection::Connection,
    protocol::{xproto::*, Event},
};

pub trait Command {
    fn execute(&mut self) -> Result<(), PincelError>;
}

pub mod draw_command;
pub mod exit_command;
pub mod left_click_command;
pub mod left_release_command;
pub mod motion_command;
pub mod right_click_command;

pub use draw_command::*;
pub use exit_command::*;
pub use left_click_command::*;
pub use left_release_command::*;
pub use motion_command::*;
pub use right_click_command::*;
