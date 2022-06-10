use std::os::raw;

use crate::core::resources::{Error, Resources};

pub struct TextureLoadOptions<'a> {
    resource_name: &'a str,
    format: gl::types::GLenum,
    pub gen_mipmaps: bool,
}

impl<'a> TextureLoadOptions<'a> {
    pub fn from_res_rgb(resource_name: &str) -> TextureLoadOptions {
        TextureLoadOptions {
            resource_name,
            format: gl::RGB,
            gen_mipmaps: false,
        }
    }

    pub fn from_res_rgba(resource_name: &str) -> TextureLoadOptions {
        TextureLoadOptions {
            resource_name,
            format: gl::RGBA,
            gen_mipmaps: false,
        }
    }
}

pub struct TextureLoadBuilder<'a> {
    options: TextureLoadOptions<'a>,
}

impl<'a> TextureLoadBuilder<'a> {
    pub fn load(self, res: &Resources) -> Result<Texture, Error> {
        Texture::from_res(self.options, res)
    }

    pub fn with_gen_mipmaps(mut self) -> Self {
        self.options.gen_mipmaps = true;
        self
    }
}

pub struct Texture {
    obj: gl::types::GLuint,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &mut self.obj) };
    }
}

impl Texture {
    pub fn from_res_rgb(resource_name: &str) -> TextureLoadBuilder {
        TextureLoadBuilder {
            options: TextureLoadOptions::from_res_rgb(resource_name),
        }
    }

    pub fn from_res_rgba(resource_name: &str) -> TextureLoadBuilder {
        TextureLoadBuilder {
            options: TextureLoadOptions::from_res_rgba(resource_name),
        }
    }

    pub fn from_res<'a>(
        options: TextureLoadOptions<'a>,
        res: &Resources,
    ) -> Result<Texture, Error> {
        let mut obj: gl::types::GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut obj);
        }

        let texture = Texture { obj };

        texture.update(options, res)?;

        Ok(texture)
    }

    pub fn update<'a>(
        &self,
        options: TextureLoadOptions<'a>,
        res: &Resources,
    ) -> Result<(), Error> {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.obj);
        }

        // https://www.khronos.org/opengl/wiki/Common_Mistakes

        match options.format {
            gl::RGB => {
                let img = res.load_rgb_image(options.resource_name)?;

                if options.gen_mipmaps {
                    unsafe {
                        gl::TexImage2D(
                            gl::TEXTURE_2D,
                            0,
                            gl::RGB8 as gl::types::GLint,
                            img.width() as i32,
                            img.height() as i32,
                            0,
                            gl::RGB,
                            gl::UNSIGNED_BYTE,
                            img.as_ptr() as *const raw::c_void,
                        );
                        gl::GenerateMipmap(gl::TEXTURE_2D);
                    }
                } else {
                    unsafe {
                        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_BASE_LEVEL, 0);
                        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);
                        gl::TexImage2D(
                            gl::TEXTURE_2D,
                            0,
                            gl::RGB8 as gl::types::GLint,
                            img.width() as i32,
                            img.height() as i32,
                            0,
                            gl::RGB,
                            gl::UNSIGNED_BYTE,
                            img.as_ptr() as *const raw::c_void,
                        );
                    }
                }
            }
            gl::RGBA => {
                let img = res.load_rgba_image(options.resource_name)?;

                if options.gen_mipmaps {
                    unsafe {
                        gl::TexImage2D(
                            gl::TEXTURE_2D,
                            0,
                            gl::RGBA8 as gl::types::GLint,
                            img.width() as i32,
                            img.height() as i32,
                            0,
                            gl::RGBA,
                            gl::UNSIGNED_BYTE,
                            img.as_ptr() as *const raw::c_void,
                        );
                        gl::GenerateMipmap(gl::TEXTURE_2D);
                    }
                } else {
                    unsafe {
                        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_BASE_LEVEL, 0);
                        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);
                        gl::TexImage2D(
                            gl::TEXTURE_2D,
                            0,
                            gl::RGBA8 as gl::types::GLint,
                            img.width() as i32,
                            img.height() as i32,
                            0,
                            gl::RGBA,
                            gl::UNSIGNED_BYTE,
                            img.as_ptr() as *const raw::c_void,
                        );
                    }
                }
            }
            _ => unreachable!("Only RGB or RGBA images can be constructed"),
        }

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok(())
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.obj);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn bind_at(&self, index: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + index);
        }
        self.bind();
    }
}
