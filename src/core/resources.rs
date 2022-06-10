use core::fmt;
use std::{
    ffi, fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileContainsNil,
    FailedToGetExePath,
    FailedToLoadImage(image::ImageError),
    ImageIsNotRgba { name: String },
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "{}", err),
            Error::FileContainsNil => write!(f, "FileContainsNil"),
            Error::FailedToGetExePath => write!(f, "FailedToGetExePath"),
            Error::FailedToLoadImage(err) => write!(f, "{}", err),
            Error::ImageIsNotRgba { name } => write!(f, "{}", name),
        }
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

impl From<image::ImageError> for Error {
    fn from(other: image::ImageError) -> Self {
        Error::FailedToLoadImage(other)
    }
}

pub struct Resources {
    root_path: PathBuf,
}

impl Resources {
    pub fn from_relative_exe_path(rel_path: &Path) -> Result<Resources, Error> {
        let exe_file_name = std::env::current_exe().map_err(|_| Error::FailedToGetExePath)?;

        let exe_path = exe_file_name.parent().ok_or(Error::FailedToGetExePath)?;

        Ok(Resources {
            root_path: exe_path.join(rel_path),
        })
    }

    pub fn from_exe_path() -> Result<Resources, Error> {
        Resources::from_relative_exe_path(Path::new(""))
    }

    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(resource_name_to_path(&self.root_path, resource_name))?;

        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);
        file.read_to_end(&mut buffer)?;

        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(Error::FileContainsNil);
        }

        Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
    }

    pub fn load_rgb_image(&self, resource_name: &str) -> Result<image::RgbImage, Error> {
        let img = image::open(resource_name_to_path(&self.root_path, resource_name))?;

        Ok(img.to_rgb())
    }

    pub fn load_rgba_image(&self, resource_name: &str) -> Result<image::RgbaImage, Error> {
        let img = image::open(resource_name_to_path(&self.root_path, resource_name))?;

        if let image::ColorType::RGBA(_) = img.color() {
            Ok(img.to_rgba())
        } else {
            Err(Error::ImageIsNotRgba {
                name: resource_name.into(),
            })
        }
    }
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}
