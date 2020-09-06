#![feature(try_trait)]

mod app_initializer;
mod application;
mod color;
mod commands;
mod error;
mod event_handler;
mod graphics_context;
mod movement;
mod window;
mod window_builder;

use application::*;
use color::*;
use error::*;
use event_handler::*;
use graphics_context::*;
use movement::*;

use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;

fn main() -> Result<(), PincelError> {
    let mut app = app_initializer::init()?;
    Ok(app.run()?)
}
