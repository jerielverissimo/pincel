use std::sync::{Arc, Mutex, Once};

#[derive(Clone, Debug)]
pub struct CurrentColorSingleton {
    inner: Arc<Mutex<CurrentColor>>,
}

impl CurrentColorSingleton {
    pub fn set(&mut self, name: &str) {
        let mut data = self.inner.lock().unwrap();
        *data = CurrentColor::from(name);
    }

    pub fn value(&self) -> u32 {
        let data = self.inner.lock().unwrap();
        data.value()
    }
}

#[derive(Debug)]
pub enum CurrentColor {
    Red,
    Blue,
    Yellow,
    Green,
    Orange,
    NotSupported,
}

impl CurrentColor {
    /// Creates a singleton
    pub fn new() -> CurrentColorSingleton {
        static mut COLOR: *const CurrentColorSingleton = 0 as *const _;
        static ONCE: Once = Once::new();

        ONCE.call_once(|| unsafe {
            let color = CurrentColorSingleton {
                inner: Arc::new(Mutex::new(CurrentColor::NotSupported)),
            };

            COLOR = std::mem::transmute(Box::new(color));
        });

        unsafe { (&*COLOR).clone() }
    }

    fn value(&self) -> u32 {
        match self {
            CurrentColor::Red => 0xff3c3c,
            CurrentColor::Blue => 0x1e6ec8,
            CurrentColor::Yellow => 0xebb437,
            CurrentColor::Green => 0x78af46,
            CurrentColor::Orange => 0xeb7d0f,
            CurrentColor::NotSupported => 0x00,
        }
    }
}

impl From<&str> for CurrentColor {
    fn from(name: &str) -> Self {
        match name {
            "red" => CurrentColor::Red,
            "blue" => CurrentColor::Blue,
            "yellow" => CurrentColor::Yellow,
            "green" => CurrentColor::Green,
            "orange" => CurrentColor::Orange,
            _ => Self::NotSupported,
        }
    }
}
