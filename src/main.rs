#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(clippy::missing_docs_in_private_items)]
#![feature(try_trait)]

mod application;
mod commands;
mod domain;
mod gui;

use application::{app, app_initializer};
use domain::{event_handler, Result};

fn main() -> Result {
    let mut app = app_initializer::init()?;
    Ok(app.run()?)
}
