use super::{Application, Command, Connection, Event, EventHandler, ExposeEvent, Result};

pub struct DrawCommand<'c, C: Connection + Send + Sync> {
    event: EventHandler<'c, C>,
}

impl<'c, C: Connection + Send + Sync> DrawCommand<'c, C> {
    pub fn new(app: &'c mut Application<C>, event: ExposeEvent) -> Self {
        Self {
            event: EventHandler {
                app,
                event: Event::Expose(event),
            },
        }
    }
}

impl<C: Connection + Send + Sync> Command for DrawCommand<'_, C> {
    fn execute(&mut self) -> Result {
        self.event.draw()?;
        Ok(())
    }
}
