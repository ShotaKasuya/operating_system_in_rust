use alloc::fmt;
use core::fmt::Write;
use noto_sans_mono_bitmap::{get_raster, RasterizedChar};
use crate::frame_buffer_writer::{FRAME_BUFFER_WRITER, FrameBufferWriter};
use crate::frame_buffer_writer::pixel_color::PixelColor;
use crate::frame_buffer_writer::text_writer::font_constants::BACKUP_CHAR;

// 行と行の間
const LINE_SPACING: usize = 2;
// 文字と文字の間
const LETTER_SPACING: usize = 0;
// 画面端からの距離
const BORDER_PADDING: usize = 1;

mod font_constants {
    use noto_sans_mono_bitmap::{FontWeight, get_raster_width, RasterHeight};

    pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;
    pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);
    pub const BACKUP_CHAR: char = '�';
    pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;    // 文字の太さ: 普通
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {($crate::frame_buffer_writer::text_writer::_print(format_args!($($arg)*)))};
}
#[macro_export]
macro_rules! println {
    () => {$crate::print!("\n")};
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    FRAME_BUFFER_WRITER.lock().write_fmt(args).unwrap();
}

fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(
            c,
            font_constants::FONT_WEIGHT,
            font_constants::CHAR_RASTER_HEIGHT,
        )
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
}

impl FrameBufferWriter {
    fn new_line(&mut self) {
        self.y_pos += font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.carriage_return();
    }

    fn carriage_return(&mut self) {
        self.x_pos = BORDER_PADDING;
    }

    pub fn clear(&mut self) {
        self.x_pos = BORDER_PADDING;
        self.y_pos = BORDER_PADDING;
        self.framebuffer.as_mut().unwrap().fill(0);
    }

    fn width(&self) -> usize {
        self.info.width
    }

    fn height(&self) -> usize {
        self.info.height
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.new_line(),
            '\r' => self.carriage_return(),
            c => {
                let new_x_pos = self.x_pos + font_constants::CHAR_RASTER_WIDTH;
                if new_x_pos >= self.width() {
                    self.new_line();
                }
                let new_y_pos =
                    self.y_pos + font_constants::CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
                if new_y_pos >= self.height() {
                    self.clear();
                }
                self.write_rendered_char(get_char_raster(c));
            }
        }
    }

    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x_pos + x, self.y_pos + y, &PixelColor::byte_to_color(*byte));
            }
        }
        self.x_pos += rendered_char.width() + LETTER_SPACING;
    }
}

unsafe impl Send for FrameBufferWriter {}

unsafe impl Sync for FrameBufferWriter {}

impl fmt::Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for char in s.chars() {
            self.write_char(char);
        }
        Ok(())
    }
}