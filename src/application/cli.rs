use crate::gui::window::init_gtk_window;
use std::thread;

pub struct Cli {
    pattern: Option<String>,
}

impl Cli {
    pub fn new() -> Self {
        let pattern = std::env::args().nth(1);
        Self { pattern }
    }

    pub fn toggle_gui(&self) {
        if let Some(opt) = &self.pattern {
            if opt == "-g" {
                thread::spawn(move || {
                    init_gtk_window();
                });
            } else {
                println!("Unknown option");
            }
        }
    }
}
