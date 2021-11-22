use std::{any::Any, rc::Rc};

use num::FromPrimitive;

use super::{
    clock::Clock,
    event::{ChannelSender, Data, EventContext, EventSystem, Listener, Message, SystemEventCode},
    input::{InputState, Keys},
    platform::{Backend, Platform, PlatformState},
};

// Application configuration.
pub struct ApplicationConfig {
    // Window starting position x axis, if applicable
    pub start_pos_x: i16,

    // Window starting position y axis, if applicable
    pub start_pos_y: i16,

    // Window starting width, if applicable
    pub start_width: u16,

    // Window starting height, if applicable
    pub start_height: u16,
}

static mut APP_STATE: Option<ApplicationState> = None;

pub struct ApplicationState {
    is_running: bool,
    is_suspended: bool,
    platform: PlatformState,
    width: u16,
    height: u16,
    clock: Clock,
    last_time: f64,
    input: InputState,
    event_system: EventSystem,
}

impl ApplicationState {
    pub fn create(config: ApplicationConfig) {
        let mut platform = PlatformState::new(Platform::Linux {
            backend: Backend::Xorg,
        });

        let input = InputState::new();

        let event_system = EventSystem::new();
        let sender = event_system.channel.sender();
        sender.send(Message::Sub {
            code: SystemEventCode::CODE_APPLICATION_QUIT,
            listener: None,
            on_event: application_on_event,
        });
        sender.send(Message::Sub {
            code: SystemEventCode::CODE_KEY_PRESSED,
            listener: None,
            on_event: application_on_key,
        });
        sender.send(Message::Sub {
            code: SystemEventCode::CODE_KEY_RELEASED,
            listener: None,
            on_event: application_on_key,
        });

        if platform.startup(
            config.start_pos_x,
            config.start_pos_y,
            config.start_width,
            config.start_height,
        ) {}

        unsafe {
            APP_STATE = Some(Self {
                is_running: true,
                is_suspended: false,
                platform,
                width: config.start_width,
                height: config.start_height,
                last_time: 0.0,
                input,
                event_system,
                clock: Clock {
                    start_time: 0.0,
                    elapsed: 0.0,
                },
            });
        }
    }

    pub fn run() {
        unsafe {
            if let Some(ref mut state) = APP_STATE {
                state.clock.start();
                state.clock.update();
                let running_time = 0;
                let frame_count = 0;
                let target_frame_seconds = 1.0 / 60.0;

                while state.is_running {
                    let channel = &state.event_system.channel;
                    if !state.platform.pump_messages(state.input, channel.sender()) {
                        state.is_running = false;
                    }

                    if !state.is_suspended {
                        // Update clock and get delta time.
                        state.clock.update();
                        let current_time = state.clock.elapsed;
                        let delta = current_time - state.last_time;
                        let frame_start_time = Clock::get_absolute_time();
                        state.event_system.listen_messages();
                    }
                }
            }
        }
    }
}

fn application_on_event(
    code: SystemEventCode,
    sender: Option<Rc<Box<dyn Any>>>,
    listener_inst: Option<Rc<Box<dyn Listener>>>,
    data: EventContext,
    channel: ChannelSender,
) -> bool {
    match code {
        SystemEventCode::CODE_APPLICATION_QUIT => {
            println!("EVENT_CODE_APPLICATION_QUIT recieved, shutting down.");
            unsafe {
                APP_STATE.as_mut().unwrap().is_running = false;
            }
            return true;
        }
        _ => {}
    }
    false
}

fn application_on_key(
    code: SystemEventCode,
    sender: Option<Rc<Box<dyn Any>>>,
    listener_inst: Option<Rc<Box<dyn Listener>>>,
    context: EventContext,
    channel: ChannelSender,
) -> bool {
    match code {
        SystemEventCode::CODE_KEY_PRESSED => {
            let key_code = unsafe { context.data.u16[0] };
            let key_code: Keys = FromPrimitive::from_u16(key_code).unwrap();

            if key_code == Keys::ESCAPE {
                // NOTE: Technically firing an event to itself, but there may be other listeners.
                let data = EventContext {
                    data: Data { u16: [0; 8] },
                };
                channel.send(Message::Pub {
                    code: SystemEventCode::CODE_APPLICATION_QUIT,
                    sender: None,
                    context: data,
                    channel: channel.clone(),
                });
            }
            return true;
        }
        _ => {}
    }
    false
}
