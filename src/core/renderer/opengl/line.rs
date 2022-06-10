use std::{cell::RefCell, rc::Rc, slice::SliceIndex};

use crate::core::resources::Resources;

use super::{
    buffer::{ArrayBuffer, VertexArray},
    color_buffer::ColorBuffer,
    line_point::LinePoint,
    shader::{Error, Program},
};
use nalgebra as na;

pub struct Line {
    program: Program,
    program_view_projection_location: i32,
    lines_vbo_count: i32,
    lines_vbo: ArrayBuffer,
    lines_vbo_capacity: Option<usize>,
    lines_vao: VertexArray,
}

impl Line {
    pub fn new(res: &Resources) -> Result<Self, Error> {
        let lines_vbo = ArrayBuffer::new();
        let lines_vao = VertexArray::new();
        lines_vao.bind();
        lines_vbo.bind();
        LinePoint::vertex_attrib_pointers();
        lines_vbo.unbind();
        lines_vao.unbind();

        let program = Program::from_res(res, "shaders/line")?;
        let program_view_projection_location = program.get_uniform_location("projection")?;

        Ok(Self {
            program,
            program_view_projection_location,
            lines_vbo,
            lines_vbo_count: 0,
            lines_vbo_capacity: None,
            lines_vao,
        })
    }

    pub fn render(&mut self, target: &ColorBuffer, vp_matrix: &na::Matrix4<f32>) {
        if self.lines_vbo_count > 0 {
            self.program.set_used();
            self.program
                .set_uniform_matrix_4fv(self.program_view_projection_location, &vp_matrix);

            self.lines_vbo.bind();

            unsafe {
                target.set_default_blend_func();
                target.enable_blend();

                gl::DrawArrays(gl::LINES, 0, self.lines_vbo_count);

                target.disable_blend();
            }
        }
    }
}
