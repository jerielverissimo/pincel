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
