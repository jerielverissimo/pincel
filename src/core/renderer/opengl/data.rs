#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct f32_f32_f32 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
}

impl f32_f32_f32 {
    pub fn new(d0: f32, d1: f32, d2: f32) -> f32_f32_f32 {
        f32_f32_f32 { d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f32, f32, f32)> for f32_f32_f32 {
    fn from(other: (f32, f32, f32)) -> Self {
        f32_f32_f32::new(other.0, other.1, other.2)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u2_u10_u10_u10_rev_float {
    pub inner: ::vec_2_10_10_10::Vector,
}

impl u2_u10_u10_u10_rev_float {
    pub fn new(inner: ::vec_2_10_10_10::Vector) -> u2_u10_u10_u10_rev_float {
        u2_u10_u10_u10_rev_float { inner }
    }

    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            4, // the number of components per generic vertex attribute
            gl::UNSIGNED_INT_2_10_10_10_REV, // data type
            gl::TRUE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f32, f32, f32, f32)> for u2_u10_u10_u10_rev_float {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        u2_u10_u10_u10_rev_float {
            inner: ::vec_2_10_10_10::Vector::new(other.0, other.1, other.2, other.3),
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f16_f16 {
    pub d0: ::half::f16,
    pub d1: ::half::f16,
}

impl f16_f16 {
    pub fn new(d0: ::half::f16, d1: ::half::f16) -> f16_f16 {
        f16_f16 { d0, d1 }
    }

    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            2,              // the number of components per generic vertex attribute
            gl::HALF_FLOAT, // data type
            gl::FALSE,      // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(::half::f16, ::half::f16)> for f16_f16 {
    fn from(other: (::half::f16, ::half::f16)) -> Self {
        f16_f16::new(other.0, other.1)
    }
}

impl From<(f32, f32)> for f16_f16 {
    fn from(other: (f32, f32)) -> Self {
        f16_f16::new(
            ::half::f16::from_f32(other.0),
            ::half::f16::from_f32(other.1),
        )
    }
}
