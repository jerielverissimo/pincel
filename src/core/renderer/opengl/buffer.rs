use std::{
    marker,
    ops::{Deref, DerefMut},
};

pub trait BufferType {
    const BUFFET_TYPE: gl::types::GLuint;
}

pub struct BufferTypeArray;

impl BufferType for BufferTypeArray {
    const BUFFET_TYPE: gl::types::GLuint = gl::ARRAY_BUFFER;
}

pub struct BufferTypeElementArray;

impl BufferType for BufferTypeElementArray {
    const BUFFET_TYPE: gl::types::GLuint = gl::ELEMENT_ARRAY_BUFFER;
}

pub struct Buffer<B>
where
    B: BufferType,
{
    vbo: gl::types::GLuint,
    _marker: marker::PhantomData<B>,
}

impl<B> Buffer<B>
where
    B: BufferType,
{
    pub fn new() -> Buffer<B> {
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        Buffer {
            vbo,
            _marker: marker::PhantomData,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(B::BUFFET_TYPE, self.vbo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(B::BUFFET_TYPE, 0);
        }
    }

    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                B::BUFFET_TYPE,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            )
        }
    }

    pub fn dynamic_draw_data_null<T>(&self, size: usize) {
        unsafe {
            gl::BufferData(
                B::BUFFET_TYPE,
                (size * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                std::ptr::null() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            )
        }
    }

    pub unsafe fn map_buffer_range_write_invalidate<'r, T>(
        &self,
        offset: usize,
        size: usize,
    ) -> Option<MappedBuffer<'r, B, T>> {
        let ptr = gl::MapBufferRange(
            B::BUFFET_TYPE,
            (offset * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
            (size * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
            gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_RANGE_BIT,
        );
        if ptr == std::ptr::null_mut() {
            return None;
        }

        Some(MappedBuffer {
            data: std::slice::from_raw_parts_mut(ptr as *mut T, size),
            _marker: std::marker::PhantomData,
        })
    }
}

impl<B: BufferType> Drop for Buffer<B> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vbo);
        }
    }
}

pub struct MappedBuffer<'a, B, DataT: 'a>
where
    B: BufferType,
{
    data: &'a mut [DataT],
    _marker: marker::PhantomData<B>,
}

impl<'a, B: BufferType, DataT: 'a> Deref for MappedBuffer<'a, B, DataT> {
    type Target = [DataT];

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, B: BufferType, DataT: 'a> DerefMut for MappedBuffer<'a, B, DataT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<'a, B: BufferType, DataT: 'a> Drop for MappedBuffer<'a, B, DataT> {
    fn drop(&mut self) {
        unsafe {
            gl::UnmapBuffer(B::BUFFET_TYPE);
        }
    }
}

pub type ArrayBuffer = Buffer<BufferTypeArray>;
pub type ElementArrayBuffer = Buffer<BufferTypeElementArray>;

pub struct VertexArray {
    vao: gl::types::GLuint,
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        VertexArray { vao }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.vao);
        }
    }
}
