#[derive(Debug, Copy, Clone)]
pub enum Keys {
    Q,
    P,
    CapsLock,
    Esc,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    NotSupported,
}

impl From<u8> for Keys {
    fn from(k: u8) -> Self {
        match k {
            9 => Self::Esc,
            10 => Self::One,
            11 => Self::Two,
            12 => Self::Three,
            13 => Self::Four,
            14 => Self::Five,
            15 => Self::Six,
            24 => Self::Q,
            33 => Self::P,
            66 => Self::CapsLock,
            _ => Self::NotSupported,
        }
    }
}
