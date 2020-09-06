use super::*;

pub struct MotionCommand<'c, C: Connection + Send + Sync> {
    event: EventHandler<'c, C>,
}

impl<'c, C: Connection + Send + Sync> MotionCommand<'c, C> {
    pub fn new(app: &'c mut Application<C>, event: MotionNotifyEvent) -> Self {
        Self {
            event: EventHandler {
                app,
                event: Event::MotionNotify(event),
            },
        }
    }
}

impl<C: Connection + Send + Sync> Command for MotionCommand<'_, C> {
    fn execute(&mut self) -> Result<(), PincelError> {
        self.event.moving()?;
        Ok(())
    }
}
