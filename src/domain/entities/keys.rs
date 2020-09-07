#[derive(Debug)]
pub enum Keys {
    Q,
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
            24 => Self::Q,
            66 => Self::CapsLock,
            9 => Self::Esc,
            10 => Self::One,
            11 => Self::Two,
            12 => Self::Three,
            13 => Self::Four,
            14 => Self::Five,
            15 => Self::Six,
            _ => Self::NotSupported,
        }
    }
}
