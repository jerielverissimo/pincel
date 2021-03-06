use std::slice;

use crate::application::app::Application;
use crate::domain::{CurrentColor, CurrentColorSingleton, GraphicContext, Keys, Movement, Result};
use chrono::prelude::*;
use image::RgbImage;
use x11::xlib;
use x11rb::{
    connection::Connection,
    protocol::{xproto::ConnectionExt, Event},
};
use Keys::{CapsLock, Esc, Five, Four, One, Six, Three, Two, P, Q};

use super::LineWidth;

#[derive(Debug)]
struct Bgr {
    b: u8,
    g: u8,
    r: u8,
    _pad: u8,
}

const LEFT_MOUSE_BUTTON: u8 = 1;
const MIDDLE_MOUSE_BUTTON: u8 = 2;
const RIGHT_MOUSE_BUTTON: u8 = 3;
const SCROLL_WHEEL_UP: u8 = 4;
const SCROLL_WHEEL_DOWN: u8 = 5;

pub struct EventHandler<'c, C>
where
    C: Connection + Send + Sync,
{
    pub app: &'c mut Application<C>,
    pub event: Event,
}

impl<C: Connection + Send + Sync> EventHandler<'_, C> {
    pub fn key_press_handler(&mut self) -> Result {
        if let Event::KeyPress(e) = self.event {
            match e.detail.into() {
                Q | CapsLock | Esc => self.exit(),
                One | Two | Three | Four | Five | Six => self.switch_color(e.detail.into()),
                P => self.save_screenshot()?,
                _ => {}
            }
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.app.is_running = false;
    }

    pub fn draw(&self) -> Result {
        if let Event::Expose(e) = self.event {
            for mov in &self.app.stack {
                if let Some(mov) = &mov {
                    self.update_color(Some(mov.color))?;
                    mov.expose(&self.app.conn, self.app.win_id, self.app.gc_id, &e);
                }
            }
            self.app.conn.flush()?;
        }
        Ok(())
    }

    pub fn clear_all_draws(&mut self) -> Result {
        if let Event::ButtonPress(event) = self.event {
            if event.detail == MIDDLE_MOUSE_BUTTON {
                for _ in 0..self.app.stack.len() {
                    if !self.app.stack.is_empty() {
                        self.app.stack.drain(..);
                    }
                }
                self.update_screen()?;
            }
        }
        Ok(())
    }

    pub fn left_click(&mut self) -> Result {
        if let Event::ButtonPress(event) = self.event {
            if event.detail == LEFT_MOUSE_BUTTON {
                self.update_color(None)?;

                let temp = Some(Movement::new(event, self.app.brush_color.clone().into()));
                self.app.stack.push(temp);
                self.app.current = self.app.stack.len() - 1;

                self.app.skip();
            }
        }
        Ok(())
    }

    pub fn right_click(&mut self) -> Result {
        if let Event::ButtonPress(event) = self.event {
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
                self.update_screen()?;
            }
        }
        Ok(())
    }

    pub fn left_release(&mut self) -> Result {
        if let Event::ButtonRelease(event) = self.event {
            if event.detail == LEFT_MOUSE_BUTTON {
                self.app.stack[self.app.current]
                    .as_mut()
                    .unwrap()
                    .finish(&self.app.conn, self.app.win_id, self.app.gc_id, &event)
                    .unwrap();
            }
        }
        Ok(())
    }

    pub fn scroll_wheel_up(&mut self) -> Result {
        if let Event::ButtonPress(event) = self.event {
            if event.detail == SCROLL_WHEEL_UP {
                let width = &mut self.app.line_width;
                match width {
                    LineWidth::Thin => {
                        *width = LineWidth::Normal;
                    }
                    LineWidth::Normal => {
                        *width = LineWidth::Wide;
                    }
                    LineWidth::Wide => {}
                }
            }
            self.update_screen()?;
        }
        Ok(())
    }

    pub fn scroll_wheel_down(&mut self) -> Result {
        if let Event::ButtonPress(event) = self.event {
            if event.detail == SCROLL_WHEEL_DOWN {
                let width = &mut self.app.line_width;
                match width {
                    LineWidth::Thin => {}
                    LineWidth::Normal => {
                        *width = LineWidth::Thin;
                    }
                    LineWidth::Wide => {
                        *width = LineWidth::Normal;
                    }
                }
            }
            self.update_screen();
        }
        Ok(())
    }

    fn update_screen(&mut self) -> Result {
        self.app
            .conn
            .clear_area(true, self.app.win_id, 0, 0, 0, 0)?;
        self.app.conn.flush()?;
        self.app.skip();
        Ok(())
    }

    pub fn moving(&mut self) -> Result {
        if self.app.stack.is_empty() {
            return Ok(());
        }

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

    fn update_color(&self, with_color: Option<CurrentColor>) -> Result {
        let new_gc;
        if let Some(color) = with_color {
            new_gc = GraphicContext::change_color(color.value(), self.app.line_width);
        } else {
            new_gc =
                GraphicContext::change_color(self.app.brush_color.value(), self.app.line_width);
        }
        self.app.conn.change_gc(self.app.gc_id, &new_gc)?;

        self.app.conn.flush()?;
        Ok(())
    }

    fn switch_color(&self, key: Keys) {
        let color = match key {
            One => "red",
            Two => "blue",
            Three => "yellow",
            Four => "green",
            Five => "orange",
            Six => "black",
            _ => "",
        };
        let mut brush_color = CurrentColorSingleton::new();
        brush_color.set(color);
    }

    unsafe fn save_ximage(&self, path: &str, image: *mut xlib::XImage, w: u32, h: u32) {
        if !image.is_null() {
            let image = &mut *image;
            let sl: &[Bgr] = {
                slice::from_raw_parts(
                    (image).data as *const _,
                    (image).width as usize * (image).height as usize,
                )
            };

            let mut bgr_iter = sl.iter();
            let mut image_buffer = RgbImage::new(w, h);

            for pix in image_buffer.pixels_mut() {
                let bgr = bgr_iter.next().unwrap();
                pix.0 = [bgr.r, bgr.g, bgr.b];
            }

            image_buffer.save(path).unwrap();
        }
    }

    fn copy_desktop_image(&self, path: &str) {
        unsafe {
            let dis = xlib::XOpenDisplay(std::ptr::null::<i8>());
            let scr = xlib::XDefaultScreenOfDisplay(dis);
            let drawable = xlib::XDefaultRootWindow(dis);
            let w = (*scr).width as u32;
            let h = (*scr).height as u32;

            let image =
                xlib::XGetImage(dis, drawable, 0, 0, w, h, xlib::XAllPlanes(), xlib::ZPixmap);

            self.save_ximage(path, image, w, h);

            xlib::XDestroyImage(image);
            xlib::XCloseDisplay(dis);
        }
    }

    fn save_screenshot(&self) -> Result {
        let home = std::env::home_dir().unwrap();
        let current_date_time: String = Utc::now()
            .to_string()
            .split('.')
            .into_iter()
            .collect::<Vec<&str>>()[0]
            .to_string();

        let path_str = self.app.config.screenshot_dir.as_str().to_owned()
            + "Screenshot from "
            + &current_date_time
            + ".png";

        let path = home.join(std::path::PathBuf::from(path_str));

        self.copy_desktop_image(path.to_str().unwrap());

        Ok(())
    }
}
