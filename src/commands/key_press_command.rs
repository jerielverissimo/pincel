use super::*;

pub struct KeyPressCommand<'c, C: Connection + Send + Sync> {
    event: EventHandler<'c, C>,
}

impl<'c, C: Connection + Send + Sync> KeyPressCommand<'c, C> {
    pub fn new(app: &'c mut Application<C>, event: KeyPressEvent) -> Self {
        Self {
            event: EventHandler {
                app,
                event: Event::KeyPress(event),
            },
        }
    }
}

impl<C: Connection + Send + Sync> Command for KeyPressCommand<'_, C> {
    fn execute(&mut self) -> Result<(), PincelError> {
        self.event.key_press_handler()
    }
}
