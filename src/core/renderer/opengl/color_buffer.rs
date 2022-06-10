use nalgebra as na;
pub struct ColorBuffer {}

impl ColorBuffer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn set_clear_color(&mut self, color: na::Vector3<f32>) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, 1.0);
        }
    }

    pub fn set_default_blend_func(&self) {
        unsafe {
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn enable_blend(&self) {
        unsafe {
            gl::Enable(gl::BLEND);
        }
    }

    pub fn disable_blend(&self) {
        unsafe {
            gl::Disable(gl::BLEND);
        }
    }
}
