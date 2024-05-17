use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use crate::display::cursor::MOUSE_CURSOR_SHAPE;

pub mod cursor;

pub struct Display {
    buffer: &'static mut [u8],
    pub info: FrameBufferInfo,
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
    pub fn write_pixel(&mut self, x: usize, y: usize, color: &PixelColor) -> bool {
        let pixel_position = x + self.info.stride * y;
        let byte_position = pixel_position * self.info.bytes_per_pixel;
        match self.info.pixel_format {
            PixelFormat::Rgb => {
                self.buffer[byte_position] = color.r;
                self.buffer[byte_position + 1] = color.g;
                self.buffer[byte_position + 2] = color.b;
                true
            }
            PixelFormat::Bgr => {
                self.buffer[byte_position] = color.b;
                self.buffer[byte_position + 1] = color.g;
                self.buffer[byte_position + 2] = color.r;
                true
            }
            _ => { false }
        }
    }

    pub fn print_cursor(&mut self, pos_x: usize, pos_y: usize) {
        for (y, mouse_cursor_sh) in MOUSE_CURSOR_SHAPE.iter().enumerate() {
            for (x, mouse_cursor_sh) in mouse_cursor_sh.iter().enumerate() {
                match mouse_cursor_sh {
                    b'@' => {
                        self.write_pixel(pos_x + x, pos_y + y, &PixelColor::new(0, 0, 0));
                    }
                    b'.' => {
                        self.write_pixel(pos_x + x, pos_y + y, &PixelColor::new(255, 255, 255));
                    }
                    _ => {}
                }
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
