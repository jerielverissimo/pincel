use std::any::Any;

mod xorg;
use xorg::Xorg;

use super::{event::ChannelSender, input::InputState};

pub enum Backend {
    Xorg,
    Wayland,
}

pub enum Platform {
    Linux { backend: Backend },
}

pub struct PlatformState {
    platform: Platform,
    internal_state: Option<Box<dyn Any>>,
}

impl PlatformState {
    pub fn new(platform: Platform) -> Self {
        Self {
            platform,
            internal_state: None,
        }
    }

    fn state(&mut self) -> &mut Option<Box<dyn Any>> {
        &mut self.internal_state
    }

    pub fn startup(&mut self, x: i16, y: i16, width: u16, height: u16) -> bool {
        match self.platform {
            Platform::Linux { ref backend } => match backend {
                Backend::Wayland => {}
                Backend::Xorg => {
                    self.internal_state = Some(Box::new(Xorg::create_window(x, y, width, height)));
                    return true;
                }
            },
        }
        false
    }

    pub fn pump_messages(&mut self, input: InputState, channel: ChannelSender) -> bool {
        match self.platform {
            Platform::Linux { ref backend } => match backend {
                Backend::Wayland => true,
                Backend::Xorg => {
                    let xorg: &mut Xorg = self.state().as_mut().unwrap().downcast_mut().unwrap();
                    xorg.pump_messages(channel, input)
                }
            },
        }
    }
}
