#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(clippy::missing_docs_in_private_items)]
#![feature(try_trait)]

mod application;
mod commands;
mod domain;
mod gui;

use application::*;
use domain::*;

fn main() -> Result {
    let mut app = app_initializer::init()?;
    Ok(app.run()?)
}
