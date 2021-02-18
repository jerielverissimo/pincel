use super::{Application, ButtonPressEvent, Command, Connection, Event, EventHandler, Result};

pub struct MiddleClickCommand<'c, C: Connection + Send + Sync> {
    event: EventHandler<'c, C>,
}

impl<'c, C: Connection + Send + Sync> MiddleClickCommand<'c, C> {
    pub fn new(app: &'c mut Application<C>, event: ButtonPressEvent) -> Self {
        Self {
            event: EventHandler {
                app,
                event: Event::ButtonPress(event),
            },
        }
    }
}

impl<C: Connection + Send + Sync> Command for MiddleClickCommand<'_, C> {
    fn execute(&mut self) -> Result {
        self.event.clear_all_draws()?;
        Ok(())
    }
}
