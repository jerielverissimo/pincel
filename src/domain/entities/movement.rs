use super::color::CurrentColor;

use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;

use std::error::Error;

const PADDING: i16 = 5;

/// A drawed objects, consisting of either a single
/// rhomboid, or two rhomboids connected by a winding line
#[derive(Clone, Debug)]
pub struct Movement {
    pub color: CurrentColor,
    pub lines: Vec<(i16, i16)>,

    left: i16,
    right: i16,
    top: i16,
    bottom: i16,

    time: u32,
    is_finished: bool,
}

impl Movement {
    pub fn new(ev: ButtonPressEvent, color: CurrentColor) -> Self {
        let left = ev.event_x - PADDING;
        let right = ev.event_x + PADDING;
        let top = ev.event_y - PADDING;
        let bottom = ev.event_y + PADDING;
        let mut lines = Vec::new();

        lines.push((ev.event_x, ev.event_y));

        Self {
            color,
            left,
            right,
            top,
            bottom,
            time: ev.time,
            lines,
            is_finished: false,
        }
    }

    pub fn motion(
        &mut self,
        conn: &impl Connection,
        win: u32,
        gc: u32,
        (event_x, event_y, time): (i16, i16, u32),
    ) -> Result<(), Box<dyn Error>> {
        if self.is_finished {
            return Ok(());
        }
        // Find all the mouse coordinates since the
        // last event received

        let events = get_motion_events(conn, win, self.time, time)?;
        self.time = time;

        // Record the previous last coordinate, and append
        // the new coordinates
        let firstline = self.lines.len() - 1;

        // Discard the first coordinate if that is identical to
        // the last recorde coordinate
        let events = events.reply().unwrap().events;

        if events.is_empty() {
            return Ok(());
        }

        let pos = events[0];
        let mut events_without_repeated_coords = Vec::new();

        if (pos.x, pos.y) == *self.lines.last().unwrap() {
            let (_, e) = events.split_first().unwrap();
            events_without_repeated_coords = e.to_vec();
        }

        // Append all coordinates
        for pos in events_without_repeated_coords {
            let x = pos.x;
            let y = pos.y;

            if x < self.left {
                self.left = x
            }
            if x > self.right {
                self.right = x
            }

            if y < self.top {
                self.top = y
            }
            if y > self.bottom {
                self.bottom = y
            }

            self.lines.push((x, y));
        }

        // Append the event coordinate, if that is different from the
        // last movement coordiante
        if (event_x, event_y) != *self.lines.last().unwrap() {
            self.lines.push((event_x, event_y))
        }

        let (_, s) = self.lines.split_at(firstline);

        // Draw a line between the new coordinates
        conn.poly_line(CoordMode::Origin, win, gc, &Self::points_from(&s))?;
        Ok(())
    }

    pub fn finish(
        &mut self,
        conn: &impl Connection,
        win: u32,
        gc: u32,
        ev: &ButtonReleaseEvent,
    ) -> Result<(), Box<dyn Error>> {
        self.motion(conn, win, gc, (ev.event_x, ev.event_y, ev.time))?;

        if self.lines.len() > 1 {
            self.left = i16::min(ev.event_x - PADDING, self.left);
            self.right = i16::max(ev.event_x + PADDING, self.right);
            self.top = i16::min(ev.event_y - PADDING, self.top);
            self.bottom = i16::max(ev.event_y + PADDING, self.bottom);
        }

        self.is_finished = true;
        Ok(())
    }

    pub fn expose(&self, conn: &impl Connection, win: u32, gc: u32, ev: &ExposeEvent) {
        // We should check if this object is in the exposed
        // area, but I can't be bohered right now, so just
        // redraw on the last Expose in every batch

        if ev.count == 0 {
            // Redraw all the lines
            poly_line(conn, CoordMode::Origin, win, gc, &self.points()).unwrap();
        }
    }

    fn points_from(lines: &[(i16, i16)]) -> Vec<Point> {
        let mut points = Vec::new();
        for line in lines {
            points.push(Point {
                x: line.0,
                y: line.1,
            });
        }
        points
    }

    fn points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for line in &self.lines {
            points.push(Point {
                x: line.0,
                y: line.1,
            });
        }
        points
    }
}
