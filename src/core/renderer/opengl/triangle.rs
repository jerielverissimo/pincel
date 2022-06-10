use crate::core::resources::Resources;

use super::{
    buffer::{ArrayBuffer, VertexArray},
    shader::{Error, Program},
    vertex::Vertex,
};

use nalgebra as na;

pub struct Triangle {
    program: Program,
    program_view_projection_location: i32,
    vbo: ArrayBuffer,
    vao: VertexArray,
}

impl Triangle {
    pub fn new(res: &Resources) -> Result<Self, Error> {
        let program = Program::from_res(res, "shaders/triangle")?;
        let program_view_projection_location = program.get_uniform_location("ViewProjection")?;

        let vertices: Vec<Vertex> = vec![
            Vertex {
                pos: (0.5, -0.5, 0.0).into(),
                clr: (1.0, 0.0, 0.0, 1.0).into(),
            }, // bottom right
            Vertex {
                pos: (0.0, 0.5, 0.0).into(),
                clr: (0.0, 0.0, 1.0, 1.0).into(),
            }, // top
            Vertex {
                pos: (-0.5, -0.5, 0.0).into(),
                clr: (0.0, 1.0, 0.0, 1.0).into(),
            }, // bottom left
        ];

        let vbo = ArrayBuffer::new();
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        let vao = VertexArray::new();

        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers();
        vbo.unbind();
        vao.unbind();

        Ok(Self {
            program,
            program_view_projection_location,
            vao,
            vbo,
        })
    }

    pub fn render(&self, vp_matrix: &na::Matrix4<f32>) {
        self.program.set_used();
        self.program
            .set_uniform_matrix_4fv(self.program_view_projection_location, &vp_matrix);
        self.vao.bind();

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
