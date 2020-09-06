use super::*;

use event_handler::EventHandler;
use x11rb::{protocol::{Event, xproto::*}, connection::Connection};
use app::Application;

pub trait Command {
    fn execute(&mut self) -> Result<(), PincelError>;
}

pub mod draw_command;
pub mod exit_command;
pub mod left_click_command;
pub mod left_release_command;
pub mod motion_command;
pub mod right_click_command;
