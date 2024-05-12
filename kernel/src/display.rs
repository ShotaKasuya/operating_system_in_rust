use bootloader_api::info::{FrameBufferInfo, PixelFormat};

pub struct Display {
    buffer: &'static mut [u8],
    info: FrameBufferInfo,
}

pub struct PixelColor {
    r: u8,
    g: u8,
    b: u8,
}

impl Display {
    pub fn new(buffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        Self {
            buffer,
            info,
        }
    }
    pub fn write_pixel(&mut self, x: u8, y: u8, color: &PixelColor) -> bool {
        let pixel_position = (self.info.stride as u8 * y + x) as usize;
        match self.info.pixel_format {
            PixelFormat::Rgb => {
                self.buffer[pixel_position] = color.r;
                self.buffer[pixel_position + 1] = color.g;
                self.buffer[pixel_position + 2] = color.b;
                true
            }
            PixelFormat::Bgr => {
                self.buffer[pixel_position] = color.b;
                self.buffer[pixel_position + 1] = color.g;
                self.buffer[pixel_position + 2] = color.r;
                true
            }
            _ => {
                false
            }
        }
    }
}

impl PixelColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
        }
    }
}
