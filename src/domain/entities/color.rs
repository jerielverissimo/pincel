use std::sync::{Arc, Mutex, Once};

#[derive(Clone, Debug)]
pub struct CurrentColorSingleton {
    inner: Arc<Mutex<CurrentColor>>,
}

impl CurrentColorSingleton {
    /// Creates a singleton
    pub fn new() -> CurrentColorSingleton {
        static mut COLOR: *const CurrentColorSingleton = 0 as *const _;
        static ONCE: Once = Once::new();

        ONCE.call_once(|| unsafe {
            let color = CurrentColorSingleton {
                inner: Arc::new(Mutex::new(CurrentColor::Yellow)),
            };

            COLOR = std::mem::transmute(Box::new(color));
        });

        unsafe { (&*COLOR).clone() }
    }

    pub fn set(&mut self, name: &str) {
        let mut data = self.inner.lock().unwrap();
        *data = CurrentColor::from(name);
    }

    pub fn value(&self) -> u32 {
        let data = self.inner.lock().unwrap();
        data.value()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CurrentColor {
    Red,
    Blue,
    Yellow,
    Green,
    Orange,
    Black,
    NotSupported,
}

impl CurrentColor {
    pub fn value(&self) -> u32 {
        match self {
            CurrentColor::Red => 0xffff3c3c,
            CurrentColor::Blue => 0xff1e6ec8,
            CurrentColor::Yellow => 0xffebb437,
            CurrentColor::Green => 0xff78af46,
            CurrentColor::Orange => 0xffeb7d0f,
            CurrentColor::Black => 0xff242424,
            CurrentColor::NotSupported => 0x00,
        }
    }
}

impl From<&str> for CurrentColor {
    fn from(name: &str) -> Self {
        match name {
            "red" => Self::Red,
            "blue" => Self::Blue,
            "yellow" => Self::Yellow,
            "green" => Self::Green,
            "orange" => Self::Orange,
            "black" => Self::Black,
            _ => Self::NotSupported,
        }
    }
}

impl From<CurrentColorSingleton> for CurrentColor {
    fn from(color: CurrentColorSingleton) -> Self {
        let data = color.inner.lock().unwrap();
        *data
    }
}
