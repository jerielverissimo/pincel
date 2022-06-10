use std::mem::size_of;

use super::data;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    pub pos: data::f32_f32_f32,
    pub clr: data::u2_u10_u10_u10_rev_float,
}

impl Vertex {
    pub fn vertex_attrib_pointers() {
        let stride = std::mem::size_of::<Self>();

        let location = 0;
        let offset = 0;

        unsafe {
            data::f32_f32_f32::vertex_attrib_pointer(stride, location, offset);
        }

        let location = 1;
        let offset = offset + std::mem::size_of::<data::u2_u10_u10_u10_rev_float>();

        unsafe {
            data::u2_u10_u10_u10_rev_float::vertex_attrib_pointer(stride, location, offset);
        }
    }
}
