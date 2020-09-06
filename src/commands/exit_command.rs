use super::*;

pub struct ExitCommand<'c, C: Connection + Send + Sync> {
    event: EventHandler<'c, C>,
}

impl<'c, C: Connection + Send + Sync> ExitCommand<'c, C> {
    pub fn new(app: &'c mut Application<C>, event: KeyPressEvent) -> Self {
        Self {
            event: EventHandler {
                app,
                event: Event::KeyPress(event),
            },
        }
    }
}

impl<C: Connection + Send + Sync> Command for ExitCommand<'_, C> {
    fn execute(&mut self) -> Result<(), PincelError> {
        self.event.exit();
        Ok(())
    }
}
