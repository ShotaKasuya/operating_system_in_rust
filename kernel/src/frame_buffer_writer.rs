use bootloader_api::info::{FrameBufferInfo, Optional, PixelFormat};
use lazy_static::lazy_static;
use spin::Mutex;
use crate::frame_buffer_writer::cursor::MOUSE_CURSOR_SHAPE;
use crate::frame_buffer_writer::pixel_color::PixelColor;

pub mod cursor;
pub mod text_writer;
pub mod pixel_color;

lazy_static! {
    pub static ref FRAME_BUFFER_WRITER: Mutex<FrameBufferWriter> = Mutex::new(FrameBufferWriter::new());
}


pub struct FrameBufferWriter {
    framebuffer: Optional<&'static mut [u8]>,
    pub info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
}

impl FrameBufferWriter {
    pub fn new() -> Self {
        Self {
            // tmp values
            framebuffer: Optional::None,
            info: FrameBufferInfo {
                byte_len: 0,
                width: 0,
                height: 0,
                pixel_format: PixelFormat::Rgb,
                bytes_per_pixel: 0,
                stride: 0,
            },
            x_pos: 0,
            y_pos: 0,
        }
    }

    pub fn init(&mut self, buffer: &'static mut [u8], info: FrameBufferInfo) {
        self.framebuffer = Optional::Some(buffer);
        self.info = info;
        self.clear();
    }

    fn write_framebuffer(&mut self, pos: usize, value: u8) {
        self.framebuffer.as_mut().unwrap()[pos] = value
    }

    fn write_pixel(&mut self, x: usize, y: usize, color: &PixelColor) -> bool {
        let pixel_position = x + self.info.stride * y;
        let byte_position = pixel_position * self.info.bytes_per_pixel;
        match self.info.pixel_format {
            PixelFormat::Rgb => {
                self.write_framebuffer(byte_position, color.r);
                self.write_framebuffer(byte_position + 1, color.g);
                self.write_framebuffer(byte_position + 2, color.b);
                true
            }
            PixelFormat::Bgr => {
                self.write_framebuffer(byte_position, color.b);
                self.write_framebuffer(byte_position + 1, color.g);
                self.write_framebuffer(byte_position + 2, color.r);
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
