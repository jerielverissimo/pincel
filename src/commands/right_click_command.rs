use super::{Application, ButtonPressEvent, Command, Connection, Event, EventHandler, Result};

pub struct RightClickCommand<'c, C: Connection + Send + Sync> {
    event: EventHandler<'c, C>,
}

impl<'c, C: Connection + Send + Sync> RightClickCommand<'c, C> {
    pub fn new(app: &'c mut Application<C>, event: ButtonPressEvent) -> Self {
        Self {
            event: EventHandler {
                app,
                event: Event::ButtonPress(event),
            },
        }
    }
}

impl<C: Connection + Send + Sync> Command for RightClickCommand<'_, C> {
    fn execute(&mut self) -> Result {
        self.event.right_click()?;
        Ok(())
    }
}
