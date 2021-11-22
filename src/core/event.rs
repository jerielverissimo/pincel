use std::{
    any::Any,
    rc::Rc,
    sync::mpsc::{channel, Receiver, Sender},
};

const MAX_MESSAGE_CODES: usize = 16384;

pub enum Message {
    Pub {
        code: SystemEventCode,
        sender: Option<Rc<Box<dyn Any>>>,
        context: EventContext,
        channel: ChannelSender,
    },
    Sub {
        code: SystemEventCode,
        listener: Option<Rc<Box<dyn Listener>>>,
        on_event: PfnOnEvent,
    },
    Shutdown,
}

#[derive(Clone)]
pub struct ChannelSender {
    pub tx: Sender<Message>,
}

impl ChannelSender {
    pub fn send(&self, msg: Message) {
        match self.tx.send(msg) {
            Ok(_) => println!(""),
            Err(e) => println!("Failed to send message {}", e),
        };
    }
}

pub struct Channel {
    pub tx: Sender<Message>,
    pub rx: Receiver<Message>,
}

impl Channel {
    pub(crate) fn new() -> Self {
        let (tx, rx) = channel();
        Channel { tx, rx }
    }

    pub fn sender(&self) -> ChannelSender {
        ChannelSender {
            tx: self.tx.clone(),
        }
    }
}

pub trait Listener: Any {
    fn as_any(&self) -> &dyn Any;
    fn dyn_clone(&self) -> Box<dyn Listener>;
    fn dyn_eq(&self, other: &Box<dyn Listener>) -> bool;
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub union Data {
    // 128 bytes
    pub i64: [i64; 2],
    pub u64: [u64; 2],
    pub f64: [f64; 2],

    pub i32: [i32; 4],
    pub u32: [u32; 4],
    pub f32: [f32; 4],

    pub i16: [i16; 8],
    pub u16: [u16; 8],

    pub i8: [i8; 16],
    pub u8: [u8; 16],
}

#[derive(Clone, Copy)]
pub struct EventContext {
    // 128 bytes
    pub data: Data,
}

pub type PfnOnEvent = fn(
    code: SystemEventCode,
    sender: Option<Rc<Box<dyn Any>>>,
    listener_inst: Option<Rc<Box<dyn Listener>>>,
    data: EventContext,
    channel: ChannelSender,
) -> bool;

#[derive(Clone)]
pub struct RegisteredEvent {
    listener: Option<Rc<Box<dyn Listener>>>,
    callback: Option<PfnOnEvent>,
}

#[derive(Clone, Default)]
pub struct EventCodeEntry {
    events: Option<Vec<RegisteredEvent>>,
}

pub struct EventSystem {
    pub channel: Channel,
    registered: Vec<EventCodeEntry>,
}

impl EventSystem {
    pub fn new() -> Self {
        let channel = Channel::new();

        let mut system = Self {
            channel,
            registered: vec![Default::default(); MAX_MESSAGE_CODES],
        };

        system.registered.fill_with(Default::default);
        system
    }

    pub fn listen_messages(&mut self) {
        match self.channel.rx.try_recv() {
            Ok(message) => match message {
                Message::Shutdown => todo!(),
                Message::Pub {
                    code,
                    sender,
                    context,
                    channel,
                } => {
                    println!("PUB: Received message code: {:?} ", code);
                    self.fire(code, sender, context, channel);
                }
                Message::Sub {
                    code,
                    listener,
                    on_event,
                } => {
                    println!("SUB: Received message code: {:?} ", code);
                    self.register(code, listener, on_event);
                }
            },
            Err(e) => println!("Error on recieved message {}", e),
        }
    }

    pub fn shutdown(&mut self) {
        for entry in &mut self.registered {
            match entry.events {
                Some(_) => entry.events = None,
                None => {}
            }
        }
    }

    pub fn register(
        &mut self,
        code: SystemEventCode,
        listener: Option<Rc<Box<dyn Listener>>>,
        on_event: PfnOnEvent,
    ) -> bool {
        let code = code as usize;
        if let None = self.registered[code].events {
            self.registered[code].events = Some(vec![]);
        }

        if let Some(ref mut events) = self.registered[code].events {
            let registered_count = events.len();

            if let Some(ref listener) = listener {
                for i in 0..registered_count {
                    if let Some(ref l) = events[i].listener {
                        if l.dyn_eq(&listener) {
                            return false;
                        }
                    }
                }
            }
            // If at this point, no duplicate was found. Proceed with registration.
            let event = RegisteredEvent {
                listener,
                callback: Some(on_event),
            };
            events.push(event);
            return true;
        }

        false
    }

    pub fn unregister(
        &mut self,
        code: usize,
        listener: Option<Box<dyn Listener>>,
        on_event: PfnOnEvent,
    ) -> bool {
        // On nothing is registered for the code, boot out.
        if let None = self.registered[code].events {
            return false;
        }

        if let Some(ref mut events) = self.registered[code].events {
            let registered_count = events.len();

            if let Some(listener) = listener {
                for i in 0..registered_count {
                    let e = &events[i];
                    if let Some(ref l) = e.listener {
                        if l.dyn_eq(&listener) && e.callback == Some(on_event) {
                            events.remove(i);
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    pub fn fire(
        &mut self,
        code: SystemEventCode,
        sender: Option<Rc<Box<dyn Any>>>,
        context: EventContext,
        channel: ChannelSender,
    ) -> bool {
        // If nothing is registered for the code, boot out.
        if let None = self.registered[code as usize].events {
            return false;
        }

        if let Some(ref mut events) = self.registered[code as usize].events {
            let registered_count = events.len();

            for i in 0..registered_count {
                let e = &events[i];
                if let Some(callback) = e.callback {
                    if (callback)(
                        code,
                        sender.clone(),
                        e.listener.clone(),
                        context,
                        channel.clone(),
                    ) {
                        // Message has been handled, do not send to other listeners;
                        return true;
                    }
                }
            }
        }
        // Not found.
        false
    }
}

// System internal event codes. Application should use codes beyond 255.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub enum SystemEventCode {
    // Shuts the application down on the next frame.
    CODE_APPLICATION_QUIT = 0x01,

    // Keyboard key pressed.
    /* Context usage:
     * u16 key_code = data.data.u16[0],
     */
    CODE_KEY_PRESSED = 0x02,

    // Keyboard key released.
    /* Context usage:
     * u16 key_code = data.data.u16[0];
     */
    CODE_KEY_RELEASED = 0x03,

    // Mouse button pressed.
    /* Context usage:
     * u16 button = data.data.u16[0];
     */
    CODE_BUTTON_PRESSED = 0x04,

    // Mouse button released.
    /* Context usage:
     * u16 button = data.data.u16[0];
     */
    CODE_BUTTON_RELEASED = 0x05,

    // Mouse moved.
    /* Context usage:
     * u16 x = data.data.u16[0];
     * u16 y = data.data.u16[1];
     */
    CODE_MOUSE_MOVED = 0x06,

    // Mouse moved.
    /* Context usage:
     * u8 z_delta = data.data.u8[0];
     */
    CODE_MOUSE_WHEEL = 0x07,

    // Resized/resolution changed from the OS
    /* Context usage:
     * u16 width = data.data.u16[0];
     * u16 height = data.data.u16[0];
     */
    CODE_RESIZED = 0x08,

    MAX_CODE = 0xFF,
}
