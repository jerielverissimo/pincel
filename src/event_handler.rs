use super::*;

const LEFT_MOUSE_BUTTON: u8 = 1;
const RIGHT_MOUSE_BUTTON: u8 = 3;

pub struct EventHandler<'c, C>
where
    C: Connection + Send + Sync,
{
    pub app: &'c mut Application<C>,
    pub event: Event,
}

impl<C: Connection + Send + Sync> EventHandler<'_, C> {
    pub fn exit(&mut self) {
        if let Event::KeyPress(e) = self.event {
            const Q: u8 = 24;
            const CAPS_LOCK: u8 = 66;
            const ESC: u8 = 9;
            match e.detail {
                Q | CAPS_LOCK | ESC => self.app.is_running = false,
                _ => {}
            }
        }
    }

    pub fn draw(&self) -> Result<(), PincelError> {
        if let Event::Expose(e) = self.event {
            for mov in &self.app.stack {
                if let Some(mov) = &mov {
                    self.uptate_color(Some(mov.color))?;
                    mov.expose(&self.app.conn, self.app.win_id, self.app.gc_id, &e);
                }
            }
            self.app.conn.flush()?;
        }
        Ok(())
    }

    pub fn left_click(&mut self) -> Result<(), PincelError> {
        if let Event::ButtonPress(event) = self.event {
            if event.detail == LEFT_MOUSE_BUTTON {
                self.uptate_color(None)?;

                let temp = Some(Movement::new(event, self.app.brush_color.clone().into()));
                self.app.stack.push(temp);
                self.app.current = self.app.stack.len() - 1;

                self.app.skip();
            }
        }
        Ok(())
    }

    pub fn right_click(&mut self) -> Result<(), PincelError> {
        if let Event::ButtonPress(event) = self.event {
            // right button
            if event.detail == RIGHT_MOUSE_BUTTON {
                if self.app.stack.is_empty() {
                    self.app.skip();
                    return Ok(());
                }
                self.app.stack.pop();
                self.app.current = if !self.app.stack.is_empty() {
                    self.app.stack.len() - 1
                } else {
                    0
                };
                self.app
                    .conn
                    .clear_area(true, self.app.win_id, 0, 0, 0, 0)?;
                self.app.conn.flush()?;
                self.app.skip();
            }
        }
        Ok(())
    }

    pub fn left_release(&mut self) -> Result<(), PincelError> {
        if let Event::ButtonRelease(event) = self.event {
            if event.detail == LEFT_MOUSE_BUTTON {
                self.app.stack[self.app.current].as_mut()?.finish(
                    &self.app.conn,
                    self.app.win_id,
                    self.app.gc_id,
                    &event,
                )?;
            }
        }
        Ok(())
    }

    pub fn moving(&mut self) -> Result<(), PincelError> {
        if let Event::MotionNotify(event) = self.event {
            if let Some(current) = &mut self.app.stack[self.app.current] {
                current.motion(
                    &self.app.conn,
                    self.app.win_id,
                    self.app.gc_id,
                    (event.event_x, event.event_y, event.time),
                )?;
            }
        }
        Ok(())
    }

    fn uptate_color(&self, with_color: Option<CurrentColor>) -> Result<(), PincelError> {
        let new_gc;
        if let Some(color) = with_color {
            new_gc = GraphicContext::change_color(color.value());
        } else {
            new_gc = GraphicContext::change_color(self.app.brush_color.value());
        }
        self.app.conn.change_gc(self.app.gc_id, &new_gc)?;

        self.app.conn.flush()?;
        Ok(())
    }
}
