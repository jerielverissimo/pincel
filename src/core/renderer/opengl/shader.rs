use core::fmt;
use std::ffi::{CStr, CString};

use gl::types::*;
use nalgebra as na;

use crate::core::resources::{self, Resources};

#[derive(Debug)]
pub enum Error {
    ResourceLoad {
        name: String,
        inner: resources::Error,
    },
    CanNotDetermineShaderTypeForResource {
        name: String,
    },
    CompileError {
        name: String,
        message: String,
    },
    LinkError {
        name: String,
        message: String,
    },
    UniformLocationNotFound {
        program_name: String,
        uniform_name: String,
    },
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ResourceLoad { name, inner } => write!(f, "{} {} ", name, inner),
            Error::CanNotDetermineShaderTypeForResource { name } => write!(f, "{} ", name),
            Error::CompileError { name, message } => write!(f, "{} {} ", name, message),
            Error::LinkError { name, message } => write!(f, "{} {} ", name, message),
            Error::UniformLocationNotFound {
                program_name,
                uniform_name,
            } => write!(f, "{} {} ", program_name, uniform_name),
        }
    }
}
pub struct Program {
    id: gl::types::GLuint,
    name: String,
}

impl Program {
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn from_res(res: &Resources, name: &str) -> Result<Program, Error> {
        const POSSIBLE_EXT: [&str; 2] = [".vert", ".frag"];

        let resource_names = POSSIBLE_EXT
            .iter()
            .map(|file_extension| format!("{}{}", name, file_extension))
            .collect::<Vec<String>>();

        let shaders = resource_names
            .iter()
            .map(|resource_name| Shader::from_res(res, resource_name))
            .collect::<Result<Vec<Shader>, Error>>()?;

        Program::from_shaders(name, &shaders[..]).map_err(|message| Error::LinkError {
            name: name.into(),
            message,
        })
    }

    pub fn from_shaders(name: &str, shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()) };
        }

        unsafe { gl::LinkProgram(program_id) };

        let mut sucess: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut sucess);
        }

        if sucess == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error: CString = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()) };
        }

        Ok(Program {
            name: name.into(),
            id: program_id,
        })
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn get_uniform_location(&self, name: &str) -> Result<i32, Error> {
        let cname = CString::new(name).expect("expected uniform name to have no nul bytes");

        let location = unsafe {
            gl::GetUniformLocation(self.id, cname.as_bytes_with_nul().as_ptr() as *const i8)
        };

        if location == -1 {
            return Err(Error::UniformLocationNotFound {
                program_name: self.name.clone(),
                uniform_name: name.into(),
            });
        }

        Ok(location)
    }

    pub fn set_uniform_matrix_4fv(&self, location: i32, value: &na::Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                location,
                1,
                gl::FALSE,
                value.as_slice().as_ptr() as *const f32,
            );
        }
    }

    pub fn set_uniform_3f(&self, location: i32, value: &na::Vector3<f32>) {
        unsafe {
            gl::Uniform3f(location, value.x, value.y, value.z);
        }
    }

    pub fn set_uniform_1i(&self, location: i32, index: i32) {
        unsafe {
            gl::Uniform1i(location, index);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn from_res(res: &Resources, name: &str) -> Result<Shader, Error> {
        const POSSIBLE_EXT: [(&str, gl::types::GLenum); 2] =
            [(".vert", gl::VERTEX_SHADER), (".frag", gl::FRAGMENT_SHADER)];

        let shader_kind = POSSIBLE_EXT
            .iter()
            .find(|&&(file_extension, _)| name.ends_with(file_extension))
            .map(|&(_, kind)| kind)
            .ok_or_else(|| Error::CanNotDetermineShaderTypeForResource { name: name.into() })?;

        let source = res.load_cstring(name).map_err(|e| Error::ResourceLoad {
            name: name.into(),
            inner: e,
        })?;

        Shader::from_source(&source, shader_kind).map_err(|message| Error::CompileError {
            name: name.into(),
            message,
        })
    }
    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    fn from_source(source: &CStr, kind: gl::types::GLuint) -> Result<Shader, String> {
        let id = match shader_from_source(source, kind) {
            Ok(id) => id,
            Err(error) => return Err(error.into()),
        };
        Ok(Shader { id })
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(source: &CStr, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut sucess: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut sucess);
    }

    if sucess == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error: CString = create_whitespace_cstring_with_len(len as usize);
        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));

    unsafe { CString::from_vec_unchecked(buffer) }
}
