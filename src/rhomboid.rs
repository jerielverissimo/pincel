use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;

#[derive(Clone, Copy, Debug)]
pub struct Rhomboid {
    x: i16,
    y: i16,
}

impl Rhomboid {
    pub fn new(ev: &ButtonPressEvent, conn: &impl Connection, win: u32, gc: u32) -> Self {
        let rhomboid = Self {
            x: ev.event_x,
            y: ev.event_y,
        };

        rhomboid.draw(conn, win, gc);
        rhomboid
    }

    pub fn draw(&self, conn: &impl Connection, win: u32, gc: u32) {
        poly_line(
            conn,
            CoordMode::Previous,
            win,
            gc,
            &[
                Point {
                    x: self.x,
                    y: self.y - 5,
                },
                Point { x: 5, y: 5 },
                Point { x: -5, y: 5 },
                Point { x: -5, y: -5 },
                Point { x: 5, y: -5 },
            ],
        )
        .unwrap();
    }
}
