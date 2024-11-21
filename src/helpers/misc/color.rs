use serde::Serialize;


#[derive(Serialize)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Color {
    #[allow(unused)] // may be used in a future.
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue
        }
    }

    pub fn as_hex(&self) -> u32 {
        ((self.red as u32) << 16)
        | ((self.green as u32) << 8)
        | self.blue as u32
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self {
            red: ((value >> 16) & 0xFF) as u8,
            green: ((value >> 8) & 0xFF) as u8,
            blue: (value & 0xFF) as u8
        }
    }
}
