use super::{Application, ButtonPressEvent, Command, Connection, Event, EventHandler, Result};

pub struct ScrollWheelUpCommand<'c, C: Connection + Send + Sync> {
    event: EventHandler<'c, C>,
}

impl<'c, C: Connection + Send + Sync> ScrollWheelUpCommand<'c, C> {
    pub fn new(app: &'c mut Application<C>, event: ButtonPressEvent) -> Self {
        Self {
            event: EventHandler {
                app,
                event: Event::ButtonPress(event),
            },
        }
    }
}

impl<C: Connection + Send + Sync> Command for ScrollWheelUpCommand<'_, C> {
    fn execute(&mut self) -> Result {
        self.event.scroll_wheel_up()?;
        Ok(())
    }
}
