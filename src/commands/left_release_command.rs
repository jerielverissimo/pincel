use super::{Application, ButtonReleaseEvent, Command, Connection, Event, EventHandler, Result};

pub struct LeftReleaseCommand<'c, C: Connection + Send + Sync> {
    event: EventHandler<'c, C>,
}

impl<'c, C: Connection + Send + Sync> LeftReleaseCommand<'c, C> {
    pub fn new(app: &'c mut Application<C>, event: ButtonReleaseEvent) -> Self {
        Self {
            event: EventHandler {
                app,
                event: Event::ButtonRelease(event),
            },
        }
    }
}

impl<C: Connection + Send + Sync> Command for LeftReleaseCommand<'_, C> {
    fn execute(&mut self) -> Result {
        self.event.left_release()?;
        Ok(())
    }
}
