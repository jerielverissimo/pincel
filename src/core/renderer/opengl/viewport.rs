use crate::core::logger::{log_output, LogLevel};

pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Viewport {
    pub fn for_window(w: i32, h: i32) -> Viewport {
        Viewport { x: 0, y: 0, w, h }
    }

    pub fn update_size(&mut self, w: i32, h: i32) {
        self.w = w;
        self.h = h;
    }

    pub fn set_used(&self) {
        unsafe {
            log_output(
                LogLevel::Debug,
                format!("Viewport: {:?}:{:?}", self.w, self.h),
            );
            gl::Viewport(self.x, self.y, self.w, self.h);
        }
    }
}
