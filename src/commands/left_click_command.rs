use super::*;

pub struct LeftClickCommand<'c, C: Connection + Send + Sync> {
    event: EventHandler<'c, C>,
}

impl<'c, C: Connection + Send + Sync> LeftClickCommand<'c, C> {
    pub fn new(app: &'c mut Application<C>, event: ButtonPressEvent) -> Self {
        Self {
            event: EventHandler {
                app,
                event: Event::ButtonPress(event),
            },
        }
    }
}

impl<C: Connection + Send + Sync> Command for LeftClickCommand<'_, C> {
    fn execute(&mut self) -> Result {
        self.event.left_click()?;
        Ok(())
    }
}
