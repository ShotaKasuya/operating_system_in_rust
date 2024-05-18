pub struct PixelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl PixelColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
        }
    }

    pub const fn black() -> Self {
        Self::new(0, 0, 0)
    }
    pub const fn white() -> Self {
        Self::new(255, 255, 255)
    }
    pub const fn red() -> Self {
        Self::new(255, 0, 0)
    }
    pub const fn green() -> Self {
        Self::new(0, 255, 0)
    }
    pub const fn blue() -> Self {
        Self::new(0, 0, 255)
    }
    pub const fn magenta() -> Self {
        Self::new(255, 0, 255)
    }
    pub const fn cyan() -> Self {
        Self::new(0, 255, 255)
    }
    pub const fn yellow() -> Self {
        Self::new(255, 255, 0)
    }

    pub const fn byte_to_color(byte: u8) -> Self {
        Self::new(
            byte, byte, byte,
        )
    }
}
